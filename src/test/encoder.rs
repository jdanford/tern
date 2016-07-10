use registers::Register;
use vm::VM;
use program::Program;
use encoder::Encoder;

fn vm_from_code(code: &str) -> Result<VM, String> {
	let mut program = Program::new();
	try!(program.read_str(code).map_err(|e| format!("{:?}", e)));

	let vm = VM::new(program.pc);

	let mut encoder = Encoder::new(vm.memory, vm.memory_size);
	try!(encoder.encode(program).map_err(|e| format!("{:?}", e)));

	Ok(vm)
}

#[test]
fn encoder_mov() {
	let code = r#"
	start:
		movi $a0, 100
		jmp end

	garbage:
		movi $a0, 456

	end:
		addi $a0, $a0, 23
		halt
	"#;

	match vm_from_code(code) {
		Ok(mut vm) => {
			vm.run();
			assert_eq!(vm.read(Register::A0), 123);
		}

		Err(e) => {
			println!("{}", e);
			assert!(false);
		}
	}
}
