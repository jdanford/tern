use regex;
use std::io;

use ternary;
use types::*;
use registers::Register;
use program::instructions::Instruction;
use program::data::StaticData;

mod patterns {
	pub static COMMA: &'static str = r",\s*";
	pub static TERNARY: &'static str = r"0t([10T]+)";
	pub static LABEL: &'static str = r"([_a-z][_a-z0-9]*):";
	pub static STATEMENT: &'static str = r"([_a-z][_a-z0-9]*)(\s+(.*))?";
	pub static STRING: &'static str = r#"^\s*"((?:\\"|[^"])+)"\s*$"#;
}

#[derive(Clone, Debug)]
pub enum CodeDecl {
	Label(String),
	Instruction(Instruction),
}

impl CodeDecl {
	pub fn size(&self) -> usize {
		match *self {
			CodeDecl::Instruction(ref inst) => inst.size(),
			_ => 0
		}
	}
}

#[derive(Clone, Debug)]
pub enum DataDecl {
	Label(String),
	Data(StaticData),
}

impl DataDecl {
	pub fn size(&self) -> usize {
		match *self {
			DataDecl::Data(ref data) => data.size(),
			_ => 0
		}
	}
}

#[derive(Debug)]
pub enum ParseError {
	Unknown,
	RegexMatchFailure,
	RegexMissingCapture,
	InvalidCodeSection,
	InvalidDataSection,
	InvalidLabel(String),
	InvalidDataType(String),
	InvalidOpcode(String),
	InvalidTernary(String, usize),
	InvalidRegister(String),
	RegexError(regex::Error),
	IOError(io::Error),
}

pub fn clean_line<'a>(raw_line: &'a str) -> &'a str {
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

pub fn line_is_label(line: &str) -> bool {
	line.chars().rev().next().unwrap() == ':'
}

pub fn line_is_data(line: &str) -> bool {
	line.chars().next().unwrap() == '@'
}

pub fn parse_code_line(line: &str) -> Result<CodeDecl, ParseError> {
	if line_is_label(line) {
		parse_label_line(line).map(CodeDecl::Label)
	} else {
		parse_instruction(line).map(CodeDecl::Instruction)
	}
}

pub fn parse_data_line(line: &str) -> Result<DataDecl, ParseError> {
	if line_is_label(line) {
		parse_label_line(line).map(DataDecl::Label)
	} else if line_is_data(line) {
		parse_data(&line[1..]).map(DataDecl::Data)
	} else {
		Err(ParseError::InvalidDataSection)
	}
}

fn compile_regex(pattern: &str) -> Result<regex::Regex, ParseError> {
	regex::Regex::new(pattern).map_err(ParseError::RegexError)
}

fn get_capture<'a>(captures: &regex::Captures<'a>, i: usize) -> Result<&'a str, ParseError> {
	captures.at(i).ok_or(ParseError::RegexMissingCapture)
}

fn with_regex_captures<T, F>(pattern: &str, s: &str, mut f: F) -> Result<T, ParseError>
		where F: FnMut(&regex::Captures) -> Result<T, ParseError> {
	let re = try!(compile_regex(pattern));
	let captures = try!(re.captures(s).ok_or(ParseError::RegexMatchFailure));
	f(&captures)
}

pub fn parse_label_line(line: &str) -> Result<String, ParseError> {
	with_regex_captures(patterns::LABEL, line, |ref captures| {
		if let Some(label) = captures.at(1) {
			Ok(label.to_string())
		} else {
			let label = &line[..line.len() - 1];
			Err(ParseError::InvalidLabel(label.to_string()))
		}
	})
}

fn parse_data(line: &str) -> Result<StaticData, ParseError> {
	with_regex_captures(patterns::STATEMENT, line, |ref captures| {
		let type_name = try!(get_capture(captures, 1));
		let rest = try!(get_capture(captures, 3));
		data_from_parts(type_name, rest)
	})
}

fn parse_instruction(line: &str) -> Result<Instruction, ParseError> {
	with_regex_captures(patterns::STATEMENT, line, |ref captures| {
		let opcode_name = try!(get_capture(captures, 1));
		let args = if let Some(args_str) = captures.at(3) {
			let comma_re = try!(compile_regex(patterns::COMMA));
			comma_re.split(args_str).collect()
		} else {
			Vec::new()
		};

		instruction_from_parts(opcode_name, &args[..])
	})
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
		unsafe { ternary::from_int(mut_ptr!(tryte), int, TRYTE_ISIZE) };
		return Ok(tryte)
	}

	with_regex_captures(patterns::TERNARY, s, |ref captures| {
		if let Some(trit_str) = captures.at(1) {
			assert!(trit_str.len() <= TRYTE_SIZE);
			unsafe { ternary::from_str(mut_ptr!(tryte), trit_str) };
			Ok(tryte)
		} else {
			Err(ParseError::InvalidTernary(s.to_string(), TRYTE_SIZE))
		}
	})
}

fn parse_half(s: &str) -> Result<Half, ParseError> {
	let mut half = EMPTY_HALF;

	if let Ok(int) = s.parse() {
		assert!(HALF_MIN <= int && int <= HALF_MAX);
		unsafe { ternary::from_int(mut_ptr!(half), int, HALF_ISIZE) };
		return Ok(half)
	}

	with_regex_captures(patterns::TERNARY, s, |ref captures| {
		if let Some(trit_str) = captures.at(1) {
			assert!(trit_str.len() <= HALF_SIZE);
			unsafe { ternary::from_str(mut_ptr!(half), trit_str) };
			Ok(half)
		} else {
			Err(ParseError::InvalidTernary(s.to_string(), HALF_SIZE))
		}
	})
}

fn parse_word(s: &str) -> Result<Word, ParseError> {
	let mut word = EMPTY_WORD;

	if let Ok(int) = s.parse() {
		assert!(WORD_MIN <= int && int <= WORD_MAX);
		unsafe { ternary::from_int(mut_ptr!(word), int, WORD_ISIZE) };
		return Ok(word)
	}

	with_regex_captures(patterns::TERNARY, s, |ref captures| {
		if let Some(trit_str) = captures.at(1) {
			assert!(trit_str.len() <= WORD_SIZE);
			unsafe { ternary::from_str(mut_ptr!(word), trit_str) };
			Ok(word)
		} else {
			Err(ParseError::InvalidTernary(s.to_string(), WORD_SIZE))
		}
	})
}

fn parse_string(s: &str) -> Result<String, ParseError> {
	with_regex_captures(patterns::STRING, s, |ref captures| {
		let string = try!(get_capture(captures, 1));
		Ok(string.to_string())
	})
}

fn data_from_parts<'a>(type_name: &'a str, rest: &'a str) -> Result<StaticData, ParseError> {
	match type_name {
		"tryte" => {
			let tryte = try!(parse_tryte(rest));
			let i = unsafe { ternary::to_int(ptr!(tryte), TRYTE_ISIZE) };
			Ok(StaticData::Tryte(i))
		}

		"half" => {
			let half = try!(parse_half(rest));
			let i = unsafe { ternary::to_int(ptr!(half), HALF_ISIZE) };
			Ok(StaticData::Half(i))
		}

		"word" => {
			let word = try!(parse_word(rest));
			let i = unsafe { ternary::to_int(ptr!(word), WORD_ISIZE) };
			Ok(StaticData::Word(i))
		}

		"string" => {
			let string = try!(parse_string(rest));
			Ok(StaticData::String(string))
		}

 		_ => Err(ParseError::InvalidDataType(type_name.to_string()))
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
			Ok(Instruction::Movi(try!(parse_register(args[0])), try!(parse_half(args[1]))))
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
			Ok(Instruction::Addi(try!(parse_register(args[0])), try!(parse_half(args[1]))))
		}

		"mul" => {
			assert_eq!(arity, 2);
			Ok(Instruction::Mul(try!(parse_register(args[0])), try!(parse_register(args[1]))))
		}

		"muli" => {
			assert_eq!(arity, 2);
			Ok(Instruction::Muli(try!(parse_register(args[0])), try!(parse_half(args[1]))))
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
			Ok(Instruction::Andi(try!(parse_register(args[0])), try!(parse_half(args[1]))))
		}

		"or" => {
			assert_eq!(arity, 3);
			Ok(Instruction::Or(try!(parse_register(args[0])), try!(parse_register(args[1])), try!(parse_register(args[2]))))
		}

		"ori" => {
			assert_eq!(arity, 2);
			Ok(Instruction::Ori(try!(parse_register(args[0])), try!(parse_half(args[1]))))
		}

		"shf" => {
			assert_eq!(arity, 3);
			Ok(Instruction::Shf(try!(parse_register(args[0])), try!(parse_register(args[1])), try!(parse_register(args[2]))))
		}

		"shfi" => {
			assert_eq!(arity, 2);
			Ok(Instruction::Shfi(try!(parse_register(args[0])), try!(parse_half(args[1]))))
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
