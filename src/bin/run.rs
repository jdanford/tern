extern crate tern;

use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use tern::util::vm_from_reader;

fn main() {
    if let Some(path) = env::args().nth(1) {
        let reader: Box<Read> = match &path[..] {
            "-" => Box::new(io::stdin()),
            _ => Box::new(File::open(path).unwrap()),
        };

        match vm_from_reader(reader) {
            Ok(mut vm) => {
                vm.run();
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
