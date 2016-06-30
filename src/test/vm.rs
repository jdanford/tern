use ternary;
use types::*;
use opcodes::Opcode;
use registers::Register;
use vm::VM;

#[test]
fn vm_mov() {
	let mut vm = VM::new(WORD_SIZE * 2);

	let dest = Register::A0;
	let src = Register::A1;

	ternary::write_trytes(vm.memory, vec![
		Opcode::Mov as isize, dest as isize, src as isize, 0,
		Opcode::Halt as isize
	]);

	vm.write(dest, 40);
	vm.write(src, 13);

	vm.run();

	assert_eq!(vm.read(dest), 13);
	assert_eq!(vm.read(src), 13);
}

#[test]
fn vm_add() {
	let mut vm = VM::new(WORD_SIZE * 2);

	let dest = Register::A0;
	let lhs = Register::A1;
	let rhs = Register::A2;

	ternary::write_trytes(vm.memory, vec![
		Opcode::Add as isize, dest as isize, lhs as isize, rhs as isize,
		Opcode::Halt as isize
	]);

	vm.write(dest, 0);
	vm.write(lhs, 7);
	vm.write(rhs, -2);

	vm.run();

	assert_eq!(vm.read(dest), 5);
	assert_eq!(vm.read(lhs), 7);
	assert_eq!(vm.read(rhs), -2);
}

#[test]
fn vm_mul() {
	let mut vm = VM::new(WORD_SIZE * 2);

	let lhs = Register::A0;
	let rhs = Register::A1;

	ternary::write_trytes(vm.memory, vec![
		Opcode::Mul as isize, lhs as isize, rhs as isize, 0,
		Opcode::Halt as isize
	]);

	vm.write(Register::HI, 999);
	vm.write(lhs, -15);
	vm.write(rhs, -3);

	vm.run();

	assert_eq!(vm.read(Register::LO), 45);
	assert_eq!(vm.read(Register::HI), 0);
	assert_eq!(vm.read(lhs), -15);
	assert_eq!(vm.read(rhs), -3);
}
