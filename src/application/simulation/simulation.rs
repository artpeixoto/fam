use std::collections::HashSet;
use std::{array, iter};
use std::ops::Not;
use getset::Getters;
use itertools::Itertools;
use wgpu::naga::{FastHashMap, FastHashSet};
use crate::application::connection::{CpuConnection, CpuConnectionEndpoint};
use crate::application::grid::connection::ConnectionEndpoint;
use crate::application::simulation::talu::{TALU_COUNT, TaluAddress, TaluBank, TaluCore, TaluOperation, TaluPortName};
use crate::application::simulation::controller::{self, Controller, ControllerPortName, TaluConfigWriter};
use crate::application::simulation::cpu_registers::{CpuRegisterAddress, CpuRegisterBank, CpuRegisterPortName, REGISTER_COUNT};
use crate::application::simulation::instruction::Instruction;
use crate::application::simulation::instruction_reader::{InstructionMemory, InstructionReader};
use crate::application::simulation::main_memory::MainMemory;
use crate::{PROGRAM_COUNTER_REGISTER_ADDR, Step};
use crate::word::Word;

pub type NetlistId = u16;

#[derive(Clone, Default, Debug)]
pub struct Netlists { 
    next_netlist_id: NetlistId,
    netlists: FastHashMap<CpuConnectionEndpoint, NetlistId>  
}

impl Netlists{
    
   pub fn new() -> Self{
        Netlists{ next_netlist_id: 0, netlists: Default::default() }
    }
    pub fn get_for_endpoint(&self, endpoint: &CpuConnectionEndpoint) -> Option<NetlistId>{
        self.netlists.get(endpoint).cloned()
    }
    pub fn get_for_connection(&self, conn: &CpuConnection) -> Option<NetlistId>{
        self.netlists.get(conn.first()).cloned()
    }
    pub fn clear(&mut self){
        self.netlists.clear();
        self.next_netlist_id = 0;
    }
     pub fn add(&mut self, conn: &CpuConnection){
        if let Some(&netlist) = self.netlists.get(conn.first()){
            self.netlists.insert(conn.second().clone(), netlist); 
        } else if let Some(&netlist) = self.netlists.get(conn.second()){
            self.netlists.insert(conn.first().clone(), netlist);
        } else {
            self.netlists.insert(conn.first().clone(), self.next_netlist_id);
            self.netlists.insert(conn.second().clone(), self.next_netlist_id);
            self.next_netlist_id +=1;
        }
    }
}

#[derive(Getters)]
pub struct Cpu {
    pub talu_bank           : TaluBank,
    pub register_bank       : CpuRegisterBank,
    pub controller          : Controller,
    pub instruction_memory  : InstructionMemory,
    pub main_memory         : MainMemory,
    pub is_done             : bool,
    // #[getset(get="pub")]
    pub connections             : FastHashSet<CpuConnection>,
    pub netlists                : Netlists
}

impl Cpu {
    #[must_use]
    pub fn execute(&mut self) -> bool {
        if self.is_done { return false; }

        self.connections.clear();

        if let Some(mut controller_read_req) =
            self.controller.cpu_registers_reader.get_read_request() {
            self.connections.insert(CpuConnection::new(
                CpuConnectionEndpoint::Controller(ControllerPortName::RegisterReader), 
                CpuConnectionEndpoint::Register(
                    *controller_read_req.addr(), 
                    CpuRegisterPortName::Output
                )
            ));

            controller_read_req.satisfy( &self.register_bank);
        }


        if let Some(mut controller_pc_read_req) =
            self.controller
            .instruction_reader
            .program_counter_reader
            .get_read_request()
        {
            self.connections.insert(CpuConnection::new(
                CpuConnectionEndpoint::Controller(
                    ControllerPortName::ProgramCounterReader
                ), 
                CpuConnectionEndpoint::Register(
                    PROGRAM_COUNTER_REGISTER_ADDR, 
                    CpuRegisterPortName::Output
                )
           ));

            controller_pc_read_req.satisfy(&self.register_bank);
        }

        if let Some(config_write_request) = 
            self.controller
            .talu_config_writer
            .get_config_write_request()
        {
            if let Some(addr) = config_write_request.address(){
                self.connections.insert(CpuConnection::new(
                    CpuConnectionEndpoint::Controller(
                        ControllerPortName::TaluConfigWriter
                    ),
                    CpuConnectionEndpoint::Talu(*addr, TaluPortName::SetupIn)
                ));
            } else {
                for talu_addr in 0..TALU_COUNT{
                    self.connections.insert(CpuConnection::new(
                        CpuConnectionEndpoint::Controller(
                            ControllerPortName::TaluConfigWriter
                        ),
                        CpuConnectionEndpoint::Talu(talu_addr, TaluPortName::SetupIn)
                    ));
                }
            }
            config_write_request.satisfy(&mut self.talu_bank);
        } 


        // give talus the requested data
        for ( talu_addr, talu ) in self.talu_bank.components.iter_mut().enumerate(){
            let mut reqs = talu.collect_read_requests();
            for (port, req) in reqs{
                self.connections.insert( CpuConnection::new(
                    CpuConnectionEndpoint::Talu(talu_addr, port),
                    CpuConnectionEndpoint::Register(*req.addr(), CpuRegisterPortName::Output)
                ));
                req.satisfy(&self.register_bank);
            }
            // talu_reads.push(reqs);
        }

        if self.controller.execute().not(){
            self.is_done = true;
        };

        for talu in self.talu_bank.components.iter_mut(){
            talu.execute();
        }

        for ( talu_addr, talu ) in self.talu_bank.components.iter_mut().enumerate(){
            let reqs = talu.collect_write_requests();
            for (talu_port, req) in reqs{
                self.connections.insert( CpuConnection::new(
                    CpuConnectionEndpoint::Talu(talu_addr, talu_port),
                    CpuConnectionEndpoint::Register(*req.addr(), CpuRegisterPortName::Input)
                ));
                req.satisfy(&mut self.register_bank);
            }
            // talu_reads.push(reqs);
        }

        if let Some(req) = self.controller.cpu_registers_writer.get_write_request(){
            self.connections.insert( CpuConnection::new(
                CpuConnectionEndpoint::Controller(ControllerPortName::RegisterWriter),
                CpuConnectionEndpoint::Register(*req.addr(), CpuRegisterPortName::Input)
            ));

            req.satisfy(&mut self.register_bank);
        }

        if let Some(write_pc_req) = self.controller.instruction_reader.program_counter_writer.get_write_request(){
            self.connections.insert( CpuConnection::new(
                CpuConnectionEndpoint::Controller(
                    ControllerPortName::ProgramCounterWriter
                ),
                CpuConnectionEndpoint::Register(
                    PROGRAM_COUNTER_REGISTER_ADDR, 
                    CpuRegisterPortName::Input
                )
            ));

            write_pc_req.satisfy(&mut self.register_bank);
        }

        self.netlists.clear();
        for conn in self.connections.iter(){
            self.netlists.add(conn)
        }

        return !self.is_done ;
    }
}


