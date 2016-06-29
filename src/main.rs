extern crate tern;

use tern::ternary;
use tern::types::*;
use tern::vm::VM;
use tern::opcode::Opcode;

fn main() { unsafe {
	let mut vm = VM::new(WORD_SIZE * 2);

	let dest = 0;
	let src = 1;

	ternary::write_int(vm.memory, Opcode::Mov as isize, TRYTE_ISIZE);
	ternary::write_int(vm.memory.offset(TRYTE_ISIZE), dest as isize, TRYTE_ISIZE);
	ternary::write_int(vm.memory.offset(TRYTE_ISIZE * 2), src as isize, TRYTE_ISIZE);

	vm.write_register_int(dest, 40);
	vm.write_register_int(src, 13);

	assert_eq!(vm.read_register_int(dest) as isize, 40);
	assert_eq!(vm.read_register_int(src) as isize, 13);

	vm.init();
	vm.step();
} }
