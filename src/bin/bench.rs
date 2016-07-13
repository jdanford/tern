extern crate tern;

use std::env;

use tern::registers::Register;
use tern::util::*;

fn parse_and_run() {
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

const DEFAULT_ITERATIONS: usize = 1024;

fn main() {
	let iterations = match env::args().nth(1) {
		Some(arg_str) => arg_str.parse().unwrap_or(DEFAULT_ITERATIONS),
		_ => DEFAULT_ITERATIONS
	};

	for _ in 0..iterations {
		parse_and_run();
	}
}
