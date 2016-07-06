use libc::{malloc, free};
use rlibc::{memset};
use std::mem::transmute;

use trit::Trit;
use ternary;
use types::*;
use opcodes::Opcode;
use registers::*;

const PC_START: Addr = 0;

pub struct VM {
	pub registers: [Word; REGISTER_COUNT],
	pub memory: *mut Trit,
	pub pc: Addr,
	pub running: bool
}

impl VM {
	pub fn new(memory_size: usize) -> VM {
		let registers = [[Trit::Zero; WORD_SIZE]; REGISTER_COUNT];
		let memory = unsafe {
			let ptr = malloc(memory_size);
			memset(transmute(ptr), 0, memory_size);
			transmute(ptr)
		};

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

	pub fn read(&mut self, r: Register) -> isize {
		unsafe { ternary::read_int(self.src(r), WORD_ISIZE) }
	}

	pub fn write(&mut self, r: Register, value: isize) {
		unsafe { ternary::write_int(self.dest(r), value, WORD_ISIZE); }
	}

	pub fn clear(&mut self, r: Register) {
		unsafe { ternary::clear(self.dest(r), WORD_ISIZE) }
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

	fn next_inst_word(&mut self) -> Word {
		let mut inst = [Trit::Zero; WORD_SIZE];

		unsafe {
			let location = self.memory.offset(self.pc as isize);
			ternary::copy(mut_ptr!(inst), location, WORD_ISIZE);
		}

		self.pc += WORD_SIZE;
		inst
	}

	fn next_inst_int(&mut self) -> isize {
		let word = self.next_inst_word();
		unsafe { ternary::read_int(ptr!(word), WORD_ISIZE) }
	}

	pub fn step(&mut self) {
		let inst = self.next_inst_word();
		let (t0, t1, t2, t3) = ternary::read_trytes(ptr!(inst));
		let opcode = Opcode::from(t0);

		match opcode {
			Opcode::Mov => {
				self.mov(Register::from(t1), Register::from(t2));
			}

			Opcode::Add => {
				self.add(Register::from(t1), Register::from(t2), Register::from(t3));
			}

			Opcode::Mul => {
				self.mul(Register::from(t1), Register::from(t2));
			}

			Opcode::Jmp => {
				let addr = self.next_inst_int() as Addr;
				self.jmp(addr);
			}

			Opcode::Call => {
				let addr = self.next_inst_int() as Addr;
				self.call(addr);
			}

			Opcode::Ret => {
				self.ret();
			}

			Opcode::Halt => {
				self.running = false;
			}

			_ => {}
		}

		self.clear(Register::ZERO);
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
			self.clear(r_dest);
			let carry = ternary::add(dest, lhs, rhs, WORD_ISIZE);
			self.clear(Register::HI);
			ternary::set_trit(self.dest(Register::HI), 0, carry);
		}
	}

	fn mul(&mut self, r_lhs: Register, r_rhs: Register) {
		let lhs = self.src(r_lhs);
		let rhs = self.src(r_rhs);

		unsafe {
			self.clear(Register::LO);
			self.clear(Register::HI);
			ternary::multiply(self.dest(Register::LO), lhs, rhs, WORD_ISIZE);
		}
	}

	fn jmp(&mut self, addr: Addr) {
		self.pc = addr;
	}

	fn call(&mut self, addr: Addr) {
		let pc = self.pc as isize;
		self.write(Register::RA, pc);
		self.jmp(addr);
	}

	fn ret(&mut self) {
		let addr = self.read(Register::RA) as Addr;
		self.jmp(addr);
	}
}

impl Drop for VM {
	fn drop(&mut self) {
		unsafe { free(transmute(self.memory)) };
	}
}
