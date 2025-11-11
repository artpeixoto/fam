use wgpu::naga::FastHashMap;
use crate::application::{draw::port::{PortDefns, PortGridDefns}, grid::{component::SimpleComponentGridDefns, cpu_register::CpuRegisterPortsGridData}, simulation::{controller::ControllerPortName, cpu_registers::CpuRegisterPortsData}};


pub type ControllerGridDefns =
    SimpleComponentGridDefns<
        ControllerPortName,
        ControllerPortsData,
		ControllerPortsGridData,
    >;

pub type ControllerPortsGridData 	= FastHashMap<ControllerPortName, PortGridDefns>;
pub type ControllerPortsData 		= FastHashMap<ControllerPortName, PortDefns>;