use ternary;
use types::*;
use opcodes::Opcode;
use registers::*;
use vm::VM;

#[test]
fn vm_mov() {
	let mut vm = VM::new(WORD_SIZE * 2);

	let dest = A0;
	let src = A1;

	ternary::write_trytes(vm.memory, vec![
		Opcode::Mov as isize, dest as isize, src as isize, 0,
		Opcode::Halt as isize
	]);

	vm.write_register_int(dest as usize, 40);
	vm.write_register_int(src as usize, 13);

	assert_eq!(vm.read_register_int(dest as usize), 40);
	assert_eq!(vm.read_register_int(src as usize), 13);

	vm.run();

	assert_eq!(vm.read_register_int(dest as usize), 13);
	assert_eq!(vm.read_register_int(src as usize), 13);
}

#[test]
fn vm_add() {
	let mut vm = VM::new(WORD_SIZE * 2);

	let dest = A0;
	let lhs = A1;
	let rhs = A2;

	ternary::write_trytes(vm.memory, vec![
		Opcode::Add as isize, dest as isize, lhs as isize, rhs as isize,
		Opcode::Halt as isize
	]);

	vm.write_register_int(dest, 0);
	vm.write_register_int(lhs, 7);
	vm.write_register_int(rhs, -2);

	assert_eq!(vm.read_register_int(dest), 0);
	assert_eq!(vm.read_register_int(lhs), 7);
	assert_eq!(vm.read_register_int(rhs), -2);

	vm.run();

	assert_eq!(vm.read_register_int(dest), 5);
	assert_eq!(vm.read_register_int(lhs), 7);
	assert_eq!(vm.read_register_int(rhs), -2);
}

#[test]
fn vm_mul() {
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
