pub mod core;
pub mod op;

use std::array;
pub use core::*;
pub use op::*;
use crate::{Step };
use crate::application::simulation::component_bank::ComponentBank;
use crate::application::simulation::cpu_registers::CpuRegisterBank;
use crate::application::simulation::main_memory::MainMemory;

pub type TaluBank = ComponentBank<TaluCore, TALU_COUNT>;
pub type TaluAddress = usize;
pub const TALU_COUNT: usize = 32;
impl TaluBank {
    pub fn new(
        main_memory: &mut MainMemory,
    ) -> Self{
        Self{
            components: Box::new(array::from_fn(|i|
                TaluCore::new(
                    i,
                    main_memory,
                )
            ))
        }
    }
}






