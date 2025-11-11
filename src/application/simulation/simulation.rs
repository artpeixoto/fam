use std::collections::HashSet;
use std::{array, iter};
use std::ops::Not;
use getset::Getters;
use itertools::Itertools;
use wgpu::naga::FastHashSet;
use crate::application::connection::{CpuConnection, CpuConnectionEndpoint};
use crate::application::simulation::talu::{TALU_COUNT, TaluAddress, TaluBank, TaluCore, TaluOperation, TaluPortName};
use crate::application::simulation::controller::{self, Controller, ControllerPortName, TaluConfigWriter};
use crate::application::simulation::cpu_registers::{CpuRegisterAddress, CpuRegisterBank, CpuRegisterPortName, REGISTER_COUNT};
use crate::application::simulation::instruction::Instruction;
use crate::application::simulation::instruction_reader::{InstructionMemory, InstructionReader};
use crate::application::simulation::main_memory::MainMemory;
use crate::{PROGRAM_COUNTER_REGISTER_ADDR, Step};
use crate::word::Word;


#[derive(Getters)]
pub struct Cpu {
    pub talu_bank            : TaluBank,
    pub register_bank       : CpuRegisterBank,
    pub controller          : Controller,
    pub instruction_memory  : InstructionMemory,
    pub main_memory         : MainMemory,

    // #[getset(get="pub")]
    pub connections             : FastHashSet<CpuConnection>
}

impl Cpu {
    #[must_use]
    pub fn execute(&mut self) -> bool {
        self.connections.clear();
        if let Some(mut controller_read_req) =
            self.controller.cpu_registers_reader.get_read_request() {
            self.connections.insert(CpuConnection::new(
                CpuConnectionEndpoint::Controller(ControllerPortName::RegisterReader), 
                CpuConnectionEndpoint::Register(
                    controller_read_req.addr(), 
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
                CpuConnectionEndpoint::Controller(ControllerPortName::MainMemoryReader), 
                CpuConnectionEndpoint::Register(
                    PROGRAM_COUNTER_REGISTER_ADDR, CpuRegisterPortName::Output
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
            for (port, req) in &mut reqs{
                self.connections.insert( CpuConnection::new(
                    CpuConnectionEndpoint::Talu(talu_addr, *port),
                    CpuConnectionEndpoint::Register(req.addr(), CpuRegisterPortName::Output)
                ));
                req.satisfy(&self.register_bank);
            }
            // talu_reads.push(reqs);
        }

        if self.controller.execute().not(){
            return false;
        };


        for talu in self.talu_bank.components.iter_mut(){
            talu.execute();
        }

        for ( talu_addr, talu ) in self.talu_bank.components.iter_mut().enumerate(){
            let mut reqs = talu.collect_write_requests();
            for (talu_port, req) in &mut reqs{
                self.connections.insert( CpuConnection::new(
                    CpuConnectionEndpoint::Talu(talu_addr, *talu_port),
                    CpuConnectionEndpoint::Register(req.addr(), CpuRegisterPortName::Input)
                ));
                req.satisfy(&mut self.register_bank);
            }
            // talu_reads.push(reqs);
        }

        if let Some(req) = self.controller.cpu_registers_writer.get_write_request(){
            self.connections.insert( CpuConnection::new(
                CpuConnectionEndpoint::Controller(ControllerPortName::RegisterWriter),
                CpuConnectionEndpoint::Register(req.addr(), CpuRegisterPortName::Output)
            ));

            req.satisfy(&mut self.register_bank);
        }

        if let Some(write_pc_req) = self.controller.instruction_reader.program_counter_writer.get_write_request(){

            self.connections.insert( CpuConnection::new(
                CpuConnectionEndpoint::Controller(ControllerPortName::RegisterWriter),
                CpuConnectionEndpoint::Register(
                    PROGRAM_COUNTER_REGISTER_ADDR, 
                    CpuRegisterPortName::Input
                )
            ));

            write_pc_req.satisfy(&mut self.register_bank);
        }

        true
    }
}


