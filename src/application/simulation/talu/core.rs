use std::collections::HashMap;
use super::TaluOperation;
use crate::application::draw::port::SignalType::Activation;
use crate::application::draw::port::{PortDefns, PortSignalDirection, SignalType};
use crate::application::grid::component::{PortDataContainer, PortName};
use crate::application::simulation::talu::CmpOp;
use crate::application::simulation::talu::TaluPortName::{
    ActivationIn, ActivationOut, DataIn0, DataIn1, DataOut0, DataOut1, SetupIn
};
use crate::application::simulation::main_memory::{MainMemory, MainMemoryIo};
use crate::application::simulation::memory_primitives::register::Register;
use crate::word::{ToBool, ToWord, Word};
use std::mem::transmute;
use std::ops::Index;
use PortSignalDirection::{Input, Output};
use SignalType::Data;
use crate::application::simulation::cpu_registers::{CpuRegisterActReader, CpuRegisterActWriter, CpuRegisterDataReader, CpuRegisterDataWriter, CpuRegisterReadRequest, CpuRegisterWriteRequest};


#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
pub enum TaluPortName {
    DataIn0,
    DataIn1,
    ActivationIn,

    DataOut0,
    DataOut1,
    ActivationOut,
    SetupIn,
}

impl PortName for TaluPortName {
    fn all_port_names() -> Vec<Self> {
        vec![
            DataIn0,
            DataIn1,
            ActivationIn,
            DataOut0,
            DataOut1,
            ActivationOut,
            SetupIn,
        ]
    }

    fn small_name(&self) -> &str {
        match self {
            DataIn0 => "di0",
            DataIn1 => "di1",
            ActivationIn => "ai",
            DataOut0 => "do0",
            DataOut1 => "do1",
            ActivationOut => "ao",
            SetupIn => "cfg",
        }
    }
}

pub struct TaluPortsDefns {
    // pub state_in            : PortInfo,
    pub data_input_0    : PortDefns,
    pub data_input_1    : PortDefns,
    pub activation_input: PortDefns,

    pub data_output_0   : PortDefns,
    pub data_output_1   : PortDefns,
    pub activation_output: PortDefns,
    pub setup_in: PortDefns,
}
impl PortDataContainer<TaluPortName, PortDefns> for TaluPortsDefns {
    fn get_for_port(&self, port_name: &TaluPortName) -> &PortDefns {
        match port_name {
            ActivationIn => &self.activation_input,
            DataIn0 => &self.data_input_0,
            DataIn1 => &self.data_input_1,
            DataOut0 => &self.data_output_0,
            DataOut1 => &self.data_output_1,
            ActivationOut => &self.activation_output,
            SetupIn => &self.setup_in
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum TaluState{
    Closing,
    JustProcessed,
    Done
}


pub struct TaluCore {
    pub addr            : usize,
    pub state           : TaluState,
    pub operation       : TaluOperation,
    pub old_operation   : TaluOperation,
    pub main_memory     : MainMemoryIo,

    pub inner_memory_0  : Word,
    pub inner_memory_1  : Word,

    pub data_input_0    : CpuRegisterDataReader,
    pub data_input_1    : CpuRegisterDataReader,
    pub activation_input: CpuRegisterActReader,

    pub data_output_0   : CpuRegisterDataWriter,
    pub data_output_1   : CpuRegisterDataWriter,
    pub activation_output: CpuRegisterActWriter,
}


// pub struct TaluConnectionRequests<'a>{
//     pub read:  HashMap<TaluPortName, CpuRegisterReadRequest<'a>> ,
//     pub write: HashMap<TaluPortName, >, 
// }

impl TaluCore {
    pub fn collect_read_requests<'a>(&'a mut self) -> HashMap<TaluPortName, CpuRegisterReadRequest<'a>> {
        [
            (DataIn0, self.data_input_0.get_read_request()),
            (DataIn1, self.data_input_1.get_read_request()),
            (ActivationIn, self.activation_input.get_read_request()),
        ]
        .into_iter()
        .filter_map(|(name, opt_req)|
            opt_req.map(|req| (name, req))
        )
        .collect()
    }
    pub fn collect_write_requests(&mut self) -> HashMap<TaluPortName, CpuRegisterWriteRequest> {
        [
            (DataOut0, self.data_output_0.get_write_request()),
            (DataOut1, self.data_output_1.get_write_request()),
            (ActivationOut, self.activation_output.get_write_request()),
        ]
        .into_iter()
        .filter_map(|(name, opt_req)|
            opt_req.map(|req| (name, req))
        )
        .collect()
    }
    pub fn new(talu_addr: usize, main_memory: &MainMemory) -> Self {
        TaluCore {
            state               : TaluState::Closing,
            addr                : talu_addr,
            main_memory         : main_memory.get_io(),
            operation           : TaluOperation::NoOp,
            old_operation       : TaluOperation::NoOp,

            inner_memory_0      : Default::default(),
            inner_memory_1      : Default::default(),

            data_input_0        : CpuRegisterDataReader::new(),
            data_input_1        : CpuRegisterDataReader::new(),
            activation_input    : CpuRegisterActReader::new(),

            data_output_0       : CpuRegisterDataWriter::new(),
            data_output_1       : CpuRegisterDataWriter::new(),
            activation_output   : CpuRegisterActWriter::new(),
        }
    }
    pub fn get_ports_info(&self) -> TaluPortsDefns {
        let ports_data = self.operation.get_ports_config();
        TaluPortsDefns {
            data_input_0: PortDefns {
                active: true,
                signal_dir: Input,
                signal_type: Data,
            },
            data_input_1: PortDefns {
                active: true,
                signal_dir: Input,
                signal_type: Data,
            },
            activation_input: PortDefns {
                active: true,
                signal_dir: Input,
                signal_type: Activation,
            },
            data_output_0: PortDefns {
                active: true,
                signal_dir: Output,
                signal_type: Data,
            },
            data_output_1: PortDefns {
                active: true,
                signal_dir: Output,
                signal_type: Data,
            },
            activation_output: PortDefns {
                active: true,
                signal_dir: Output,
                signal_type: Activation,
            },
            setup_in:  PortDefns{
                active: true,
                signal_dir: Input,
                signal_type: SignalType::TaluSetup 
            }
        }
    }


    pub fn set_new_operation(&mut self, new_operation: TaluOperation){
        self.old_operation = self.operation.clone();
        self.operation = new_operation.clone();
        self.state = TaluState::Done;

        let ports_config = new_operation.get_ports_config();
        self.data_input_0.set_connection(ports_config.data_input_0);
        self.data_input_1.set_connection(ports_config.data_input_1);
        self.activation_input
            .set_connection(ports_config.activation_input);
        self.data_output_0
            .set_connection(ports_config.data_output_0);
        self.data_output_1
            .set_connection(ports_config.data_output_1);
        self.activation_output
            .set_connection(ports_config.activation_output);
        self.inner_memory_0 = 0;
        self.inner_memory_1 = 0;
    }

    pub fn execute(&mut self) {
        let op = self.operation;
        match &op {
            TaluOperation::NoOp => {}
            TaluOperation::Mov {..} => {
                if self.activation_input.read().unwrap(){
                    let in_0 = self.data_input_0.read().unwrap();
                    self.data_output_0.write(in_0);
                    self.activation_output.write(true);
                    self.state = TaluState::JustProcessed;
                } else {
                    if self.state == TaluState::JustProcessed{
                        self.activation_output.write(false);
                        self.state = TaluState::Closing;
                    }  else {
                        self.activation_output.clear();
                    }
                }
            }
            TaluOperation::Cmp { op, .. } => {
                if self.activation_input.read().unwrap() {
                    let in_0 = self.data_input_0.read().unwrap();
                    let in_1 = self.data_input_1.read().unwrap();
                    let res = match  op{
                        CmpOp::LessThan => in_0 < in_1,
                        CmpOp::LessThanOrEq => in_0 <= in_1,
                        CmpOp::GreaterThan => in_0 > in_1,
                        CmpOp::GreaterThanOrEq => in_0 >= in_1,
                        CmpOp::Eq => in_0 == in_1,
                        CmpOp::NotEq => in_0 != in_1,
                    };
                    self.data_output_0.write(res.to_word());
                    self.activation_output.write(true);
                    self.state = TaluState::JustProcessed;
                } else {
                    if (self.state == TaluState::JustProcessed){
                        self.activation_output.write(false);
                        self.state = TaluState::Closing;
                    } else {
                        self.activation_output.clear();
                    }
                }
            }
           TaluOperation::Not {
                ..
            } => {
                if self.activation_input.read().unwrap() {
                    let inp = self.data_input_0.read().unwrap();
                    self.data_output_0.write(!inp);
                    self.activation_output.write(true);
                    self.state = TaluState::JustProcessed;
                } else {
                    if self.state == TaluState::JustProcessed{
                        self.state = TaluState::Closing;
                        self.activation_output.write(false);
                    } else {
                        self.activation_output.clear();
                    }
                }
            }
            TaluOperation::And {
                ..
            } => {
                if self.activation_input.read().unwrap() {
                    let inp_0 = self.data_input_0.read().unwrap();
                    let inp_1 = self.data_input_1.read().unwrap();

                    let res = inp_0 & inp_1;
                    self.data_output_0.write(res);
                    self.activation_output.write(true);
                    self.state = TaluState::JustProcessed;
                } else {
                    if self.state == TaluState::JustProcessed{
                        self.state = TaluState::Closing;
                        self.activation_output.write(false);
                    } else {
                        self.state = TaluState::Done;
                        self.activation_output.clear();
                    }
                }
            }
            TaluOperation::Or {
                ..
            } => {
                if self.activation_input.read().unwrap() {
                    let inp_0 = self.data_input_0.read().unwrap();
                    let inp_1 = self.data_input_1.read().unwrap();

                    let res = inp_0 | inp_1;
                    self.data_output_0.write(res);
                    self.activation_output.write(true);
                    self.state = TaluState::JustProcessed;
                } else {
                    if self.state == TaluState::JustProcessed{
                        self.state = TaluState::Closing;
                        self.activation_output.write(false);
                    } else {
                        self.state = TaluState::Done;
                        self.activation_output.clear();
                    }
                }
            }
            TaluOperation::Xor {
                ..
            } => {
                if self.activation_input.read().unwrap() {
                    let inp_0 = self.data_input_0.read().unwrap();
                    let inp_1 = self.data_input_1.read().unwrap();

                    let res = inp_0 ^ inp_1;
                    self.data_output_0.write(res);
                    self.activation_output.write(true);
                    self.state = TaluState::JustProcessed;
                } else {
                    if self.state == TaluState::JustProcessed{
                        self.state = TaluState::Closing;
                        self.activation_output.write(false);
                    } else {
                        self.state = TaluState::Done; self.activation_output.clear();
                    }
                }
            }
            TaluOperation::ShiftLeft {
                ..
            } => {
                if self.activation_input.read().unwrap() {
                    let inp_0 = self.data_input_0.read().unwrap();
                    let inp_1 = self.data_input_1.read().unwrap();

                    let res = inp_0 << inp_1;
                    self.data_output_0.write(res);

                    self.activation_output.write(true);
                    self.state = TaluState::JustProcessed;
                } else {
                    if self.state == TaluState::JustProcessed{
                        self.state = TaluState::Closing;
                        self.activation_output.write(false);
                    } else {
                        self.state = TaluState::Done;
                        self.activation_output.clear();
                    }
                }
            }
            TaluOperation::ShiftRight {
                ..
            } => {
                if self.activation_input.read().unwrap() {
                    let inp_0 = self.data_input_0.read().unwrap();
                    let inp_1 = self.data_input_1.read().unwrap();

                    let res = inp_0 >> inp_1;
                    self.data_output_0.write(res);

                    self.activation_output.write(true);
                    self.state = TaluState::JustProcessed;
                } else {
                    if self.state == TaluState::JustProcessed{
                        self.state = TaluState::Closing;
                        self.activation_output.write(false);
                    } else {
                        self.state = TaluState::Done;
                        self.activation_output.clear();
                    }
                }
            }
            TaluOperation::SelectPart { .. } => {
                todo!()
            }
            TaluOperation::Add { ..
            } => {
                if self.activation_input.read().unwrap() {
                    let inp_0 = self.data_input_0.read().unwrap();
                    let inp_1 = self.data_input_1.read().unwrap();

                    let (first_word, overflow) = inp_0.overflowing_add(inp_1);
                    self.data_output_0.write(first_word);
                    self.data_output_1.write(overflow as i32 );
                    self.activation_output.write(true);
                    self.state = TaluState::JustProcessed;
                } else {
                    if self.state == TaluState::JustProcessed{
                        self.state = TaluState::Closing;
                        self.activation_output.write(false);
                    } else {
                        self.state = TaluState::Done;
                        self.activation_output.clear();
                    }
                }
            }
            TaluOperation::Sub {
                ..
            } => {
                if self.activation_input.read().unwrap() {
                    let inp_0 = self.data_input_0.read().unwrap();
                    let inp_1 = self.data_input_1.read().unwrap();

                    let (first_word, overflow) = inp_0.overflowing_sub(inp_1);
                    self.data_output_0.write(first_word,);
                    self.data_output_1.write(overflow as i32);

                    self.activation_output.write(true);
                    self.state = TaluState::JustProcessed;
                } else {
                    if self.state == TaluState::JustProcessed{
                        self.state = TaluState::Closing;
                        self.activation_output.write(false);
                    } else {
                        self.state = TaluState::Done;
                        self.activation_output.clear();
                    }
                }
            }
            TaluOperation::Mul {
                second_word_output,
                ..
            } => {
                if self.activation_input.read().unwrap() {
                    let inp_0 = self.data_input_0.read().unwrap();
                    let inp_1 = self.data_input_1.read().unwrap();

                    if let Some(_second_word_output) = second_word_output {
                        let (first_word_res, second_word_res) =
                            (inp_0 as i32).widening_mul(inp_1 as i32);
                        self.data_output_0
                            .write(unsafe { transmute(first_word_res) },);
                        self.data_output_1
                            .write(second_word_res);
                    self.state = TaluState::JustProcessed;
                    } else {
                        self.data_output_0
                            .write((inp_0) * (inp_1), );
                    }

                    self.activation_output.write(true);
                } else {
                    if self.state == TaluState::JustProcessed{
                        self.state = TaluState::Closing;
                        self.activation_output.write(false);
                    } else {
                        self.state = TaluState::Done;
                        self.activation_output.clear();
                    }
                }
            }
            TaluOperation::Div {
                div_by_zero_flag_output,
                ..
            } => {
                if self.activation_input.read().unwrap() {
                    let dividend = self.data_input_0.read().unwrap();
                    let divisor = self.data_input_1.read().unwrap();

                    if divisor == 0 {
                        if let Some(_div_by_zero_flag_output) = div_by_zero_flag_output {
                            self.data_output_1.write(1);
                        }
                        self.data_output_0.write(0);
                    } else {
                        let res = dividend / divisor;
                        self.data_output_1.write(res);
                        if let Some(_div_by_zero_flag_output) = div_by_zero_flag_output {
                            self.data_output_1.write(0);
                        }
                    }

                    self.state = TaluState::JustProcessed;
                    self.activation_output.write(true);
                } else {
                    if self.state == TaluState::JustProcessed{
                        self.state = TaluState::Closing;
                        self.activation_output.write(false);
                    } else {
                        self.state = TaluState::Done;
                        self.activation_output.clear();
                    }
                }
            }
            TaluOperation::Rem {
                div_by_zero_flag_output,
                ..
            } => {
                if self.activation_input.read().unwrap() {
                    let dividend = self.data_input_0.read().unwrap();
                    let divisor = self.data_input_1.read().unwrap();

                    if divisor == 0 {
                        if let Some(_div_by_zero_flag_output) = div_by_zero_flag_output {
                            self.data_output_1.write(1, );
                        }
                        self.data_output_0.write(0);
                    } else {
                        let res = dividend % divisor;
                        self.data_output_1.write(res);
                        if let Some(_div_by_zero_flag_output) = div_by_zero_flag_output {
                            self.data_output_1.write(0);
                        }
                    }

                    self.activation_output.write(true);
                    self.state = TaluState::JustProcessed;
                } else {
                    if self.state == TaluState::JustProcessed{
                        self.state = TaluState::Closing;
                        self.activation_output.write(false);
                    } else {
                        self.state = TaluState::Done;
                        self.activation_output.clear();
                    }
                }
            }
            TaluOperation::Neg {
                ..
            } => {
                if self.activation_input.read().unwrap() {
                    let res = -self.data_input_0.read().unwrap();
                    self.data_output_0.write(res);

                    self.activation_output.write(true);
                    self.state = TaluState::JustProcessed;
                } else {
                    if self.state == TaluState::JustProcessed{
                        self.state = TaluState::Closing;
                        self.activation_output.write(false);
                    } else {
                        self.state = TaluState::Done;
                        self.activation_output.clear();
                    }
                }
            }
            TaluOperation::ReadFromMem {
                ..
            } => {
                if self.activation_input.read() .unwrap(){
                    let addr = self.data_input_0.read().unwrap();
                    let res = self.main_memory.read(addr as usize);
                    self.data_output_0.write(res);
                    self.activation_output.write(true);
                    self.state = TaluState::JustProcessed;
                } else {
                    if self.state == TaluState::JustProcessed{
                        self.state = TaluState::Closing;
                        self.activation_output.write(false);
                    } else {
                        self.state = TaluState::Done;
                        self.activation_output.clear();
                    }
                }
            }
            TaluOperation::WriteToMem {
                ..
            } => {
                if self.activation_input.read().unwrap() {
                    let data = self.data_input_0.read().unwrap();
                    let addr = self.data_input_1.read().unwrap();
                    self.main_memory.write(addr as usize, data);
                    self.activation_output.write(true);
                    self.state = TaluState::JustProcessed;
                } else {
                    if self.state == TaluState::JustProcessed{
                        self.state = TaluState::Closing;
                        self.activation_output.write(false);
                    } else {
                        self.state = TaluState::Done;
                        self.activation_output.clear();
                    }
                }
            }
            TaluOperation::Latch {
                ..
            } => {
                let hold_input = self.data_input_1.read().unwrap().to_bool();
                let previous_hold = self.inner_memory_1.to_bool();
                if hold_input {
                    if self.activation_input.read().unwrap() {
                        if !previous_hold {
                            let current_data = self.data_input_0.read().unwrap();
                            self.inner_memory_0 = current_data;
                            self.data_output_0.write(current_data);
                            todo!()
                        } else {
                        }
                    }
                }
                todo!()
            }
        }
    }
}

