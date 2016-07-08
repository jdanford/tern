extern crate tern;

use std::env;
use std::io;

use tern::trit::Trit;
use tern::ternary;
use tern::types::*;

macro_rules! ptr {
	($e:expr) => (&$e[0] as *const _)
}

macro_rules! mut_ptr {
	($e:expr) => (&mut $e[0] as *mut _)
}

fn main() {
	let arg = env::args().nth(1).unwrap_or_default();
	let is_ternary_mode = arg == "--ternary";

	loop {
		let mut trits = [Trit::Zero; WORD_SIZE];
		let mut input = String::new();

		match io::stdin().read_line(&mut input) {
			Ok(_) => {
				let s = input.trim();
				if s.len() == 0 {
					break;
				}

				if is_ternary_mode {
					unsafe {
						ternary::write_str(mut_ptr!(trits), &s[..]);
						println!("=> {}", ternary::read_int(ptr!(trits), WORD_ISIZE));
					}
				} else {
					let n = s.parse::<isize>().unwrap();

					unsafe {
						ternary::write_int(mut_ptr!(trits), n, WORD_ISIZE);
						print!("=> ");
						ternary::print(ptr!(trits), io::stdout(), WORD_ISIZE);
					}
				}
			}

			Err(error) => println!("error: {}", error),
		}
	}
}
