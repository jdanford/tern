use libc::malloc;
use std::mem::transmute;

use ternary;
use trit::Trit;
use types::*;

const REGISTER_COUNT: usize = 48;

const PC: usize = REGISTER_COUNT - 5;
const SP: usize = REGISTER_COUNT - 4;
const HI: usize = REGISTER_COUNT - 3;
const LO: usize = REGISTER_COUNT - 2;
const ZERO: usize = REGISTER_COUNT - 1;

const PC_START: usize = 0;

pub struct VM {
	pub registers: [Word; REGISTER_COUNT],
	pub memory: *mut Trit,
	pub running: bool
}

impl VM {
	pub fn new(memory_size: usize) -> VM {
		let memory = unsafe { transmute(malloc(memory_size)) };

		VM {
			registers: [[Trit::Zero; WORD_SIZE]; REGISTER_COUNT],
			memory: memory,
			running: false
		}
	}

	pub unsafe fn write_register(&mut self, i: usize, src: *const Trit) {
		ternary::copy(mut_ptr!(self.registers[i]), src, WORD_ISIZE);
	}

	pub fn write_register_int(&mut self, i: usize, value: isize) {
		unsafe { ternary::write_int(mut_ptr!(self.registers[i]), value, WORD_ISIZE); }
	}

	pub fn print_register(&self, i: usize) {
		print!("${:02} = ", i);

		for trit in self.registers[i].into_iter().rev() {
			print!("{}", trit)
		}

		println!("")
	}

	pub fn print_registers(&self) {
		for i in 0..REGISTER_COUNT {
			self.print_register(i);
		}
	}

	pub fn read_register_int(&mut self, i: usize) -> isize {
		unsafe { ternary::read_int(mut_ptr!(self.registers[i]), WORD_ISIZE) }
	}

	pub fn init(&mut self) {
		unsafe {
			ternary::write_int(mut_ptr!(self.registers[PC]), PC_START as isize, WORD_ISIZE);
		}

		self.running = true;
	}

	pub fn run(&mut self) {
		self.init();

		while self.running {
			self.step();
		}
	}

	pub unsafe fn move_pc(&mut self, offset: isize) {
		ternary::write_int(self.dest(ZERO), offset, WORD_ISIZE);
		ternary::addmul(self.dest(PC), self.src(ZERO), Trit::Pos, WORD_ISIZE);
		self.clear_zero();
	}

	pub fn clear_zero(&mut self) {
		unsafe { ternary::clear(self.dest(ZERO), WORD_ISIZE); }
	}

	pub fn next_inst(&mut self) -> Word {
		let pc = unsafe { ternary::read_int(self.src(PC), WORD_ISIZE) };
		if pc < 0 {
			panic!("PC is negative!")
		}

		let mut inst = [Trit::Zero; WORD_SIZE];
		unsafe {
			ternary::copy(mut_ptr!(inst), self.memory.offset(pc as isize), WORD_ISIZE);
		}

		inst
	}

	pub fn step(&mut self) {
		let inst = self.next_inst();
		let opcode = unsafe {
			let raw_opcode = ternary::read_int(ptr!(inst), TRYTE_ISIZE) as i16;
			transmute(raw_opcode)
		};

		use opcode::Opcode::*;
		unsafe { match opcode {
			Mov => {
				let dest = ternary::read_int(tryte_ptr!(inst, 1), TRYTE_ISIZE) as usize;
				let src = ternary::read_int(tryte_ptr!(inst, 2), TRYTE_ISIZE) as usize;
				self.mov(dest, src);
			}

			Add => {
				let dest = ternary::read_int(tryte_ptr!(inst, 1), TRYTE_ISIZE) as usize;
				let lhs = ternary::read_int(tryte_ptr!(inst, 2), TRYTE_ISIZE) as usize;
				let rhs = ternary::read_int(tryte_ptr!(inst, 3), TRYTE_ISIZE) as usize;
				self.add(dest, lhs, rhs);
			}

			_ => {}
		} }

		self.clear_zero();
	}

	fn src(&self, index: usize) -> *const Trit {
		ptr!(self.registers[index])
	}

	fn dest(&mut self, index: usize) -> *mut Trit {
		mut_ptr!(self.registers[index])
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
			let carry = ternary::add(dest, lhs, rhs, WORD_ISIZE);
			ternary::clear(self.dest(HI), WORD_ISIZE);
			ternary::set_trit(self.dest(HI), 0, carry);
		}
	}
}
