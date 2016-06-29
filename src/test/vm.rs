use ternary;
use types::*;
use opcode::Opcode;
use vm::VM;

#[test]
fn vm_mov() {
	let mut vm = VM::new(WORD_SIZE * 2);

	let dest = 0;
	let src = 1;

	let trytes = vec![Opcode::Mov as isize, dest as isize, src as isize];
	ternary::write_trytes(vm.memory, trytes);

	vm.write_register_int(dest, 40);
	vm.write_register_int(src, 13);

	assert_eq!(vm.read_register_int(dest) as isize, 40);
	assert_eq!(vm.read_register_int(src) as isize, 13);

	vm.init();
	vm.step();

	assert_eq!(vm.read_register_int(dest) as isize, 13);
	assert_eq!(vm.read_register_int(src) as isize, 13);
}

#[test]
fn vm_add() {
	let mut vm = VM::new(WORD_SIZE * 2);

	let dest = 0;
	let lhs = 1;
	let rhs = 2;

	let trytes = vec![Opcode::Add as isize, dest as isize, lhs as isize, rhs as isize];
	ternary::write_trytes(vm.memory, trytes);

	vm.write_register_int(dest, 0);
	vm.write_register_int(lhs, 7);
	vm.write_register_int(rhs, -2);

	assert_eq!(vm.read_register_int(dest) as isize, 0);
	assert_eq!(vm.read_register_int(lhs) as isize, 7);
	assert_eq!(vm.read_register_int(rhs) as isize, -2);
	// assert_eq!(vm.read_register_int(HI) as isize, 0);

	vm.init();
	vm.step();

	assert_eq!(vm.read_register_int(dest) as isize, 5);
	assert_eq!(vm.read_register_int(lhs) as isize, 7);
	assert_eq!(vm.read_register_int(rhs) as isize, -2);
	// assert_eq!(vm.read_register_int(HI) as isize, 0);
}
