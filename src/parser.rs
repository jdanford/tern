use combine::{any, choice, char, digit, many1, optional, parser, string, try, Parser, ParserExt};
use combine::primitives::{State, Stream, ParseResult};

use trit::Trit;
use registers::*;

pub fn trit<I>(input: State<I>) -> ParseResult<Trit, I> where I: Stream<Item=char> {
	choice([char('T'), char('0'), char('1')]).map(Trit::from).parse_state(input)
}

pub fn trits<I>(input: State<I>) -> ParseResult<Vec<Trit>, I> where I: Stream<Item=char> {
	many1(parser(trit)).parse_state(input)
}

pub fn ternary_literal<I>(input: State<I>) -> ParseResult<Vec<Trit>, I> where I: Stream<Item=char> {
	(string("0t"), parser(trits)).map(|t| t.1.into_iter().rev().collect()).parse_state(input)
}

fn isize_from_digit(c: char) -> isize {
	c.to_string().parse::<isize>().unwrap()
}

fn isize_from_digits<I>(iterable: I) -> isize where I: IntoIterator<Item=char> {
	iterable.into_iter().fold(0, |acc, c| acc * 10 + isize_from_digit(c))
}

pub fn number_sign<I>(input: State<I>) -> ParseResult<isize, I> where I: Stream<Item=char> {
	let raw_sign = choice([char('+'), char('-')]).map(|c| match c {
		'+' => 1,
		_ => -1
	});

	optional(raw_sign).map(|s| s.unwrap_or(1)).parse_state(input)
}

pub fn decimal_value<I>(input: State<I>) -> ParseResult<isize, I> where I: Stream<Item=char> {
	many1::<Vec<_>, _>(digit()).map(isize_from_digits).parse_state(input)
}

pub fn decimal_literal<I>(input: State<I>) -> ParseResult<isize, I> where I: Stream<Item=char> {
	(parser(number_sign), parser(decimal_value)).map(|(s, n)| s * n).parse_state(input)
}

pub fn any_string<I>(input: State<I>) -> ParseResult<String, I> where I: Stream<Item=char> {
	many1::<String, _>(any()).parse_state(input)
}

pub fn register<I>(input: State<I>) -> ParseResult<Register, I> where I: Stream<Item=char> {
	let int_register = parser(decimal_value).map(Register::from);
	let str_register = parser(any_string).map(|s| Register::from_str(&s[..]));
	let register = try(int_register).or(try(str_register));
	char('$').with(register).parse_state(input)
}
