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

	vm.write_register_int(dest, 40);
	vm.write_register_int(src, 13);

	vm.run();

	assert_eq!(vm.read_register_int(dest), 13);
	assert_eq!(vm.read_register_int(src), 13);
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

	vm.write_register_int(dest, 0);
	vm.write_register_int(lhs, 7);
	vm.write_register_int(rhs, -2);

	vm.run();

	assert_eq!(vm.read_register_int(dest), 5);
	assert_eq!(vm.read_register_int(lhs), 7);
	assert_eq!(vm.read_register_int(rhs), -2);
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

	vm.write_register_int(Register::HI, 999);
	vm.write_register_int(lhs, -15);
	vm.write_register_int(rhs, -3);

	vm.run();

	assert_eq!(vm.read_register_int(Register::LO), 45);
	assert_eq!(vm.read_register_int(Register::HI), 0);
	assert_eq!(vm.read_register_int(lhs), -15);
	assert_eq!(vm.read_register_int(rhs), -3);
}
