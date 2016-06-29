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

	fn src(&self, r: Register) -> *const Trit {
		ptr!(self.registers[r as usize])
	}

	fn dest(&mut self, r: Register) -> *mut Trit {
		mut_ptr!(self.registers[r as usize])
	}

	pub fn read_register_int(&mut self, r: Register) -> isize {
		unsafe { ternary::read_int(self.dest(r), WORD_ISIZE) }
	}

	pub fn write_register_int(&mut self, r: Register, value: isize) {
		unsafe { ternary::write_int(self.dest(r), value, WORD_ISIZE); }
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
		unsafe { ternary::clear(self.dest(Register::ZERO), WORD_ISIZE); }
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
		let opcode = unsafe { transmute(t0 as i16) };
		let (r1, r2, r3) = unsafe { (transmute(t1 as i16), transmute(t2 as i16), transmute(t3 as i16)) };

		match opcode {
			Opcode::Mov => {
				self.mov(r1, r2);
			}

			Opcode::Add => {
				self.add(r1, r2, r3);
			}

			Opcode::Mul => {
				self.mul(r1, r2);
			}

			Opcode::Halt => {
				self.running = false;
			}

			_ => {}
		}

		self.clear_zero();
	}

	fn mov(&mut self, r_dest: Register, r_src: Register) {
		let dest = self.dest(r_dest);
		let src = self.src(r_src);
		unsafe { ternary::copy(dest, src, WORD_ISIZE); }
	}

	fn add(&mut self, r_dest: Register, r_lhs: Register, r_rhs: Register) {
		let dest = self.dest(r_dest);
		let lhs = self.src(r_lhs);
		let rhs = self.src(r_rhs);

		unsafe {
			ternary::clear(dest, WORD_ISIZE);
			let carry = ternary::add(dest, lhs, rhs, WORD_ISIZE);
			ternary::clear(self.dest(Register::HI), WORD_ISIZE);
			ternary::set_trit(self.dest(Register::HI), 0, carry);
		}
	}

	fn mul(&mut self, r_lhs: Register, r_rhs: Register) {
		let lhs = self.src(r_lhs);
		let rhs = self.src(r_rhs);

		unsafe {
			ternary::clear(self.dest(Register::LO), WORD_ISIZE);
			ternary::clear(self.dest(Register::HI), WORD_ISIZE);
			ternary::multiply(self.dest(Register::LO), lhs, rhs, WORD_ISIZE);
		}
	}
}
