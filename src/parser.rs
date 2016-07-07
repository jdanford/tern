use combine::*;
use combine::primitives::Stream;

use trit::Trit;
use types::*;
use ternary;
use opcodes::Opcode;
use registers::Register;
use instructions::Instruction;

fn isize_from_digit(c: char) -> isize {
	c.to_string().parse::<isize>().unwrap()
}

fn isize_from_digits<I>(iterable: I) -> isize where I: IntoIterator<Item=char> {
	iterable.into_iter().fold(0, |acc, c| acc * 10 + isize_from_digit(c))
}

macro_rules! parser_fn {
	($name:ident -> $ty:ty $block:block) => {
		pub fn $name<I>(input: State<I>) -> ParseResult<$ty, I> where I: Stream<Item=char> {
			let mut parser = $block;
			parser.parse_state(input)
		}
	}
}

parser_fn!(trit -> Trit {
	choice([char('T'), char('0'), char('1')]).map(Trit::from)
});

parser_fn!(trits -> Vec<Trit> {
	many1(parser(trit))
});

parser_fn!(ternary_literal -> Vec<Trit> {
	(string("0t"), parser(trits)).map(|(_, t)| t.into_iter().rev().collect())
});

parser_fn!(tryte -> Tryte {
	(string("0t"), parser(trits)).map(|(_, t)| {
		let mut tryte = EMPTY_TRYTE;
		unsafe { ternary::copy_from_iter(mut_ptr!(tryte), t.into_iter().rev()) };
		tryte
	})
});

parser_fn!(halfword -> Halfword {
	(string("0t"), parser(trits)).map(|(_, t)| {
		let mut halfword = EMPTY_HALFWORD;
		unsafe { ternary::copy_from_iter(mut_ptr!(halfword), t.into_iter().rev()) };
		halfword
	})
});

parser_fn!(word -> Word {
	(string("0t"), parser(trits)).map(|(_, t)| {
		let mut word = EMPTY_WORD;
		unsafe { ternary::copy_from_iter(mut_ptr!(word), t.into_iter().rev()) };
		word
	})
});

parser_fn!(number_sign -> isize {
	let raw_sign = choice([char('+'), char('-')]).map(|c| match c {
		'+' => 1,
		_ => -1
	});

	optional(raw_sign).map(|s| s.unwrap_or(1))
});

parser_fn!(decimal_value -> isize {
	many1::<Vec<_>, _>(digit()).map(isize_from_digits)
});

parser_fn!(decimal_literal -> isize {
	(parser(number_sign), parser(decimal_value)).map(|(s, n)| s * n)
});

parser_fn!(alpha_num_string -> String {
	many1(alpha_num())
});

parser_fn!(any_string -> String {
	many1(satisfy(|c| c != '\n'))
});

parser_fn!(label -> String {
	many1(try(char('_')).or(alpha_num()))
});

parser_fn!(register -> Register {
	let int_register = parser(decimal_value).map(Register::from);
	let named_register = parser(alpha_num_string).map(|s| Register::from(&s[..]));
	let register = try(int_register).or(try(named_register));
	char('$').with(register)
});

parser_fn!(opcode -> Opcode {
	parser(alpha_num_string).map(|s| Opcode::from(&s[..]))
});

parser_fn!(comma -> () {
	(spaces(), char(','), spaces()).map(|_| ())
});

parser_fn!(args_reg -> Register {
	spaces().with(parser(register))
});

parser_fn!(args_label -> String {
	spaces().with(parser(label))
});

parser_fn!(args_reg_reg -> (Register, Register) {
	spaces().with((
		parser(register),
		parser(comma),
		parser(register),
	)).map(|(r1, _, r2)| (r1, r2))
});

parser_fn!(args_reg_half -> (Register, Halfword) {
	spaces().with((
		parser(register),
		parser(comma),
		parser(halfword),
	)).map(|(r, _, h)| (r, h))
});

parser_fn!(args_reg_word -> (Register, Word) {
	spaces().with((
		parser(register),
		parser(comma),
		parser(word),
	)).map(|(r, _, w)| (r, w))
});

parser_fn!(args_reg_label -> (Register, String) {
	spaces().with((
		parser(register),
		parser(comma),
		parser(label),
	)).map(|(r, _, l)| (r, l))
});

parser_fn!(args_reg_reg_reg -> (Register, Register, Register) {
	spaces().with((
		parser(register),
		parser(comma),
		parser(register),
		parser(comma),
		parser(register),
	)).map(|(r1, _, r2, _, r3)| (r1, r2, r3))
});

parser_fn!(args_reg_reg_tryte -> (Register, Register, Tryte) {
	spaces().with((
		parser(register),
		parser(comma),
		parser(register),
		parser(comma),
		parser(tryte),
	)).map(|(r1, _, r2, _, t)| (r1, r2, t))
});

parser_fn!(inst_mov -> Instruction {
	string("mov").with(parser(args_reg_reg)).map(|(r1, r2)| Instruction::Mov(r1, r2))
});

parser_fn!(inst_movi -> Instruction {
	string("movi").with(parser(args_reg_half)).map(|(r, h)| Instruction::Movi(r, h))
});

parser_fn!(inst_add -> Instruction {
	string("add").with(parser(args_reg_reg_reg)).map(|(r1, r2, r3)| Instruction::Add(r1, r2, r3))
});

parser_fn!(inst_addi -> Instruction {
	string("addi").with(parser(args_reg_half)).map(|(r, h)| Instruction::Addi(r, h))
});

parser_fn!(inst_cmp -> Instruction {
	string("cmp").with(parser(args_reg_reg_reg)).map(|(r1, r2, r3)| Instruction::Cmp(r1, r2, r3))
});

parser_fn!(inst_jmp -> Instruction {
	string("jmp").with(parser(args_label)).map(|l| Instruction::Jmp(l))
});

parser_fn!(inst_j01 -> Instruction {
	string("j01").with(parser(args_reg_label)).map(|(r, l)| Instruction::J01(r, l))
});

parser_fn!(inst_halt -> Instruction {
	string("halt").map(|_| Instruction::Halt)
});

parser_fn!(instruction -> Instruction {
	try(parser(inst_mov))
	.or(try(parser(inst_movi)))
	.or(try(parser(inst_add)))
	.or(try(parser(inst_addi)))
	.or(try(parser(inst_cmp)))
	.or(try(parser(inst_jmp)))
	.or(try(parser(inst_j01)))
	.or(try(parser(inst_halt)))
});

parser_fn!(comment -> () {
	spaces().skip(char(';')).skip(parser(any_string))
});

#[derive(Debug)]
pub enum Line {
	Label(Label),
	Instruction(Instruction),
}

parser_fn!(line -> Line {
	let line_instruction = parser(instruction).map(Line::Instruction);
	let line_label = parser(label).skip(char(':')).map(Line::Label);
	try(line_instruction).or(try(line_label))
});

// parser_fn!(program -> Vec<Line> {
// 	let full_line = spaces().with(parser(line));
// 	let full_comment = parser(comment).skip(skip_many1(newline()));
// 	let separator = skip_many1(newline()).skip(optional(full_comment));
// 	sep_by(full_line, separator).skip(optional(newline()))
// });
