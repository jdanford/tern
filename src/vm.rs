use libc::malloc;
use std::mem::transmute;

use ternary;
use trit::Trit;
use types::*;
use opcodes::Opcode;
use registers::*;

const PC_START: usize = 0;

pub struct VM {
	pub registers: [Word; REGISTER_COUNT],
	pub memory: *mut Trit,
	pub pc: usize,
	pub running: bool
}

impl VM {
	pub fn new(memory_size: usize) -> VM {
		let registers = [[Trit::Zero; WORD_SIZE]; REGISTER_COUNT];
		let memory = unsafe { transmute(malloc(memory_size)) };

		VM {
			registers: registers,
			memory: memory,
			pc: PC_START,
			running: false
		}
	}

	fn src(&self, i: usize) -> *const Trit {
		ptr!(self.registers[i])
	}

	fn dest(&mut self, i: usize) -> *mut Trit {
		mut_ptr!(self.registers[i])
	}

	pub fn read_register_int(&mut self, i: usize) -> isize {
		unsafe { ternary::read_int(self.dest(i), WORD_ISIZE) }
	}

	pub fn write_register_int(&mut self, i: usize, value: isize) {
		unsafe { ternary::write_int(self.dest(i), value, WORD_ISIZE); }
	}

	pub fn init(&mut self) {
		self.running = true;
	}

	pub fn run(&mut self) {
		self.init();

		while self.running {
			self.step();
		}
	}

	pub fn clear_zero(&mut self) {
		unsafe { ternary::clear(self.dest(ZERO), WORD_ISIZE); }
	}

	pub fn next_inst(&mut self) -> Word {
		let mut inst = [Trit::Zero; WORD_SIZE];

		unsafe {
			let location = self.memory.offset(self.pc as isize);
			ternary::copy(mut_ptr!(inst), location, WORD_ISIZE);
		}

		self.pc += WORD_SIZE;
		inst
	}

	pub fn step(&mut self) {
		let inst = self.next_inst();
		let opcode = unsafe {
			let raw_opcode = ternary::read_int(ptr!(inst), TRYTE_ISIZE) as i16;
			transmute(raw_opcode)
		};

		unsafe { match opcode {
			Opcode::Mov => {
				let dest = ternary::read_int(tryte_ptr!(inst, 1), TRYTE_ISIZE) as usize;
				let src = ternary::read_int(tryte_ptr!(inst, 2), TRYTE_ISIZE) as usize;
				self.mov(dest, src);
			}

			Opcode::Add => {
				let dest = ternary::read_int(tryte_ptr!(inst, 1), TRYTE_ISIZE) as usize;
				let lhs = ternary::read_int(tryte_ptr!(inst, 2), TRYTE_ISIZE) as usize;
				let rhs = ternary::read_int(tryte_ptr!(inst, 3), TRYTE_ISIZE) as usize;
				self.add(dest, lhs, rhs);
			}

			Opcode::Mul => {
				let lhs = ternary::read_int(tryte_ptr!(inst, 1), TRYTE_ISIZE) as usize;
				let rhs = ternary::read_int(tryte_ptr!(inst, 2), TRYTE_ISIZE) as usize;
				self.mul(lhs, rhs);
			}

			Opcode::Halt => {
				self.running = false;
			}

			_ => {}
		} }

		self.clear_zero();
	}

	fn mov(&mut self, i_dest: usize, i_src: usize) {
		let dest = self.dest(i_dest);
		let src = self.src(i_src);
		unsafe { ternary::copy(dest, src, WORD_ISIZE); }
	}

	fn add(&mut self, i_dest: usize, i_lhs: usize, i_rhs: usize) {
		let dest = self.dest(i_dest);
		let lhs = self.src(i_lhs);
		let rhs = self.src(i_rhs);

		unsafe {
			ternary::clear(dest, WORD_ISIZE);
			let carry = ternary::add(dest, lhs, rhs, WORD_ISIZE);
			ternary::clear(self.dest(HI), WORD_ISIZE);
			ternary::set_trit(self.dest(HI), 0, carry);
		}
	}

	fn mul(&mut self, i_lhs: usize, i_rhs: usize) {
		let lhs = self.src(i_lhs);
		let rhs = self.src(i_rhs);

		unsafe {
			ternary::clear(self.dest(LO), WORD_ISIZE);
			ternary::clear(self.dest(HI), WORD_ISIZE);
			ternary::multiply(self.dest(LO), lhs, rhs, WORD_ISIZE);
		}
	}
}
