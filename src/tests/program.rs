use registers::Register;
use util::*;

#[test]
fn program_mov() {
	let code = r#"
	.data
	trytes:
		@tryte 1
		@tryte 2
		@tryte 3
	message:
		@string "Hello!"

	.code
	start:
		movi $a0, 103
		jmp end

	garbage:
		movi $a0, 456

	end:
		addi $a0, 20
		mova $a1, message
		halt
	"#;

	match vm_from_code(code) {
		Ok(mut vm) => {
			vm.run();
			assert_eq!(vm.read(Register::A0), 123);
			assert_eq!(vm.read(Register::A1), 72);
		}

		Err(e) => {
			println!("{}", e);
			assert!(false);
		}
	}
}
