use itertools::Itertools;
use wgpu::naga::{FastHashMap, FastHashSet};

use crate::application::connection::{CpuConnection, CpuConnectionEndpoint};
use crate::application::draw::port::PortGridDefns;
use crate::application::draw::talu::TaluBankGridDefns;
use crate::application::draw::cpu_register::CpuRegisterBankGridDefns;
use crate::application::draw::instruction_memory::InstructionMemoryGridDefns;
use crate::application::grid::blocked_point::BlockedPoints;
use crate::application::grid::component::{ComponentGridData, PortDataContainer};
use crate::application::grid::connection::{ConnectionEndpoint, ConnectionEndpointPair};
use crate::application::grid::controller::ControllerGridDefns;
use crate::application::grid::grid_limits::GridLimits;
use crate::application::grid::path::{Path, find_path_a_star};
use crate::application::simulation::controller::Controller;
use crate::application::simulation::simulation::Netlists;
use crate::application::simulation::talu::TaluBank;
use crate::application::simulation::cpu_registers::CpuRegisterBank;

pub struct CpuGridData{
    pub talu_bank            : TaluBankGridDefns,
    pub register_bank        : CpuRegisterBankGridDefns,
    pub controller           : ControllerGridDefns,
    pub instruction_memory   : InstructionMemoryGridDefns,
    pub blocked_points       : BlockedPoints,
    pub paths                : FastHashMap<CpuConnection, Path> 

}

impl CpuGridData {
    pub fn get_port_grid_data<'a>
        (&'a self, endpoint: &CpuConnectionEndpoint) -> &'a PortGridDefns
    {
        match endpoint{
            CpuConnectionEndpoint::Register(reg_addr, reg_port_name) => {
                self.register_bank.comp_grid_datas[*reg_addr].ports_grid_data.get_for_port(reg_port_name)
            },
            CpuConnectionEndpoint::Talu(talu_addr, talu_port_name) => {
                self.talu_bank.comp_grid_datas[*talu_addr].ports_grid_data.get_for_port(talu_port_name)
            },
            CpuConnectionEndpoint::Controller(controller_port_name) => {
                self.controller.ports_grid_data.get_for_port(controller_port_name)
            },
            CpuConnectionEndpoint::MainMemory => {
                todo!();
            },
        }
    }
    pub fn update_blocked_points(&mut self){
        let all_blocked_points = {
            let mut blocked = self.talu_bank.blocked_points.clone();
            blocked.add_from(self.register_bank.blocked_points());
            blocked.add_from(self.instruction_memory.blocked_points());
            blocked.add_from(self.controller.blocked_points());
            blocked
        } ;
        self.blocked_points = all_blocked_points;
    }

    pub fn calculate_paths(&mut self,  connections: &FastHashSet<CpuConnection>,netlists: &Netlists, grid_limits: &GridLimits){
        self.paths.clear();
        // self.paths.retain(|k, _v| connections.contains(k));
        let missing_conns = connections.iter().filter(|conn| !self.paths.contains_key(*conn)).cloned().collect_vec();
        for conn in missing_conns{
            let start_pos = self.get_port_grid_data(conn.first()).position;
            let end_pos = self.get_port_grid_data(conn.second()).position;
            let res = 
                find_path_a_star(
                    &start_pos, &end_pos, 
                    &conn,
                    &self.paths,
                    netlists,
                    &self.blocked_points,
                    &grid_limits
                )
                .unwrap();
            
            self.paths.insert(conn.clone(), res);
        }
    }
}





