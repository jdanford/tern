use libc::malloc;
use std::mem::transmute;

use ternary;
use trit::Trit;
use types::*;
use inst::Inst;

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

macro_rules! ptr {
	($e:expr) => (&$e[0] as *const _)
}

macro_rules! mut_ptr {
	($e:expr) => (&mut $e[0] as *mut _)
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

	pub unsafe fn write_register_int(&mut self, i: usize, value: isize) {
		ternary::write_int(mut_ptr!(self.registers[i]), value, WORD_ISIZE);
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

	pub unsafe fn read_register_int(&mut self, i: usize) -> isize {
		ternary::read_int(mut_ptr!(self.registers[i]), WORD_ISIZE)
	}

	pub unsafe fn init(&mut self) {
		ternary::write_int(mut_ptr!(self.registers[PC]), PC_START as isize, WORD_ISIZE);
		self.running = true;
	}

	pub unsafe fn run(&mut self) {
		self.init();

		while self.running {
			self.step();
		}
	}

	pub unsafe fn move_pc(&mut self, offset: isize) {
		ternary::write_int(mut_ptr!(self.registers[ZERO]), offset, WORD_ISIZE);
		ternary::addmul(mut_ptr!(self.registers[PC]), ptr!(self.registers[ZERO]), Trit::Pos, WORD_ISIZE);
		self.clear_zero();
	}

	pub unsafe fn clear_zero(&mut self) {
		ternary::clear(mut_ptr!(self.registers[ZERO]), WORD_ISIZE);
	}

	pub unsafe fn next_inst(&mut self) -> Word {
		let pc = ternary::read_int(&self.registers[PC] as *const _, WORD_ISIZE);
		if pc < 0 {
			panic!("PC is negative!")
		}

		let mut inst = [Trit::Zero; WORD_SIZE];
		ternary::copy(mut_ptr!(inst), self.memory.offset(pc as isize), WORD_ISIZE);

		inst
	}

	pub unsafe fn step(&mut self) {
		let inst = self.next_inst();
		// ternary::print(ptr!(inst), io::stdout(), WORD_ISIZE);
		let raw_opcode = ternary::read_int(ptr!(inst), TRYTE_ISIZE) as i16;
		let opcode = transmute(raw_opcode);
		match opcode {
			Inst::Mov => {
				let dest = ternary::read_int(ptr!(inst).offset(TRYTE_ISIZE), TRYTE_ISIZE);
				let src = ternary::read_int(ptr!(inst).offset(TRYTE_ISIZE * 2), TRYTE_ISIZE);
				self.mov(dest as usize, src as usize);
			}

			_ => {}
		}

		self.clear_zero();
	}

	unsafe fn mov(&mut self, i_dest: usize, i_src: usize) {
		let dest = mut_ptr!(self.registers[i_dest]);
		let src = ptr!(self.registers[i_src]);
		ternary::copy(dest, src, WORD_ISIZE);
	}
}
