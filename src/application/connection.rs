use std::cmp::{max, min};
use crate::application::{draw::{talu::TaluBankGridDefns, port::PortGridDefns}, grid::component::{ComponentCalculatedDefns, PortDataContainer}, simulation::{talu::{TaluAddress, TaluBank, TaluPortName}, controller::ControllerPortName, cpu_registers::{CpuRegisterAddress, CpuRegisterPortName}, simulation::Cpu}};


#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
pub enum CpuConnectionEndpoint{
    Register(CpuRegisterAddress, CpuRegisterPortName),
    Talu(TaluAddress, TaluPortName),
    Controller(ControllerPortName),
    MainMemory,
}


#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, getset::Getters)]
pub struct CpuConnection{ 
    #[getset(get="pub")]
    first: CpuConnectionEndpoint,
    #[getset(get="pub")]
    second: CpuConnectionEndpoint,
}


impl CpuConnection{
    pub fn new(first_endpoint: CpuConnectionEndpoint, second_endpoint: CpuConnectionEndpoint)-> Self{
        let first = min(first_endpoint.clone(), second_endpoint.clone());
        let second = max(first_endpoint.clone(), second_endpoint.clone());
        return Self { first, second };
    }
}
    
impl TaluBankGridDefns{
    pub fn get_port_grid_data(&self, talu_addr: TaluAddress, port_name: TaluPortName) -> &PortGridDefns{
        self.comp_grid_datas[talu_addr].ports_grid_data.get_for_port( &port_name )
    }
}

