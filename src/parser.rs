use combine::{choice, char, digit, many1, parser, string, Parser, ParserExt};
use combine::primitives::{State, Stream, ParseResult};

use trit::Trit;

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

// pub fn number_sign<I>(input: State<I>) -> ParseResult<isize, I> where I: Stream<Item=char> {
//
// }

pub fn decimal_literal<I>(input: State<I>) -> ParseResult<isize, I> where I: Stream<Item=char> {
	many1::<Vec<_>, _>(digit()).map(isize_from_digits).parse_state(input)
}
