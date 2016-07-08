use regex;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use ternary;
use types::*;
use registers::Register;
use instructions::Instruction;

mod patterns {
	pub static LABEL: &'static str = r"([_a-z][_a-z0-9]*):";
	pub static INSTRUCTION: &'static str = r"([a-z][a-z0-9]*)(\s+(.*))?";
	pub static COMMA: &'static str = r",\s*";
	pub static TERNARY: &'static str = r"0t([10T]+)";
}

#[derive(Debug)]
pub struct Program {
	pc: usize,
	instructions: Vec<Instruction>,
	labels: HashMap<Label, Addr>,
}

fn clean_line<'a>(raw_line: &'a str) -> &'a str {
	let line = raw_line.trim();
	let mut end = line.len();
	for (i, c) in line.chars().enumerate() {
		if c == ';' {
			end = i;
			break;
		}
	}

	&line[..end].trim()
}

#[derive(Debug)]
enum ParsedLine {
	Label(Label),
	Instruction(Instruction),
}

#[derive(Debug)]
pub enum ParseError {
	Unknown,
	InvalidLabel(String),
	InvalidOpcode(String),
	InvalidTernary(String, usize),
	InvalidRegister(String),
	RegexError(regex::Error),
	IOError(io::Error),
}

fn parse_line<'a>(line: &'a str) -> Result<ParsedLine, ParseError> {
	if line.chars().rev().next().unwrap() == ':' {
		let label_re = try!(Regex::new(patterns::LABEL).map_err(ParseError::RegexError));
		let captures = try!(label_re.captures(line).ok_or(ParseError::Unknown));
		if let Some(label) = captures.iter().nth(1).unwrap() {
			println!("label: {}", label);
			Ok(ParsedLine::Label(label.to_string()))
		} else {
			let label = &line[..line.len() - 1];
			Err(ParseError::InvalidLabel(label.to_string()))
		}
	} else {
		parse_instruction(line).map(ParsedLine::Instruction)
	}
}

fn parse_instruction<'a>(line: &'a str) -> Result<Instruction, ParseError> {
	let instruction_re = try!(Regex::new(patterns::INSTRUCTION).map_err(ParseError::RegexError));
	let captures = try!(instruction_re.captures(line).ok_or(ParseError::Unknown));
	let captures = captures.iter().collect::<Vec<_>>();

	let opcode_name = try!(captures[1].ok_or(ParseError::Unknown));
	let args = if let Some(args_str) = captures[3] {
		let comma_re = try!(Regex::new(patterns::COMMA).map_err(ParseError::RegexError));
		comma_re.split(args_str).collect()
	} else {
		Vec::new()
	};

	instruction_from_parts(opcode_name, &args[..])
}

fn parse_label(s: &str) -> Result<String, ParseError> {
	Ok(s.to_string())
}

fn parse_register(s: &str) -> Result<Register, ParseError> {
	s.parse().map_err(|name| ParseError::InvalidRegister(name))
}

fn parse_tryte(s: &str) -> Result<Tryte, ParseError> {
	let mut tryte = EMPTY_TRYTE;

	if let Ok(int) = s.parse() {
		assert!(TRYTE_MIN <= int && int <= TRYTE_MAX);
		unsafe { ternary::write_int(mut_ptr!(tryte), int, TRYTE_ISIZE) };
		return Ok(tryte)
	}

	let ternary_re = try!(Regex::new(patterns::TERNARY).map_err(ParseError::RegexError));
	let captures = try!(ternary_re.captures(s).ok_or(ParseError::Unknown));
	if let Some(trit_str) = captures.iter().nth(1).unwrap() {
		assert!(trit_str.len() <= TRYTE_SIZE);
		unsafe { ternary::write_str(mut_ptr!(tryte), trit_str) };
		Ok(tryte)
	} else {
		Err(ParseError::InvalidTernary(s.to_string(), TRYTE_SIZE))
	}
}

fn parse_halfword(s: &str) -> Result<Halfword, ParseError> {
	let mut halfword = EMPTY_HALFWORD;

	if let Ok(int) = s.parse() {
		assert!(HALFWORD_MIN <= int && int <= HALFWORD_MAX);
		unsafe { ternary::write_int(mut_ptr!(halfword), int, HALFWORD_ISIZE) };
		return Ok(halfword)
	}

	let ternary_re = try!(Regex::new(patterns::TERNARY).map_err(ParseError::RegexError));
	let captures = try!(ternary_re.captures(s).ok_or(ParseError::Unknown));
	if let Some(trit_str) = captures.iter().nth(1).unwrap() {
		assert!(trit_str.len() <= HALFWORD_SIZE);
		unsafe { ternary::write_str(mut_ptr!(halfword), trit_str) };
		Ok(halfword)
	} else {
		Err(ParseError::InvalidTernary(s.to_string(), HALFWORD_SIZE))
	}
}

fn parse_word(s: &str) -> Result<Word, ParseError> {
	let mut word = EMPTY_WORD;

	if let Ok(int) = s.parse() {
		assert!(WORD_MIN <= int && int <= WORD_MAX);
		unsafe { ternary::write_int(mut_ptr!(word), int, WORD_ISIZE) };
		return Ok(word)
	}

	let ternary_re = try!(Regex::new(patterns::TERNARY).map_err(ParseError::RegexError));
	let captures = try!(ternary_re.captures(s).ok_or(ParseError::Unknown));
	if let Some(trit_str) = captures.iter().nth(1).unwrap() {
		assert!(trit_str.len() <= WORD_SIZE);
		unsafe { ternary::write_str(mut_ptr!(word), trit_str) };
		Ok(word)
	} else {
		Err(ParseError::InvalidTernary(s.to_string(), WORD_SIZE))
	}
}

fn instruction_from_parts<'a>(opcode_name: &'a str, args: &[&'a str]) -> Result<Instruction, ParseError> {
	let arity = args.len();

	match opcode_name {
		"mov" => {
			assert_eq!(arity, 2);
			Ok(Instruction::Mov(try!(parse_register(args[0])), try!(parse_register(args[1]))))
		}

		"movi" => {
			assert_eq!(arity, 2);
			Ok(Instruction::Movi(try!(parse_register(args[0])), try!(parse_halfword(args[1]))))
		}

		"movw" => {
			assert_eq!(arity, 2);
			Ok(Instruction::Movw(try!(parse_register(args[0])), try!(parse_word(args[1]))))
		}

		"lb" => {
			assert_eq!(arity, 3);
			Ok(Instruction::Lb(try!(parse_register(args[0])), try!(parse_register(args[1])), try!(parse_tryte(args[2]))))
		}

		"lh" => {
			assert_eq!(arity, 3);
			Ok(Instruction::Lh(try!(parse_register(args[0])), try!(parse_register(args[1])), try!(parse_tryte(args[2]))))
		}

		"lw" => {
			assert_eq!(arity, 3);
			Ok(Instruction::Lw(try!(parse_register(args[0])), try!(parse_register(args[1])), try!(parse_tryte(args[2]))))
		}

		"sb" => {
			assert_eq!(arity, 3);
			Ok(Instruction::Sb(try!(parse_register(args[0])), try!(parse_register(args[1])), try!(parse_tryte(args[2]))))
		}

		"sh" => {
			assert_eq!(arity, 3);
			Ok(Instruction::Sh(try!(parse_register(args[0])), try!(parse_register(args[1])), try!(parse_tryte(args[2]))))
		}

		"sw" => {
			assert_eq!(arity, 3);
			Ok(Instruction::Sw(try!(parse_register(args[0])), try!(parse_register(args[1])), try!(parse_tryte(args[2]))))
		}

		"add" => {
			assert_eq!(arity, 3);
			Ok(Instruction::Add(try!(parse_register(args[0])), try!(parse_register(args[1])), try!(parse_register(args[2]))))
		}

		"addi" => {
			assert_eq!(arity, 2);
			Ok(Instruction::Addi(try!(parse_register(args[0])), try!(parse_halfword(args[1]))))
		}

		"mul" => {
			assert_eq!(arity, 2);
			Ok(Instruction::Mul(try!(parse_register(args[0])), try!(parse_register(args[1]))))
		}

		"muli" => {
			assert_eq!(arity, 2);
			Ok(Instruction::Muli(try!(parse_register(args[0])), try!(parse_halfword(args[1]))))
		}

		"not" => {
			assert_eq!(arity, 2);
			Ok(Instruction::Not(try!(parse_register(args[0])), try!(parse_register(args[1]))))
		}

		"and" => {
			assert_eq!(arity, 3);
			Ok(Instruction::And(try!(parse_register(args[0])), try!(parse_register(args[1])), try!(parse_register(args[2]))))
		}

		"andi" => {
			assert_eq!(arity, 2);
			Ok(Instruction::Andi(try!(parse_register(args[0])), try!(parse_halfword(args[1]))))
		}

		"or" => {
			assert_eq!(arity, 3);
			Ok(Instruction::Or(try!(parse_register(args[0])), try!(parse_register(args[1])), try!(parse_register(args[2]))))
		}

		"ori" => {
			assert_eq!(arity, 2);
			Ok(Instruction::Ori(try!(parse_register(args[0])), try!(parse_halfword(args[1]))))
		}

		"shf" => {
			assert_eq!(arity, 3);
			Ok(Instruction::Shf(try!(parse_register(args[0])), try!(parse_register(args[1])), try!(parse_register(args[2]))))
		}

		"shfi" => {
			assert_eq!(arity, 2);
			Ok(Instruction::Shfi(try!(parse_register(args[0])), try!(parse_halfword(args[1]))))
		}

		"cmp" => {
			assert_eq!(arity, 3);
			Ok(Instruction::Cmp(try!(parse_register(args[0])), try!(parse_register(args[1])), try!(parse_register(args[2]))))
		}

		"jmp" => {
			assert_eq!(arity, 1);
			Ok(Instruction::Jmp(try!(parse_label(args[0]))))
		}

		"jr" => {
			assert_eq!(arity, 1);
			Ok(Instruction::Jr(try!(parse_register(args[0]))))
		}

		"jT" => {
			assert_eq!(arity, 2);
			Ok(Instruction::JT(try!(parse_register(args[0])), try!(parse_label(args[1]))))
		}

		"j0" => {
			assert_eq!(arity, 2);
			Ok(Instruction::J0(try!(parse_register(args[0])), try!(parse_label(args[1]))))
		}

		"j1" => {
			assert_eq!(arity, 2);
			Ok(Instruction::J1(try!(parse_register(args[0])), try!(parse_label(args[1]))))
		}

		"jT0" => {
			assert_eq!(arity, 2);
			Ok(Instruction::JT0(try!(parse_register(args[0])), try!(parse_label(args[1]))))
		}

		"jT1" => {
			assert_eq!(arity, 2);
			Ok(Instruction::JT1(try!(parse_register(args[0])), try!(parse_label(args[1]))))
		}

		"j01" => {
			assert_eq!(arity, 2);
			Ok(Instruction::J01(try!(parse_register(args[0])), try!(parse_label(args[1]))))
		}

		"call" => {
			assert_eq!(arity, 1);
			Ok(Instruction::Call(try!(parse_label(args[0]))))
		}

		"callr" => {
			assert_eq!(arity, 1);
			Ok(Instruction::Callr(try!(parse_register(args[0]))))
		}

		"ret" => {
			assert_eq!(arity, 0);
			Ok(Instruction::Ret)
		}

		"sys" => {
			assert_eq!(arity, 0);
			Ok(Instruction::Sys)
		}

		"break" => {
			assert_eq!(arity, 0);
			Ok(Instruction::Break)
		}

		"halt" => {
			assert_eq!(arity, 0);
			Ok(Instruction::Halt)
		}

		_ => Err(ParseError::InvalidOpcode(opcode_name.to_string()))
	}
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
			let line = clean_line(&raw_line[..]);

			if line.is_empty() {
				continue;
			}

			match try!(parse_line(line)) {
				ParsedLine::Label(label) => {
					self.labels.insert(label, self.pc);
				}

				ParsedLine::Instruction(instruction) => {
					self.pc += instruction.size();
					self.instructions.push(instruction);
				}
			}
		}

		self.debug();

		Ok(())
	}

	fn debug(&self) {
		let mut labels = self.labels.iter().collect::<Vec<_>>();
		labels.sort_by_key(|&(_, pc)| pc);

		let instructions = self.instructions.iter().cloned();

		let mut pc = 0;
		for instruction in instructions {
			if let Some(&(label, &label_pc)) = labels.get(0) {
				if label_pc == pc {
					println!("{}:", label);
					labels.remove(0);
				}
			}

			println!("{:02} {:?}", pc, instruction);
			pc += instruction.size();
		}
	}
}
