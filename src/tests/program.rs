use registers::Register;
use util::*;

#[test]
fn program_mov() {
	let code = r#"
	.data
	@tryte 1
	@tryte 2
	@tryte 3
	@string "Hello!"

	.code
	start:
		movi $a0, 103
		jmp end

	garbage:
		movi $a0, 456

	end:
		addi $a0, 20
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
