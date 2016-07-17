extern crate tern;

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
    loop {
        let mut trits = [Trit::Zero; WORD_SIZE];
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let s = input.trim();
                if s.len() == 0 {
                    break;
                }

                if s.starts_with("0t") {
                    unsafe {
                        ternary::from_str(mut_ptr!(trits), &s[..]);
                        println!("=> {}", ternary::to_int(ptr!(trits), WORD_ISIZE));
                    }
                } else {
                    let n = s.parse::<isize>().unwrap();

                    unsafe {
                        ternary::from_int(mut_ptr!(trits), n, WORD_ISIZE);
                        println!("=> {}", ternary::to_str(ptr!(trits), WORD_ISIZE));
                    }
                }
            }

            Err(error) => println!("error: {}", error),
        }
    }
}
