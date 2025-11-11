use crate::application::draw::talu::TaluBankGridDefns;
use crate::application::draw::cpu_register::CpuRegisterBankGridDefns;
use crate::application::draw::instruction_memory::InstructionMemoryGridDefns;
use crate::application::grid::blocked_point::BlockedPoints;
use crate::application::simulation::talu::TaluBank;
use crate::application::simulation::cpu_registers::CpuRegisterBank;

pub struct CpuGridDefns{
    pub talu_bank            : TaluBankGridDefns,
    pub register_bank        : CpuRegisterBankGridDefns,
    pub instruction_memory   : InstructionMemoryGridDefns,
    pub blocked_points       : BlockedPoints,
}





