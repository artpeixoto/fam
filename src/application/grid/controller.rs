use wgpu::naga::FastHashMap;
use crate::application::{draw::port::{PortDefns, PortGridDefns}, grid::{component::SimpleComponentGridData, cpu_register::CpuRegisterPortsGridData}, simulation::{controller::ControllerPortName, cpu_registers::CpuRegisterPortsData}};


pub type ControllerGridDefns =
    SimpleComponentGridData<
        ControllerPortName,
        ControllerPortsData,
		ControllerPortsGridData,
    >;

pub type ControllerPortsGridData 	= FastHashMap<ControllerPortName, PortGridDefns>;
pub type ControllerPortsData 		= FastHashMap<ControllerPortName, PortDefns>;