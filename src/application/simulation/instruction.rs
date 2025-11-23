use crate::application::simulation::cpu_registers::CpuRegisterAddress;
use crate::application::simulation::talu::{TaluAddress, TaluOperation};
use crate::word::Word;

pub const CONTROLLER_INSTRUCTION_SIZE   		: usize = 64;


#[derive( PartialEq, Copy, Clone, Debug,Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum Instruction {
    
    SetTaluConfig{
        talu_addr	: TaluAddress,
        talu_config	: TaluOperation, 
    },
    ResetAllTalus,
    
    SetLiteral{
        literal	: Word,
        reg_addr: CpuRegisterAddress,
    },

    // PopStack{
    //     register_index	: CpuRegistersAddress,
    // },
    //
    // PushToStack{
    //     register_index	: CpuRegistersAddress,
    // },

    WaitForActivationSignal{
        register_index  : CpuRegisterAddress
    },

    Jump{
        // relative        : bool,
        addr            : Word
    },
    #[default]
    NoOp,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum HorizontalDir{
    Left,
    Right
}




