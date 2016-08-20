use regex;
use std::char;
use std::io;

use ternary;
use types::*;
use opcodes::Opcode;
use registers::Register;
use program::instructions::Instruction;
use program::data::StaticData;

mod patterns {
    pub static COMMA: &'static str = r",\s*";
    pub static TERNARY: &'static str = r"0t([10T]+)";
    pub static LABEL: &'static str = r"([_a-zA-Z][_a-zA-Z0-9]*):";
    pub static STATEMENT: &'static str = r"([_a-zA-Z][_a-zA-Z0-9]*)(\s+(.*))?";
    pub static STRING: &'static str = r#"^\s*"(.+)"\s*$"#;
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
            _ => 0,
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
            _ => 0,
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
    InvalidEscapeSequence(String),
    InvalidLabel(String),
    InvalidDataType(String),
    InvalidDataSpec(String),
    InvalidArity(String, usize, usize),
    InvalidOpcode(String),
    InvalidTernary(String, usize),
    InvalidDecimal(String),
    InvalidRegister(String),
    RegexError(regex::Error),
    IOError(io::Error),
}

pub type ParseResult<T> = Result<T, ParseError>;

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
    line.chars().next().unwrap() == '%'
}

pub fn parse_code_line(line: &str) -> ParseResult<CodeDecl> {
    if line_is_label(line) {
        parse_label_line(line).map(CodeDecl::Label)
    } else {
        parse_instruction(line).map(CodeDecl::Instruction)
    }
}

pub fn parse_data_line(line: &str) -> ParseResult<DataDecl> {
    if line_is_label(line) {
        parse_label_line(line).map(DataDecl::Label)
    } else if line_is_data(line) {
        parse_data(&line[1..]).map(DataDecl::Data)
    } else {
        Err(ParseError::InvalidDataSection)
    }
}

fn compile_regex(pattern: &str) -> ParseResult<regex::Regex> {
    regex::Regex::new(pattern).map_err(ParseError::RegexError)
}

fn get_capture<'a>(captures: &regex::Captures<'a>, i: usize) -> ParseResult<&'a str> {
    captures.at(i).ok_or(ParseError::RegexMissingCapture)
}

fn with_regex_captures<T, F>(pattern: &str, s: &str, mut f: F) -> ParseResult<T>
    where F: FnMut(&regex::Captures) -> ParseResult<T>
{
    let re = try!(compile_regex(pattern));
    let captures = try!(re.captures(s).ok_or(ParseError::RegexMatchFailure));
    f(&captures)
}

pub fn parse_label_line(line: &str) -> ParseResult<String> {
    with_regex_captures(patterns::LABEL, line, |ref captures| {
        if let Some(label) = captures.at(1) {
            Ok(label.to_string())
        } else {
            let label = &line[..line.len() - 1];
            Err(ParseError::InvalidLabel(label.to_string()))
        }
    })
}

fn parse_data(line: &str) -> ParseResult<StaticData> {
    with_regex_captures(patterns::STATEMENT, line, |ref captures| {
        let type_name = try!(get_capture(captures, 1));
        let rest = try!(get_capture(captures, 3));
        data_from_parts(type_name, rest)
    })
}

fn parse_instruction(line: &str) -> ParseResult<Instruction> {
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

fn parse_label(s: &str) -> ParseResult<String> {
    Ok(s.to_string())
}

fn parse_register(s: &str) -> ParseResult<Register> {
    s.parse().map_err(|name| ParseError::InvalidRegister(name))
}

fn parse_decimal(s: &str) -> ParseResult<isize> {
    s.parse().map_err(|_| ParseError::InvalidDecimal(s.to_string()))
}

fn parse_tryte(s: &str) -> ParseResult<Tryte> {
    let mut tryte = EMPTY_TRYTE;

    if let Ok(int) = s.parse() {
        assert!(TRYTE_MIN <= int && int <= TRYTE_MAX);
        unsafe { ternary::from_int(mut_ptr!(tryte), int, TRYTE_ISIZE) };
        return Ok(tryte);
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

fn parse_half(s: &str) -> ParseResult<Half> {
    let mut half = EMPTY_HALF;

    if let Ok(int) = s.parse() {
        assert!(HALF_MIN <= int && int <= HALF_MAX);
        unsafe { ternary::from_int(mut_ptr!(half), int, HALF_ISIZE) };
        return Ok(half);
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

fn parse_word(s: &str) -> ParseResult<Word> {
    let mut word = EMPTY_WORD;

    if let Ok(int) = s.parse() {
        assert!(WORD_MIN <= int && int <= WORD_MAX);
        unsafe { ternary::from_int(mut_ptr!(word), int, WORD_ISIZE) };
        return Ok(word);
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

fn parse_string(s: &str) -> ParseResult<String> {
    with_regex_captures(patterns::STRING, s, |ref captures| {
        let string = try!(get_capture(captures, 1));
        let unescaped_string = try!(unescape_string(string));
        Ok(unescaped_string)
    })
}

fn unescape_string(s: &str) -> ParseResult<String> {
    let mut result = String::new();
    let mut chars = s.chars();

    while let Some(c) = chars.next() {
        let unescaped = if c == '\\' {
            try!(unescape_chars(&mut chars))
        } else {
            c
        };

        result.push(unescaped);
    }

    Ok(result)
}

fn unescape_chars<I>(chars: &mut I) -> ParseResult<char> where I: Iterator<Item = char> {
    match chars.next() {
        Some('u') => {
            let seq: String = chars.take(4).collect();

            let mut code = 0;
            for c in seq.chars() {
                let n = try!(c.to_digit(16)
                    .ok_or_else(|| ParseError::InvalidEscapeSequence(seq.clone())));
                code = code * 16 + n;
            }

            char::from_u32(code).ok_or_else(|| ParseError::InvalidEscapeSequence(seq.clone()))
        }
        Some('b') => Ok('\x08'),
        Some('f') => Ok('\x0c'),
        Some('n') => Ok('\n'),
        Some('r') => Ok('\r'),
        Some('t') => Ok('\t'),
        Some('\\') => Ok('\\'),
        Some('"') => Ok('"'),
        _ => Err(ParseError::InvalidEscapeSequence("\\".to_string())),
    }
}

fn data_from_parts<'a>(type_name: &'a str, rest: &'a str) -> ParseResult<StaticData> {
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

        "array" => {
            let mut parts = rest.split(" x ").map(|s| s.trim());
            let data_str = try!(parts.next()
                .ok_or_else(|| ParseError::InvalidDataSpec(rest.to_string())));
            let count_str = try!(parts.next()
                .ok_or_else(|| ParseError::InvalidDataSpec(rest.to_string())));

            if let DataDecl::Data(data) = try!(parse_data_line(data_str)) {
                let count = try!(parse_decimal(count_str));
                assert!(count > 0);
                Ok(StaticData::Array(Box::new(data), count as usize))
            } else {
                Err(ParseError::InvalidDataSpec(data_str.to_string()))
            }
        }

        _ => Err(ParseError::InvalidDataType(type_name.to_string())),
    }
}

fn instruction_from_parts<'a>(opcode_name: &'a str, args: &[&'a str]) -> ParseResult<Instruction> {
    if !Opcode::name_is_valid(opcode_name) {
        return Err(ParseError::InvalidOpcode(opcode_name.to_string()));
    }

    let opcode = Opcode::from(opcode_name);

    let expected_arity = opcode.arity();
    let actual_arity = args.len();
    if expected_arity != actual_arity {
        return Err(ParseError::InvalidArity(opcode_name.to_string(), expected_arity, actual_arity));
    }

    match opcode {
        Opcode::Mov => {
            Ok(Instruction::Mov(try!(parse_register(args[0])), try!(parse_register(args[1]))))
        }

        Opcode::Movi => {
            Ok(Instruction::Movi(try!(parse_register(args[0])), try!(parse_half(args[1]))))
        }

        Opcode::Movw => {
            Ok(Instruction::Movw(try!(parse_register(args[0])), try!(parse_word(args[1]))))
        }

        Opcode::Mova => {
            Ok(Instruction::Mova(try!(parse_register(args[0])), try!(parse_label(args[1]))))
        }

        Opcode::Lt => {
            Ok(Instruction::Lt(try!(parse_register(args[0])),
                               try!(parse_register(args[1])),
                               try!(parse_tryte(args[2]))))
        }

        Opcode::Lh => {
            Ok(Instruction::Lh(try!(parse_register(args[0])),
                               try!(parse_register(args[1])),
                               try!(parse_tryte(args[2]))))
        }

        Opcode::Lw => {
            Ok(Instruction::Lw(try!(parse_register(args[0])),
                               try!(parse_register(args[1])),
                               try!(parse_tryte(args[2]))))
        }

        Opcode::St => {
            Ok(Instruction::St(try!(parse_register(args[0])),
                               try!(parse_register(args[1])),
                               try!(parse_tryte(args[2]))))
        }

        Opcode::Sh => {
            Ok(Instruction::Sh(try!(parse_register(args[0])),
                               try!(parse_register(args[1])),
                               try!(parse_tryte(args[2]))))
        }

        Opcode::Sw => {
            Ok(Instruction::Sw(try!(parse_register(args[0])),
                               try!(parse_register(args[1])),
                               try!(parse_tryte(args[2]))))
        }

        Opcode::Add => {
            Ok(Instruction::Add(try!(parse_register(args[0])),
                                try!(parse_register(args[1])),
                                try!(parse_register(args[2]))))
        }

        Opcode::Addi => {
            Ok(Instruction::Addi(try!(parse_register(args[0])), try!(parse_half(args[1]))))
        }

        Opcode::Mul => {
            Ok(Instruction::Mul(try!(parse_register(args[0])), try!(parse_register(args[1]))))
        }

        Opcode::Muli => {
            Ok(Instruction::Muli(try!(parse_register(args[0])), try!(parse_half(args[1]))))
        }

        Opcode::Not => {
            Ok(Instruction::Not(try!(parse_register(args[0])), try!(parse_register(args[1]))))
        }

        Opcode::And => {
            Ok(Instruction::And(try!(parse_register(args[0])),
                                try!(parse_register(args[1])),
                                try!(parse_register(args[2]))))
        }

        Opcode::Andi => {
            Ok(Instruction::Andi(try!(parse_register(args[0])), try!(parse_half(args[1]))))
        }

        Opcode::Or => {
            Ok(Instruction::Or(try!(parse_register(args[0])),
                               try!(parse_register(args[1])),
                               try!(parse_register(args[2]))))
        }

        Opcode::Ori => {
            Ok(Instruction::Ori(try!(parse_register(args[0])), try!(parse_half(args[1]))))
        }

        Opcode::Shf => {
            Ok(Instruction::Shf(try!(parse_register(args[0])),
                                try!(parse_register(args[1])),
                                try!(parse_register(args[2]))))
        }

        Opcode::Shfi => {
            Ok(Instruction::Shfi(try!(parse_register(args[0])), try!(parse_half(args[1]))))
        }

        Opcode::Cmp => {
            Ok(Instruction::Cmp(try!(parse_register(args[0])),
                                try!(parse_register(args[1])),
                                try!(parse_register(args[2]))))
        }

        Opcode::Jmp => Ok(Instruction::Jmp(try!(parse_label(args[0])))),

        Opcode::JT => {
            Ok(Instruction::JT(try!(parse_register(args[0])), try!(parse_label(args[1]))))
        }

        Opcode::J0 => {
            Ok(Instruction::J0(try!(parse_register(args[0])), try!(parse_label(args[1]))))
        }

        Opcode::J1 => {
            Ok(Instruction::J1(try!(parse_register(args[0])), try!(parse_label(args[1]))))
        }

        Opcode::JT0 => {
            Ok(Instruction::JT0(try!(parse_register(args[0])), try!(parse_label(args[1]))))
        }

        Opcode::JT1 => {
            Ok(Instruction::JT1(try!(parse_register(args[0])), try!(parse_label(args[1]))))
        }

        Opcode::J01 => {
            Ok(Instruction::J01(try!(parse_register(args[0])), try!(parse_label(args[1]))))
        }

        Opcode::Call => Ok(Instruction::Call(try!(parse_label(args[0])))),

        Opcode::Ret => Ok(Instruction::Ret),

        Opcode::Syscall => Ok(Instruction::Syscall),

        Opcode::Break => Ok(Instruction::Break),

        Opcode::Halt => Ok(Instruction::Halt),
    }
}
