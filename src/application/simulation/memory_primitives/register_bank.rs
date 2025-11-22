use std::ops::{Deref};
use itertools::Itertools;
use crate::{Step};
use crate::application::simulation::cpu_registers::CpuRegisterAddress;
use crate::word::Word;
use super::register::{Register,};

pub struct RegisterBank<Data, const COUNT: usize> {
    pub registers       : Box<[Register<Data>; COUNT]>,
}


impl<Data, const COUNT: usize> RegisterBank<Data, { COUNT }> {
    pub fn new(initial_vals: impl Iterator<Item=Data>) -> Self {
        let registers  =
            Box::new(
                initial_vals
                .map(|data| Register::new(data))
                .collect_array()
                .unwrap()
            );

        Self { registers }
    }
}