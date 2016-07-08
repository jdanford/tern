// use trit::Trit;
// use opcodes::Opcode;
// use registers::Register;
// use instructions::Instruction;
use reader::Program;

#[test]
fn reader_from_file() {
	let mut program = Program::new();
	assert!(program.read_file("fibonacci.tasm").is_ok());
}
