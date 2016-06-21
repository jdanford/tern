use trit::Trit;
use types::*;

const REG_COUNT: usize = 48;

// const PC: usize = REG_COUNT - 4;
// const SP: usize = REG_COUNT - 3;
// const OVER: usize = REG_COUNT - 2;
// const UNDER: usize = REG_COUNT - 1;

pub struct VM {
	registers: [Word; REG_COUNT],
	memory: Vec<Trit>
}

impl VM {
	pub fn new(memory_size: usize) -> VM {
		let mut memory = Vec::with_capacity(memory_size);
		unsafe { memory.set_len(memory_size) }

		VM {
			registers: [[Trit::Zero; WORD_SIZE]; REG_COUNT],
			memory: memory
		}
	}
}
