extern crate tern;

use tern::ternary;
use tern::types::*;
use tern::opcodes::Opcode;
use tern::registers::*;
use tern::vm::VM;

fn main() {
	let mut vm = VM::new(WORD_SIZE * 2);

	let lhs = A0;
	let rhs = A1;

	ternary::write_trytes(vm.memory, vec![
		Opcode::Mul as isize, lhs as isize, rhs as isize, 0,
		Opcode::Halt as isize
	]);

	vm.write_register_int(HI, 999);
	vm.write_register_int(lhs, -15);
	vm.write_register_int(rhs, -3);

	assert_eq!(vm.read_register_int(LO), 0);
	assert_eq!(vm.read_register_int(HI), 999);
	assert_eq!(vm.read_register_int(lhs), -15);
	assert_eq!(vm.read_register_int(rhs), -3);

	vm.run();

	assert_eq!(vm.read_register_int(LO), 45);
	assert_eq!(vm.read_register_int(HI), 0);
	assert_eq!(vm.read_register_int(lhs), -15);
	assert_eq!(vm.read_register_int(rhs), -3);
}
