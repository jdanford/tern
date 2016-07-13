use vm::VM;
use program::Program;
use encoder::Encoder;

pub fn vm_from_code(code: &str) -> Result<VM, String> {
	let mut program = Program::new();
	try!(program.read_str(code).map_err(|e| format!("{:?}", e)));

	let vm = VM::new(program.pc);

	let mut encoder = Encoder::new(vm.memory, vm.memory_size);
	try!(encoder.encode(program).map_err(|e| format!("{:?}", e)));

	Ok(vm)
}
