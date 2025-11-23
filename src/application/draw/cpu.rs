use crate::application::draw::controller::ControllerDrawingDefns;
use crate::application::draw::talu::TaluBankDrawingDefns;
use crate::application::draw::cpu_register::{CpuRegisterBankDrawingDefns, CpuRegisterDrawingDefn};
use crate::application::draw::grid_to_screen::GridScreenTransformer;
use crate::application::draw::instruction_memory::InstructionMemoryDrawingDefns;
use crate::application::draw::port::PortDrawingDefns;

pub struct CpuDrawingData{
    pub port                    : PortDrawingDefns,
    pub register_bank           : CpuRegisterBankDrawingDefns,
    pub talu_bank               : TaluBankDrawingDefns,
    pub instruction_memory      : InstructionMemoryDrawingDefns,
    pub controller              : ControllerDrawingDefns,
}