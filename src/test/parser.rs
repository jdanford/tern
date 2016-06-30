use combine;
use combine::{Parser};

use trit::Trit;
use parser;

#[test]
fn parser_trit() {
	let mut parser = combine::parser(parser::trit);

	assert_eq!(parser.parse("T"), Ok((Trit::Neg, "")));
	assert_eq!(parser.parse("0"), Ok((Trit::Zero, "")));
	assert_eq!(parser.parse("1"), Ok((Trit::Pos, "")));

	assert!(parser.parse("t").is_err());
	assert!(parser.parse("2").is_err());
}

#[test]
fn parser_trits() {
	let mut parser = combine::parser(parser::trits);
	assert_eq!(parser.parse("T01"), Ok((vec![Trit::Neg, Trit::Zero, Trit::Pos], "")));
}

#[test]
fn parser_ternary_literal() {
	let mut parser = combine::parser(parser::ternary_literal);
	assert_eq!(parser.parse("0tT01"), Ok((vec![Trit::Pos, Trit::Zero, Trit::Neg], "")));
}

#[test]
fn parser_decimal_literal() {
	let mut parser = combine::parser(parser::decimal_literal);
	assert_eq!(parser.parse("12345"), Ok((12345, "")));
}
