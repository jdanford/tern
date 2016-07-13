use std::fs::File;
use std::io;
use std::io::prelude::*;

use program::parser::*;
use util::next_aligned_addr;

#[derive(Clone, Debug)]
pub enum ReadMode {
	Code,
	Data,
}

#[derive(Clone, Debug)]
pub struct DecodedProgram {
	read_mode: ReadMode,
	pub code: Vec<CodeDecl>,
	pub data: Vec<DataDecl>,
}

impl DecodedProgram {
	pub fn new() -> DecodedProgram {
		DecodedProgram {
			read_mode: ReadMode::Code,
			code: Vec::new(),
			data: Vec::new(),
		}
	}

	pub fn code_size(&self) -> usize {
		self.code.iter().map(|d| match *d {
			CodeDecl::Instruction(ref inst) => inst.size(),
			_ => 0
		}).sum()
	}

	pub fn data_size(&self) -> usize {
		let len = self.data.len();
		let mut addr = 0;

		for i in 0..len {
			match self.data[i] {
				DataDecl::Data(ref data) => {
					addr += data.size();

					if i < len - 1 {
						match self.data[i + 1] {
							DataDecl::Data(ref next_data) => {
								addr = next_aligned_addr(addr, next_data.alignment());
							}

							_ => {}
						}
					} else {
						return addr + data.size();
					}
				}

				_ => {}
			}
		}

		return 0;
	}

	pub fn size(&self) -> usize {
		self.code_size() + self.data_size()
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
			match line {
				".code" => {
					self.read_mode = ReadMode::Code;
				}

				".data" => {
					self.read_mode = ReadMode::Data;
				}

				_ => match self.read_mode {
					ReadMode::Data => {
						self.data.push(try!(parse_data_line(line)));
					}

					ReadMode::Code => {
						self.code.push(try!(parse_code_line(line)));
					}
				}
			}
		}

		Ok(())
	}

	pub fn debug(&self) {
		self.print_data();
		println!("");
		self.print_code();
	}

	fn print_data(&self) {
		println!(".data");
		for data_decl in self.data.iter().cloned() {
			match data_decl {
				DataDecl::Label(label) => {
					println!("{}:", label);
				}

				DataDecl::Data(data) => {
					println!("  {:?}", data);
				}
			}
		}
	}

	fn print_code(&self) {
		println!(".code");
		for code_decl in self.code.iter().cloned() {
			match code_decl {
				CodeDecl::Label(label) => {
					println!("{}:", label);
				}

				CodeDecl::Instruction(instruction) => {
					println!("  {:?}", instruction);
				}
			}
		}
	}
}
