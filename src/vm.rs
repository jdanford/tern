use libc::malloc;
use std::mem::transmute;

use trit::Trit;
use ternary;
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

	fn clear_zero(&mut self) {
		unsafe { ternary::clear(self.dest(ZERO), WORD_ISIZE); }
	}

	fn next_inst(&mut self) -> Word {
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
		let (t0, t1, t2, t3) = ternary::read_trytes(ptr!(inst));
		let raw_opcode = t0 as i16;
		let opcode = unsafe { transmute(raw_opcode) };

		match opcode {
			Opcode::Mov => {
				self.mov(t1 as usize, t2 as usize);
			}

			Opcode::Add => {
				self.add(t1 as usize, t2 as usize, t3 as usize);
			}

			Opcode::Mul => {
				self.mul(t1 as usize, t2 as usize);
			}

			Opcode::Halt => {
				self.running = false;
			}

			_ => {}
		}

		self.clear_zero();
	}

	fn mov(&mut self, r_dest: usize, r_src: usize) {
		let dest = self.dest(r_dest);
		let src = self.src(r_src);
		unsafe { ternary::copy(dest, src, WORD_ISIZE); }
	}

	fn add(&mut self, r_dest: usize, r_lhs: usize, r_rhs: usize) {
		let dest = self.dest(r_dest);
		let lhs = self.src(r_lhs);
		let rhs = self.src(r_rhs);

		unsafe {
			ternary::clear(dest, WORD_ISIZE);
			let carry = ternary::add(dest, lhs, rhs, WORD_ISIZE);
			ternary::clear(self.dest(HI), WORD_ISIZE);
			ternary::set_trit(self.dest(HI), 0, carry);
		}
	}

	fn mul(&mut self, r_lhs: usize, r_rhs: usize) {
		let lhs = self.src(r_lhs);
		let rhs = self.src(r_rhs);

		unsafe {
			ternary::clear(self.dest(LO), WORD_ISIZE);
			ternary::clear(self.dest(HI), WORD_ISIZE);
			ternary::multiply(self.dest(LO), lhs, rhs, WORD_ISIZE);
		}
	}
}
