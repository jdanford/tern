use combine::{alpha_num, choice, char, digit, many1, optional, parser, spaces, string, try, Parser, ParserExt};
use combine::primitives::{State, Stream, ParseResult};

use trit::Trit;
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

parser_fn!(args_reg_reg -> (Register, Register) {
	spaces().with((
		parser(register),
		parser(comma),
		parser(register),
	)).map(|(r1, _, r2)| (r1, r2))
});

parser_fn!(inst_mov -> Instruction {
	string("mov").with(parser(args_reg_reg)).map(|(r1, r2)| Instruction::Mov(r1, r2))
});
