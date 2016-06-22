use libc::malloc;
use ternary;
use trit::Trit;
use types::*;
use inst::Inst;
use std::mem::transmute;

const REGISTER_COUNT: usize = 48;

const PC: usize = REGISTER_COUNT - 5;
const SP: usize = REGISTER_COUNT - 4;
const OVER: usize = REGISTER_COUNT - 3;
const UNDER: usize = REGISTER_COUNT - 2;
const ZERO: usize = REGISTER_COUNT - 1;

const PC_START: usize = 0;

static WORD_SIZE_TRITS: &'static [Trit] = &[Trit::Zero, Trit::Neg, Trit::Zero, Trit::Pos];

pub struct VM {
	registers: [Word; REGISTER_COUNT],
	memory: *mut Trit,
	running: bool
}

macro_rules! ptr {
	($e:expr) => (
		(&$e[0] as *const _)
	)
}

macro_rules! mut_ptr {
	($e:expr) => (
		(&mut $e[0] as *mut _)
	)
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
		ternary::copy(&mut self.registers[i][0] as *mut _, src, WORD_SIZE as isize);
	}

	pub unsafe fn write_register_int(&mut self, i: usize, value: isize) {
		ternary::write_int(&mut self.registers[i][0] as *mut _, value, WORD_SIZE as isize);
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
		ternary::read_int(&mut self.registers[i][0] as *mut _, WORD_SIZE as isize)
	}

	pub unsafe fn init(&mut self) {
		ternary::write_int(&mut self.registers[PC][0] as *mut _, PC_START as isize, WORD_SIZE as isize);
		self.running = true;
	}

	pub unsafe fn run(&mut self) {
		self.init();

		while self.running {
			self.step();
		}
	}

	pub unsafe fn next_inst(&mut self) -> Word {
		let raw_pc = ternary::read_int(&self.registers[PC] as *const _, WORD_SIZE as isize);
		if raw_pc < 0 {
			panic!("PC is negative!")
		}

		let pc = raw_pc as usize;
		let mut inst = [Trit::Zero; WORD_SIZE];
		ternary::copy(&mut inst[0] as *mut _, self.memory.offset(pc as isize), WORD_SIZE as isize);
		ternary::addmul(&mut self.registers[PC][0] as *mut _, &self.registers[PC][0] as *const _, Trit::Pos, WORD_SIZE as isize);

		inst
	}

	pub unsafe fn step(&mut self) {
		let inst = self.next_inst();
		let inst_ptr = &inst as *const _;
		let raw_opcode = ternary::read_int(inst_ptr, TRYTE_SIZE as isize) as i16;
		let opcode = unsafe { transmute(raw_opcode) };
		match opcode {
			Inst::Mov => {
				let dest = ternary::read_int(inst_ptr.offset(TRYTE_SIZE as isize), TRYTE_SIZE as isize) as usize;
				let src = ternary::read_int(inst_ptr.offset((TRYTE_SIZE * 2) as isize), TRYTE_SIZE as isize) as usize;
				self.mov(dest, src);
			}

			_ => {}
		}

		ternary::clear(&mut self.registers[ZERO][0] as *mut _, WORD_SIZE as isize);
	}

	unsafe fn mov(&mut self, i_dest: usize, i_src: usize) {
		let mut dest = &mut self.registers[i_dest][0] as *mut _;
		let src = &mut self.registers[i_src][0] as *const _;
		ternary::copy(dest, src, WORD_SIZE as isize);
	}

	// fn movw(&mut self, dest: &mut Word, src: &Word) {
	// 	self.mov(dest, src)
	// }

	// fn ld(&mut self, dest: &mut Word, src: &Word) {}

	// fn ldr(&mut self, dest: &mut Word, src: &Word) {}

	// fn st(&mut self, dest: &mut Word, src: &Word) {}

	// fn str(&mut self, dest: &mut Word, src: &Word) {}

	// fn add(&mut self, dest: &mut Word, lhs: &Word, rhs: &Word) {}

	// fn addi(&mut self, dest: &mut Word, src: &Immediate) {}

	// fn mul(&mut self, dest: &mut Word, lhs: &Word, rhs: &Word) {}

	// fn muli(&mut self, dest: &mut Word, src: &Immediate) {}

	// fn not(&mut self, dest: &mut Word, src: &Word) {}

	// fn and(&mut self, dest: &mut Word, lhs: &Word, rhs: &Word) {}

	// fn andi(&mut self, dest: &mut Word, src: &Immediate) {}

	// fn or(&mut self, dest: &mut Word, lhs: &Word, rhs: &Word) {}

	// fn ori(&mut self, dest: &mut Word, src: &Immediate) {}

	// fn shf(&mut self, dest: &mut Word, lhs: &Word, rhs: &Word) {}

	// fn shfi(&mut self, dest: &mut Word, src: &Immediate) {}

	// fn cmp(&mut self, dest: &mut Word, lhs: &Word, rhs: &Word) {}

	// fn jmp(&mut self, dest: &mut Word) {}

	// fn jr(&mut self, dest: &mut Word) {}

	// fn jT(&mut self, dest: &mut Word, shift: usize) {}

	// fn j0(&mut self, dest: &mut Word, shift: usize) {}

	// fn j1(&mut self, dest: &mut Word, shift: usize) {}

	// fn jT0(&mut self, dest: &mut Word, shift: usize) {}

	// fn jT1(&mut self, dest: &mut Word, shift: usize) {}

	// fn j01(&mut self, dest: &mut Word, shift: usize) {}

	// fn call(&mut self, dest: &mut Word) {}

	// fn callr(&mut self, dest: &mut Word) {}

	// fn ret(&mut self) {}

	// fn sys(&mut self) {}
}
