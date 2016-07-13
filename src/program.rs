use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use types::*;
use instructions::Instruction;
use parser::*;

#[derive(Debug)]
pub struct Program {
	pub pc: usize,
	pub instructions: Vec<Instruction>,
	pub labels: HashMap<String, Addr>,
}

impl Program {
	pub fn new() -> Program {
		Program {
			pc: 0,
			labels: HashMap::new(),
			instructions: Vec::new(),
		}
	}

	pub fn read_file<'a>(&mut self, path: &'a str) -> Result<(), ParseError> {
		let file = try!(File::open(path).map_err(ParseError::IOError));
		self.read(file)
	}

	pub fn read<R: Read>(&mut self, reader: R) -> Result<(), ParseError> {
		let buffer = io::BufReader::new(reader);
		for line_result in buffer.lines() {
			let raw_line = try!(line_result.map_err(ParseError::IOError));
			try!(self.read_line(&raw_line[..]));
		}

		Ok(())
	}

	pub fn read_str(&mut self, s: &str) -> Result<(), ParseError> {
		for raw_line in s.lines() {
			try!(self.read_line(raw_line));
		}

		Ok(())
	}

	pub fn read_line(&mut self, raw_line: &str) -> Result<(), ParseError> {
		let line = clean_line(raw_line);

		if !line.is_empty() {
			match try!(parse_line(line)) {
				ParsedLine::StaticData(_) => {}

				ParsedLine::Label(label) => {
					self.labels.insert(label, self.pc);
				}

				ParsedLine::Instruction(instruction) => {
					self.pc += instruction.size();
					self.instructions.push(instruction);
				}
			}
		}

		Ok(())
	}

	pub fn debug(&self) {
		let mut labels = self.labels.iter().collect::<Vec<_>>();
		labels.sort_by_key(|&(_, pc)| pc);

		let instructions = self.instructions.iter().cloned();

		let mut pc = 0;
		for instruction in instructions {
			if let Some(&(label, &label_pc)) = labels.get(0) {
				if label_pc == pc {
					println!("{} {}:", pc, label);
					labels.remove(0);
				}
			}

			println!("{} {:?}", pc, instruction);
			pc += instruction.size();
		}
	}
}
