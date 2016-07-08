extern crate tern;

use std::env;

use tern::reader::Program;

fn main() {
	if let Some(path) = env::args().nth(1) {
		let mut program = Program::new();
		match program.read_file(&path[..]) {
			Ok(()) => {
				program.debug();
			}

			Err(e) => {
				println!("error: {:?}", e);
			}
		}
	} else {
		let program_name = env::args().nth(0).unwrap();
		println!("usage: {} <file>", program_name);
	}
}
