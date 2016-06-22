// use trit::Trit;
use ternary;
use types::*;
use inst::Inst;
use vm::VM;

// #[test]
fn vm_mov() {
	let mut vm = VM::new(WORD_SIZE * 2);

	let dest = 0 as usize;
	let src = 1 as usize;

	vm.with_memory(|memory| {
		ternary::write_int(&mut memory[..], Inst::Mov as isize, TRYTE_SIZE);
		ternary::write_int(&mut memory[TRYTE_SIZE..], dest as isize, TRYTE_SIZE);
		ternary::write_int(&mut memory[(TRYTE_SIZE * 2)..], src as isize, TRYTE_SIZE);
	});

	vm.write_register_int(dest, 40);
	vm.write_register_int(src, 13);

	assert_eq!(vm.read_register_int(dest) as isize, 40);
	assert_eq!(vm.read_register_int(src) as isize, 13);

	vm.init();
	vm.step();

	assert_eq!(vm.read_register_int(dest) as isize, 13);
	assert_eq!(vm.read_register_int(src) as isize, 13);
}
