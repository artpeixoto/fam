use std::ops::Deref;
use std::sync::Arc;
use crate::application::draw::instruction_memory::InstructionMemoryCurrentPosition;
use crate::{Step,  PROGRAM_COUNTER_REGISTER_ADDR};
use crate::application::simulation::cpu_registers::{CpuRegisterDataReader, CpuRegisterDataWriter, };
use crate::application::simulation::instruction::Instruction;
use crate::application::simulation::main_memory::MainMemory;
use crate::application::simulation::cpu_registers::CpuRegisterBank;
use crate::application::simulation::main_memory::MainMemoryIo;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
pub enum IncrementCmd{
	NoIncrement,
	Increment,
	GoTo(i32),
}

pub struct InstructionMemory(
	pub Arc<Vec<Instruction>>,
);

impl InstructionMemory{
	pub fn new(program: Vec<Instruction>) -> Self {
		Self(Arc::new(program))
	}
}

pub struct InstructionReader{
	pub program_counter_reader	: CpuRegisterDataReader,
	pub program_counter_writer  : CpuRegisterDataWriter,
	increment_cmd				: IncrementCmd,
	instruction_memory			: Arc<Vec<Instruction>>,
}

impl InstructionReader{
	pub fn new (
		instruction_memory	: &InstructionMemory,
	) -> InstructionReader {
		Self {
			instruction_memory		: instruction_memory.0.clone(),
			program_counter_reader	: CpuRegisterDataReader::Connected {source:
			PROGRAM_COUNTER_REGISTER_ADDR, value: None},
			program_counter_writer	: CpuRegisterDataWriter::Deactivated,
			increment_cmd			: IncrementCmd::Increment,
		}
	}
}

impl InstructionReader{
	pub fn set_increment_cmd(&mut self, cmd: IncrementCmd){
		self.increment_cmd = cmd;
	}

	pub fn read<'a>(&'a self) -> Option<impl Deref<Target=Instruction> + 'a>{
		let addr = self.program_counter_reader.read().unwrap() as usize;
		self.instruction_memory.get(addr)
	}

	pub fn get_instruction_pos(&self) -> Option<InstructionMemoryCurrentPosition>{
		Some(self.program_counter_reader.read()? as usize)
	}
	pub fn step(&mut self) {
		match self.increment_cmd {
		    IncrementCmd::Increment => {
				let current_pc = self.program_counter_reader.read().unwrap() ;
				self.program_counter_writer.set_connection(Some(PROGRAM_COUNTER_REGISTER_ADDR));
				self.program_counter_writer.write(current_pc+1);
			},
			IncrementCmd::NoIncrement => {
				self.program_counter_writer.set_connection(None);
			},
			IncrementCmd::GoTo(new_pc) => {
				self.program_counter_writer.set_connection(Some(PROGRAM_COUNTER_REGISTER_ADDR));
				self.program_counter_writer.write(new_pc );
			}
		}
	}
}