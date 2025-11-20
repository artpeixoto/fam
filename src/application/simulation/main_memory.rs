use std::ops::Deref;
use std::sync::{Arc, RwLock};
use crate::application::simulation::memory_primitives::register::Register;
use crate::{ Step};
use crate::word::{Word};
pub const MAIN_MEMORY_LEN: usize = 1024;

type MainMemoryInner = Arc<RwLock<Vec<Word>>>;
pub struct MainMemory(pub MainMemoryInner);

impl MainMemory{
    pub fn new(content: Vec<Word>) -> Self{
        MainMemory(Arc::new(RwLock::new(content)))
    }

}

pub struct MainMemoryIo(MainMemoryInner);

impl MainMemory{
    pub fn get_io(&self) -> MainMemoryIo {
        MainMemoryIo(self.0.clone())
    }
}

impl MainMemoryIo {
    pub fn read(& self, addr: usize) -> Word{
        self.0.read().unwrap().get(addr).unwrap().clone()
    }
    pub fn write(&self, addr: usize, value: Word) {
        *self.0.write().unwrap().get_mut(addr).unwrap() = value;
    }
}
