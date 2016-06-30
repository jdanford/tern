extern crate tern;

use tern::ternary;
use tern::types::*;
use tern::opcodes::Opcode;
use tern::registers::Register::*;
use tern::vm::VM;

fn main() {
	let mut vm = VM::new(WORD_SIZE * 2);

	let lhs = A0;
	let rhs = A1;

	ternary::write_trytes(vm.memory, vec![
		Opcode::Mul as isize, lhs as isize, rhs as isize, 0,
		Opcode::Halt as isize
	]);

	vm.write(HI, 999);
	vm.write(lhs, -15);
	vm.write(rhs, -3);

	assert_eq!(vm.read(LO), 0);
	assert_eq!(vm.read(HI), 999);
	assert_eq!(vm.read(lhs), -15);
	assert_eq!(vm.read(rhs), -3);

	vm.run();

	assert_eq!(vm.read(LO), 45);
	assert_eq!(vm.read(HI), 0);
	assert_eq!(vm.read(lhs), -15);
	assert_eq!(vm.read(rhs), -3);
}
