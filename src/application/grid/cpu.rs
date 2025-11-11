use crate::application::connection::CpuConnectionEndpoint;
use crate::application::draw::port::PortGridDefns;
use crate::application::draw::talu::TaluBankGridDefns;
use crate::application::draw::cpu_register::CpuRegisterBankGridDefns;
use crate::application::draw::instruction_memory::InstructionMemoryGridDefns;
use crate::application::grid::blocked_point::BlockedPoints;
use crate::application::grid::component::PortDataContainer;
use crate::application::grid::connection::{ConnectionEndpoint, ConnectionEndpointPair};
use crate::application::grid::controller::ControllerGridDefns;
use crate::application::simulation::controller::Controller;
use crate::application::simulation::talu::TaluBank;
use crate::application::simulation::cpu_registers::CpuRegisterBank;

pub struct CpuGridDefns{
    pub talu_bank            : TaluBankGridDefns,
    pub register_bank        : CpuRegisterBankGridDefns,
    pub controller           : ControllerGridDefns,
    pub instruction_memory   : InstructionMemoryGridDefns,
    pub blocked_points       : BlockedPoints,
}
impl CpuGridDefns {
    pub fn get_port_defns<'a>
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
}





