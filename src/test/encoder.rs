use registers::Register;
use util::*;

#[test]
fn encoder_mov() {
	let code = r#"
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
