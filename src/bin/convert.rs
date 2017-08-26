extern crate tern;

use std::io;
use tern::ternary;
use tern::types::*;

fn main() {
    loop {
        let trits = &mut EMPTY_WORD;
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let s = input.trim();
                if s.is_empty() {
                    break;
                }

                if s.starts_with("0t") {
                    ternary::from_str(trits, &s);
                    println!("=> {}", ternary::to_int(trits));
                } else {
                    let n = s.parse::<isize>().unwrap();

                    ternary::from_int(trits, n);
                    println!("=> {}", ternary::to_str(trits));
                }
            }

            Err(error) => println!("error: {}", error),
        }
    }
}
