use std::ops::Index;
use crate::application::direction::Direction;
use crate::application::draw::port::{PortDefns, PortGridDefns, PortSignalDirection};
use crate::application::grid::blocked_point::BlockedPoints;
use crate::application::grid::component::PortDataContainer;
use crate::application::grid::pos::{GridPos, GridDist, grid_pos, GridSize, grid_size};
use crate::application::simulation::cpu_registers::{CpuRegisterPortName, CpuRegisterPortsData};



#[derive(Clone, PartialEq, Debug, Eq, )]
pub struct CpuRegisterPortsGridData {
    pub input : PortGridDefns,
    pub output: PortGridDefns,
}

impl PortDataContainer<CpuRegisterPortName, PortGridDefns> for CpuRegisterPortsGridData {
    fn get(&self, port_name: &CpuRegisterPortName) -> &PortGridDefns {
        match port_name{
            CpuRegisterPortName::Input => {&self.input}
            CpuRegisterPortName::Output => {&self.output}
        }
    }
}

impl Index<CpuRegisterPortName> for CpuRegisterPortsGridData {
    type Output = PortGridDefns; 

    fn index(&self, index: CpuRegisterPortName) -> &Self::Output {
        match index{
            CpuRegisterPortName::Input => {&self.input}
            CpuRegisterPortName::Output => {&self.output}
        }
    }
}