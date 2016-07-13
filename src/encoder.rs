use std::collections::HashMap;

use trit::Trit;
use ternary;
use types::*;
use opcodes::Opcode;
use registers::Register;
use instructions::Instruction;
use program::Program;

#[derive(Debug)]
pub enum EncodeError {
	InsufficientMemory,
	InvalidLabel(Label),
	IntOutOfRange(isize, isize, isize),
}

pub struct Encoder {
	memory: *mut Trit,
	memory_size: usize,
	labels: HashMap<Label, Addr>,
	pc: Addr,
}

impl Encoder {
	pub fn new(memory: *mut Trit, memory_size: usize) -> Encoder {
		Encoder {
			memory: memory,
			memory_size: memory_size,
			labels: HashMap::new(),
			pc: 0,
		}
	}

	fn add_labels(&mut self, labels: HashMap<Label, Addr>) {
		for (label, &addr) in labels.iter() {
			self.labels.insert(label.clone(), addr);
		}
	}

	pub fn encode(&mut self, program: Program) -> Result<usize, EncodeError> {
		let pc_start = self.pc;
		let pc_end = pc_start + program.pc;
		try!(self.check_pc(pc_end));

		self.add_labels(program.labels);

		for instruction in program.instructions.iter() {
			let inst_size = instruction.size();

			let new_pc = self.pc + inst_size;
			try!(self.check_pc(new_pc));

			unsafe {
				let local_memory = self.memory.offset(self.pc as isize);
				ternary::clear(local_memory, inst_size as isize);
				try!(self.encode_instruction(local_memory, instruction));
			};

			self.pc = new_pc;
		}

		let size = self.pc - pc_start;
		Ok(size)
	}

	unsafe fn encode_instruction(&self, memory: *mut Trit, instruction: &Instruction) -> Result<(), EncodeError> {
		match *instruction {
			Instruction::Mov(r1, r2) => {
				try!(self.encode_opcode(memory, Opcode::Mov));
				try!(self.encode_register(tryte_offset!(memory, 1), r1));
				try!(self.encode_register(tryte_offset!(memory, 2), r2));
			}

			Instruction::Movi(r, half) => {
				try!(self.encode_opcode(memory, Opcode::Movi));
				try!(self.encode_register(tryte_offset!(memory, 1), r));
				try!(self.encode_halfword(tryte_offset!(memory, 2), half));
			}

			Instruction::Movw(r, word) => {
				try!(self.encode_opcode(memory, Opcode::Movw));
				try!(self.encode_register(tryte_offset!(memory, 1), r));
				try!(self.encode_word(tryte_offset!(memory, 4), word));
			}

			Instruction::Lb(r1, r2, offset) => {
				try!(self.encode_opcode(memory, Opcode::Lb));
				try!(self.encode_register(tryte_offset!(memory, 1), r1));
				try!(self.encode_register(tryte_offset!(memory, 2), r2));
				try!(self.encode_tryte(tryte_offset!(memory, 3), offset));
			}

			Instruction::Lh(r1, r2, offset) => {
				try!(self.encode_opcode(memory, Opcode::Lh));
				try!(self.encode_register(tryte_offset!(memory, 1), r1));
				try!(self.encode_register(tryte_offset!(memory, 2), r2));
				try!(self.encode_tryte(tryte_offset!(memory, 3), offset));
			}

			Instruction::Lw(r1, r2, offset) => {
				try!(self.encode_opcode(memory, Opcode::Lw));
				try!(self.encode_register(tryte_offset!(memory, 1), r1));
				try!(self.encode_register(tryte_offset!(memory, 2), r2));
				try!(self.encode_tryte(tryte_offset!(memory, 3), offset));
			}

			Instruction::Sb(r1, r2, offset) => {
				try!(self.encode_opcode(memory, Opcode::Sb));
				try!(self.encode_register(tryte_offset!(memory, 1), r1));
				try!(self.encode_register(tryte_offset!(memory, 2), r2));
				try!(self.encode_tryte(tryte_offset!(memory, 3), offset));
			}

			Instruction::Sh(r1, r2, offset) => {
				try!(self.encode_opcode(memory, Opcode::Sh));
				try!(self.encode_register(tryte_offset!(memory, 1), r1));
				try!(self.encode_register(tryte_offset!(memory, 2), r2));
				try!(self.encode_tryte(tryte_offset!(memory, 3), offset));
			}

			Instruction::Sw(r1, r2, offset) => {
				try!(self.encode_opcode(memory, Opcode::Sw));
				try!(self.encode_register(tryte_offset!(memory, 1), r1));
				try!(self.encode_register(tryte_offset!(memory, 2), r2));
				try!(self.encode_tryte(tryte_offset!(memory, 3), offset));
			}

			Instruction::Add(r1, r2, r3) => {
				try!(self.encode_opcode(memory, Opcode::Add));
				try!(self.encode_register(tryte_offset!(memory, 1), r1));
				try!(self.encode_register(tryte_offset!(memory, 2), r2));
				try!(self.encode_register(tryte_offset!(memory, 3), r3));
			}

			Instruction::Addi(r, half) => {
				try!(self.encode_opcode(memory, Opcode::Addi));
				try!(self.encode_register(tryte_offset!(memory, 1), r));
				try!(self.encode_halfword(tryte_offset!(memory, 2), half));
			}

			Instruction::Mul(r1, r2) => {
				try!(self.encode_opcode(memory, Opcode::Mul));
				try!(self.encode_register(tryte_offset!(memory, 1), r1));
				try!(self.encode_register(tryte_offset!(memory, 2), r2));
			}

			Instruction::Muli(r, half) => {
				try!(self.encode_opcode(memory, Opcode::Muli));
				try!(self.encode_register(tryte_offset!(memory, 1), r));
				try!(self.encode_halfword(tryte_offset!(memory, 2), half));
			}

			Instruction::Not(r1, r2) => {
				try!(self.encode_opcode(memory, Opcode::Not));
				try!(self.encode_register(tryte_offset!(memory, 1), r1));
				try!(self.encode_register(tryte_offset!(memory, 2), r2));
			}

			Instruction::And(r1, r2, r3) => {
				try!(self.encode_opcode(memory, Opcode::And));
				try!(self.encode_register(tryte_offset!(memory, 1), r1));
				try!(self.encode_register(tryte_offset!(memory, 2), r2));
				try!(self.encode_register(tryte_offset!(memory, 3), r3));
			}

			Instruction::Andi(r, half) => {
				try!(self.encode_opcode(memory, Opcode::Andi));
				try!(self.encode_register(tryte_offset!(memory, 1), r));
				try!(self.encode_halfword(tryte_offset!(memory, 2), half));
			}

			Instruction::Or(r1, r2, r3) => {
				try!(self.encode_opcode(memory, Opcode::Or));
				try!(self.encode_register(tryte_offset!(memory, 1), r1));
				try!(self.encode_register(tryte_offset!(memory, 2), r2));
				try!(self.encode_register(tryte_offset!(memory, 3), r3));
			}

			Instruction::Ori(r, half) => {
				try!(self.encode_opcode(memory, Opcode::Ori));
				try!(self.encode_register(tryte_offset!(memory, 1), r));
				try!(self.encode_halfword(tryte_offset!(memory, 2), half));
			}

			Instruction::Shf(r1, r2, r3) => {
				try!(self.encode_opcode(memory, Opcode::Shf));
				try!(self.encode_register(tryte_offset!(memory, 1), r1));
				try!(self.encode_register(tryte_offset!(memory, 2), r2));
				try!(self.encode_register(tryte_offset!(memory, 3), r3));
			}

			Instruction::Shfi(r, half) => {
				try!(self.encode_opcode(memory, Opcode::Shfi));
				try!(self.encode_register(tryte_offset!(memory, 1), r));
				try!(self.encode_halfword(tryte_offset!(memory, 2), half));
			}

			Instruction::Cmp(r1, r2, r3) => {
				try!(self.encode_opcode(memory, Opcode::Cmp));
				try!(self.encode_register(tryte_offset!(memory, 1), r1));
				try!(self.encode_register(tryte_offset!(memory, 2), r2));
				try!(self.encode_register(tryte_offset!(memory, 3), r3));
			}

			Instruction::Jmp(ref label) => {
				try!(self.encode_opcode(memory, Opcode::Jmp));
				try!(self.encode_label(tryte_offset!(memory, 4), label));
			}

			Instruction::Jr(r) => {
				try!(self.encode_opcode(memory, Opcode::Jr));
				try!(self.encode_register(tryte_offset!(memory, 1), r));
			}

			Instruction::JT(r, ref label) => {
				try!(self.encode_opcode(memory, Opcode::JT));
				try!(self.encode_register(tryte_offset!(memory, 1), r));
				try!(self.encode_relative_label(tryte_offset!(memory, 2), label));
			}

			Instruction::J0(r, ref label) => {
				try!(self.encode_opcode(memory, Opcode::J0));
				try!(self.encode_register(tryte_offset!(memory, 1), r));
				try!(self.encode_relative_label(tryte_offset!(memory, 2), label));
			}

			Instruction::J1(r, ref label) => {
				try!(self.encode_opcode(memory, Opcode::J1));
				try!(self.encode_register(tryte_offset!(memory, 1), r));
				try!(self.encode_relative_label(tryte_offset!(memory, 2), label));
			}

			Instruction::JT0(r, ref label) => {
				try!(self.encode_opcode(memory, Opcode::JT0));
				try!(self.encode_register(tryte_offset!(memory, 1), r));
				try!(self.encode_relative_label(tryte_offset!(memory, 2), label));
			}

			Instruction::JT1(r, ref label) => {
				try!(self.encode_opcode(memory, Opcode::JT1));
				try!(self.encode_register(tryte_offset!(memory, 1), r));
				try!(self.encode_relative_label(tryte_offset!(memory, 2), label));
			}

			Instruction::J01(r, ref label) => {
				try!(self.encode_opcode(memory, Opcode::J01));
				try!(self.encode_register(tryte_offset!(memory, 1), r));
				try!(self.encode_relative_label(tryte_offset!(memory, 2), label));
			}

			Instruction::Call(ref label) => {
				try!(self.encode_opcode(memory, Opcode::Call));
				try!(self.encode_label(tryte_offset!(memory, 4), label));
			}

			Instruction::Callr(r) => {
				try!(self.encode_opcode(memory, Opcode::Callr));
				try!(self.encode_register(tryte_offset!(memory, 1), r));
			}

			Instruction::Ret => {
				try!(self.encode_opcode(memory, Opcode::Ret));
			}

			Instruction::Sys => {
				try!(self.encode_opcode(memory, Opcode::Sys));
			}

			Instruction::Break => {
				try!(self.encode_opcode(memory, Opcode::Break));
			}

			Instruction::Halt => {
				try!(self.encode_opcode(memory, Opcode::Halt));
			}
		}

		Ok(())
	}

	unsafe fn encode_opcode(&self, memory: *mut Trit, opcode: Opcode) -> Result<(), EncodeError> {
		ternary::from_int(memory, opcode as isize, WORD_ISIZE);
		Ok(())
	}

	unsafe fn encode_register(&self, memory: *mut Trit, register: Register) -> Result<(), EncodeError> {
		ternary::from_int(memory, register as isize, WORD_ISIZE);
		Ok(())
	}

	unsafe fn encode_tryte(&self, memory: *mut Trit, tryte: Tryte) -> Result<(), EncodeError> {
		ternary::copy(memory, ptr!(tryte), TRYTE_ISIZE);
		Ok(())
	}

	unsafe fn encode_halfword(&self, memory: *mut Trit, halfword: Halfword) -> Result<(), EncodeError> {
		ternary::copy(memory, ptr!(halfword), HALFWORD_ISIZE);
		Ok(())
	}

	unsafe fn encode_word(&self, memory: *mut Trit, word: Word) -> Result<(), EncodeError> {
		ternary::copy(memory, ptr!(word), WORD_ISIZE);
		Ok(())
	}

	unsafe fn encode_label(&self, memory: *mut Trit, label: &Label) -> Result<(), EncodeError> {
		let addr = try!(self.label_addr(label));
		ternary::from_int(memory, addr as isize, WORD_ISIZE);
		Ok(())
	}

	unsafe fn encode_relative_label(&self, memory: *mut Trit, label: &Label) -> Result<(), EncodeError> {
		let reladdr = try!(self.relative_addr(label));
		ternary::from_int(memory, reladdr, WORD_ISIZE);
		Ok(())
	}

	fn check_pc(&self, pc: usize) -> Result<(), EncodeError> {
		if pc > self.memory_size {
			Err(EncodeError::InsufficientMemory)
		} else {
			Ok(())
		}
	}

	pub fn label_addr(&self, label: &Label) -> Result<Addr, EncodeError> {
		match self.labels.get(label) {
			Some(&addr) => Ok(addr),
			_ => Err(EncodeError::InvalidLabel(label.clone()))
		}
	}

	pub fn relative_addr(&self, label: &Label) -> Result<RelAddr, EncodeError> {
		let addr = try!(self.label_addr(label));
		Ok(addr as RelAddr - self.pc as RelAddr)
	}
}
