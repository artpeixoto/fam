use crate::application::grid::component::PortName;
use crate::application::simulation::talu::{TaluAddress, TaluOperation, TaluBank};
use crate::application::simulation::cpu_registers::{CpuRegisterDataReader, CpuRegisterDataWriter};
use crate::application::simulation::instruction::Instruction;
use crate::application::simulation::instruction_reader::IncrementCmd::{GoTo, Increment, NoIncrement};
use crate::application::simulation::instruction_reader::{InstructionMemory, InstructionReader};
use crate::word::ToBool;
use std::fmt::Debug;

#[derive( PartialEq, Eq, Copy, Clone, Debug, )]
pub enum ControllerExecutionState {
	Running,
	WaitingForActivation,
}
pub struct Controller{
	pub cpu_registers_reader	: CpuRegisterDataReader,
	pub cpu_registers_writer	: CpuRegisterDataWriter,
	
	pub talu_config_writer		: TaluConfigWriter	,
	pub state					: ControllerExecutionState,
	pub instruction_reader  	: InstructionReader,
	
	previous_instruction		: Option<Instruction>,
}

impl Controller{
	pub fn new(
		instruction_memory	: &InstructionMemory,
	) -> Self {
		let instruction_reader = InstructionReader::new(
			instruction_memory,
		);
			
		let configurator = TaluConfigWriter::Deactivated;
			
		Controller{
			previous_instruction: None,
			cpu_registers_reader: CpuRegisterDataReader::new(),
			cpu_registers_writer: CpuRegisterDataWriter::new(),
			talu_config_writer   : configurator,
			instruction_reader,
			state				: ControllerExecutionState::Running
		}	
	}

	pub fn reset_outputs(&mut self){
		self.talu_config_writer 	  = TaluConfigWriter::Deactivated;
		self.cpu_registers_writer = CpuRegisterDataWriter::Deactivated;
	}

	#[must_use]
	pub fn execute(&mut self) -> bool {
		match self.state {
			ControllerExecutionState::Running => {
				let Some(current_instruction) = self.instruction_reader.read().map(|i| i.to_owned()) else
				{return
					false};

				match current_instruction {
					Instruction::SetTaluConfig {  talu_config, talu_addr, } => {
						self.talu_config_writer = TaluConfigWriter::WritingToSingle{
							target: talu_addr,
							op: talu_config
						};

						self.instruction_reader.set_increment_cmd(Increment);
					}
					Instruction::SetLiteral {  literal , register,} => {
						self.cpu_registers_writer.set_connection(Some(register));
						self.cpu_registers_writer.write(literal);
						self.instruction_reader.set_increment_cmd(Increment);
					}
					Instruction::WaitForActivationSignal { register_index } => {
						self.cpu_registers_reader.set_connection(Some(register_index));
						self.state =  ControllerExecutionState::WaitingForActivation;
						self.instruction_reader.set_increment_cmd(NoIncrement);
					}
					Instruction::Jump { addr } => {
						self.instruction_reader.set_increment_cmd(GoTo(addr));
					}
					Instruction::ResetAll => {
						self.talu_config_writer = TaluConfigWriter::WritingToAll {op:
						TaluOperation::NoOp};
						self.instruction_reader.set_increment_cmd(Increment);
					}
					Instruction::NoOp => {
						self.instruction_reader.set_increment_cmd(Increment);
					}
				}
			}
			ControllerExecutionState::WaitingForActivation => {
				let is_activated = self.cpu_registers_reader.read().unwrap().to_bool();
				if is_activated {
					self.instruction_reader.set_increment_cmd(Increment);
					self.state =  ControllerExecutionState::Running;
				} else {
					self.instruction_reader.set_increment_cmd(NoIncrement);
				}
			}
		}

		self.instruction_reader.step();
		true
	}
}

pub enum TaluConfigWriter{
	Deactivated,
	WritingToSingle{
		target	: TaluAddress,
		op		: TaluOperation,
	},
	WritingToAll{
		op		: TaluOperation,
	}
}



impl TaluConfigWriter{
	pub fn get_config_write_request(&self) -> Option<TaluConfigWriteRequest>{
		match &self{
			TaluConfigWriter::Deactivated => {
				None
			}
			TaluConfigWriter::WritingToSingle { target, op } => {
				Some(TaluConfigWriteRequest { 
					address: Some(*target), 
					operation: *op 
				})
				// talu_bank.components[*target].set_new_operation(op.clone());
			}
			TaluConfigWriter::WritingToAll { op } => {
				Some(TaluConfigWriteRequest{
					address: None,
					operation: op.clone()
				})
			}
		}
	}
}

pub struct TaluConfigWriteRequest{
	address: Option<TaluAddress>,
	operation: TaluOperation, 
}

impl TaluConfigWriteRequest{
	pub fn satisfy(self, talu_bank: &mut TaluBank){
		if let Some(addr) = self.address{
			talu_bank.components[addr].set_new_operation(self.operation);
		} else{
			for talu in talu_bank.components.iter_mut() {
				talu.set_new_operation(self.operation.clone());
			}
		}
	}
	pub fn address(&self) -> &Option<TaluAddress>{
		&self.address
	}
}





#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
pub enum ControllerPortName{
	RegisterReader, 
	RegisterWriter,
	ProgramCounterReader,
	ProgramCounterWriter,
	TaluConfigWriter,
	MainMemoryReader,
}
impl PortName for ControllerPortName{
	fn all_port_names() -> Vec<Self> {
		vec![
			Self::RegisterReader,
			Self::RegisterWriter,
			Self::ProgramCounterReader,
			Self::ProgramCounterWriter,
			Self::TaluConfigWriter,
			Self::MainMemoryReader,
		]
	}

	fn small_name(&self) -> &str {
		match self{
			ControllerPortName::RegisterReader => "di",
			ControllerPortName::RegisterWriter => "do",
			ControllerPortName::ProgramCounterReader => "pci",
			ControllerPortName::ProgramCounterWriter => "pco",
			ControllerPortName::TaluConfigWriter => "ac",
			ControllerPortName::MainMemoryReader => "mr",
		}
	}
}