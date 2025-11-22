use std::ops::Index;
use getset::Getters;
use itertools::Itertools;
use crate::word::{Activation, Word};
use crate::application::draw::port::{PortDefns, PortSignalDirection, SignalType};
use crate::application::grid::component::{PortDataContainer, PortName};
use crate::application::simulation::component_bank::ComponentBank;
use crate::application::simulation::cpu_registers::CpuRegisterDataReader::{Active, Deactivated};
use crate::tools::used_in::UsedIn;
use crate::word::{ToActivation, ToWord};

pub type CpuRegisterAddress = usize;
pub const REGISTER_COUNT: CpuRegisterAddress = 64;
pub type CpuRegisterBank = ComponentBank<CpuRegister, REGISTER_COUNT>;

impl CpuRegisterBank {
    pub fn new() -> Self{
        let registers = (0..REGISTER_COUNT).into_iter().map(|address|CpuRegister::new(address))
            .collect_array().unwrap().pipe(Box::new);
        CpuRegisterBank {
           components: registers
        }
    }
}

pub struct CpuRegister{
    pub address : CpuRegisterAddress,
    pub value   : Word,
}

impl PortDataContainer<CpuRegisterPortName, PortDefns> for CpuRegisterPortsData{
    fn get_for_port(&self, port_name: &CpuRegisterPortName) -> &PortDefns {
        match port_name {
            CpuRegisterPortName::Input => &self.input,
            CpuRegisterPortName::Output => &self.output,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CpuRegisterPortsData {
    pub input: PortDefns,
    pub output: PortDefns,
}

impl CpuRegister {
    pub fn new(address: CpuRegisterAddress) -> Self{
        CpuRegister{
            address,
            value   : 0
        }
    }
    pub fn ports_info(&self) -> CpuRegisterPortsData {
        CpuRegisterPortsData {
            input: PortDefns {
                active: true,
                signal_dir: PortSignalDirection::Input,
                signal_type: SignalType::Data,
            },
            output: PortDefns {
                active: true,
                signal_dir: PortSignalDirection::Output,
                signal_type: SignalType::Data,
            },
        }
    }
    pub fn write(&mut self, new_value: Word) {
        self.value = new_value;
    }
    pub fn read(&self) -> Word {
        self.value
    }
}

impl Index<CpuRegisterPortName> for CpuRegisterPortsData {
    type Output = PortDefns;

    fn index(&self, index: CpuRegisterPortName) -> &Self::Output {
        match index{
            CpuRegisterPortName::Input  => {&self.input}
            CpuRegisterPortName::Output => {&self.output}
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
pub enum CpuRegisterPortName{
    Input,
    Output,
}
impl PortName for CpuRegisterPortName{
    fn all_port_names() -> Vec<Self> {
        vec![
            Self::Input,
            Self::Output,
        ]
    }

    fn small_name(&self) -> &str {
        match self{
            CpuRegisterPortName::Input => "in",
            CpuRegisterPortName::Output => "out"
        }
    }
}


impl CpuRegisterPortName{
    pub fn iter_ports()  -> impl Iterator<Item=CpuRegisterPortName>{
        [
            CpuRegisterPortName::Input,
            CpuRegisterPortName::Output,
        ]
        .into_iter()
    }
}



pub enum CpuRegisterDataReader {
    Deactivated,
    Active {
        source: CpuRegisterAddress,
        value : Option<Word>,
    }
}

impl CpuRegisterDataReader {
    pub fn new() -> Self{
        Deactivated
    }
    pub fn deactivate(&mut self) {
        *self = Deactivated;
    }
    pub fn is_active(&self) -> bool{
        matches!(self, Active {..})
    }
    pub fn set_connection(&mut self, target: Option<CpuRegisterAddress>){
        if let Some(target) = target {
            *self = Active {
                source: target,
                value: None,
            };
        } else {
            *self = Deactivated;
        }
    }

    pub fn read(&self) -> Option<Word> {
        if let Active{ source:_, value} = self
        && let Some(val) = value
        {
            Some(*val)
        }  else {
            None
        }
    }
    pub fn get_read_request(&mut self) -> Option<CpuRegisterReadRequest<'_>> {
        if let Active{ source, value} = self{
            Some(CpuRegisterReadRequest{
                register_addr: source,
                value_cell: value,
            })
        } else {
            None
        }
    }
}

pub enum CpuRegisterDataWriter{
    Deactivated,
    Connected{
        target: CpuRegisterAddress,
        value: Option<Word>
    }
}

impl CpuRegisterDataWriter{
    pub fn new() -> Self{
        Self::Deactivated 
    }
    pub fn deactivate(&mut self, ){
        *self = CpuRegisterDataWriter::Deactivated;
    }
    pub fn is_active(&self) -> bool{
        matches!(self,  CpuRegisterDataWriter::Connected{..})
    }
    pub fn  set_connection(&mut self, target: Option<CpuRegisterAddress>){
        if let Some(target) = target {
            *self = Self::Connected {
                target,
                value: None,
            };
        } else {
            *self = Self::Deactivated;
        }

    }
    pub fn write(&mut self, value: Word) {
        if let CpuRegisterDataWriter::Connected{
            target,
            value: inner_value
        } = self{
            *inner_value = Some(value)
        } else {
        }
    }
    pub fn clear(&mut self){
        if let CpuRegisterDataWriter::Connected { target, value } = self{
            *value = None;
        }
    }

    pub fn get_write_request(&self) -> Option<CpuRegisterWriteRequest>{
        if let CpuRegisterDataWriter::Connected {
            target,
            value: inner_value,
        } = self
        && let Some(val) = inner_value
        {
            Some(CpuRegisterWriteRequest{
                register_addr: target,
                value : val,
            })
        } else {
            None
        }
    }
}

pub struct CpuRegisterWriteRequest<'a>{
    register_addr: &'a CpuRegisterAddress,
    value        : &'a Word,
}

impl<'a> CpuRegisterWriteRequest<'a> {
    pub fn satisfy(self, register_bank: &mut CpuRegisterBank) {
         register_bank.components[*self.register_addr].write(*self.value);
    }

    pub fn addr(&self) -> &CpuRegisterAddress{
        self.register_addr
    }
}
pub struct CpuRegisterReadRequest<'a>{
    register_addr : &'a CpuRegisterAddress,
    value_cell    : &'a mut Option<Word>
}

impl CpuRegisterReadRequest<'_>{
    pub fn addr(&self) -> &CpuRegisterAddress{
        self.register_addr
    }
    pub fn satisfy(self, register_bank: &CpuRegisterBank)  {
        *self.value_cell = Some( register_bank.components[*self.register_addr].value)
    }
}

pub struct CpuRegisterActReader{
    inner     : CpuRegisterDataReader,
}

impl CpuRegisterActReader{

    pub fn new() -> Self{
        Self{inner: CpuRegisterDataReader::new()} 
    }
    pub fn deactivate(&mut self, ){
        self.inner.deactivate();
    }
    pub fn is_active(&self) -> bool{
        self.inner.is_active()
    }
    pub fn  set_connection(&mut self, target: Option<CpuRegisterAddress>){
        self.inner.set_connection(target);
    }

    pub fn get_read_request<'a>(&'a mut self) ->  Option<CpuRegisterReadRequest<'a>>{
       self.inner.get_read_request() 
    }
    pub fn read(&self) -> Option<Activation>{
        self.inner.read().map(|val| val.to_activation())
    }
}



pub struct CpuRegisterActWriter {
    inner       : CpuRegisterDataWriter,
}

impl CpuRegisterActWriter {
    pub fn new() -> Self{
        Self {
            inner   : CpuRegisterDataWriter::Deactivated
        }
    }
    #[inline]
    pub fn set_connection(&mut self, target: Option<CpuRegisterAddress>){
        self.inner.set_connection(target);
    }
    #[inline]
    pub fn get_write_request(&self) -> Option<CpuRegisterWriteRequest>{
        self.inner.get_write_request()
    }
    #[inline]
    pub fn deactivate(&mut self){
        self.inner.deactivate();
    }
    #[inline]
    pub fn is_active(&self) -> bool{
        self.inner.is_active()
    }
    #[inline]
    pub fn clear(&mut self) {
        self.inner.clear();
    }
    #[inline]
    pub fn write(&mut self,  value: bool) {
        self.inner.write(value.to_word())
    }
    
}

