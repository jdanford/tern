extern crate tern;

use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use tern::program::DecodedProgram;

fn main() {
    if let Some(path) = env::args().nth(1) {
        let reader: Box<Read> = match &path[..] {
            "-" => Box::new(io::stdin()),
            _ => Box::new(File::open(path).unwrap()),
        };

        let mut program = DecodedProgram::new();
        match program.read(reader) {
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
