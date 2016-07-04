use combine;
use combine::{Parser};

use trit::Trit;
use registers::Register;
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
	assert_eq!(parser.parse("+12345"), Ok((12345, "")));
	assert_eq!(parser.parse("-12345"), Ok((-12345, "")));
}

#[test]
fn parser_int_register() {
	let mut parser = combine::parser(parser::register);
	assert_eq!(parser.parse("$0"), Ok((Register::ZERO, "")));
	assert_eq!(parser.parse("$1"), Ok((Register::RA, "")));
	assert_eq!(parser.parse("$2"), Ok((Register::LO, "")));
	assert_eq!(parser.parse("$3"), Ok((Register::HI, "")));
	assert_eq!(parser.parse("$4"), Ok((Register::SP, "")));
	assert_eq!(parser.parse("$5"), Ok((Register::FP, "")));
	assert_eq!(parser.parse("$6"), Ok((Register::A0, "")));
	assert_eq!(parser.parse("$7"), Ok((Register::A1, "")));
	assert_eq!(parser.parse("$8"), Ok((Register::A2, "")));
	assert_eq!(parser.parse("$9"), Ok((Register::A3, "")));
	assert_eq!(parser.parse("$10"), Ok((Register::A4, "")));
	assert_eq!(parser.parse("$11"), Ok((Register::A5, "")));
	assert_eq!(parser.parse("$12"), Ok((Register::T0, "")));
	assert_eq!(parser.parse("$13"), Ok((Register::T1, "")));
	assert_eq!(parser.parse("$14"), Ok((Register::T2, "")));
	assert_eq!(parser.parse("$15"), Ok((Register::T3, "")));
	assert_eq!(parser.parse("$16"), Ok((Register::T4, "")));
	assert_eq!(parser.parse("$17"), Ok((Register::T5, "")));
	assert_eq!(parser.parse("$18"), Ok((Register::S0, "")));
	assert_eq!(parser.parse("$19"), Ok((Register::S1, "")));
	assert_eq!(parser.parse("$20"), Ok((Register::S2, "")));
	assert_eq!(parser.parse("$21"), Ok((Register::S3, "")));
	assert_eq!(parser.parse("$22"), Ok((Register::S4, "")));
	assert_eq!(parser.parse("$23"), Ok((Register::S5, "")));
}

#[test]
fn parser_named_register() {
	let mut parser = combine::parser(parser::register);
	assert_eq!(parser.parse("$zero"), Ok((Register::ZERO, "")));
	assert_eq!(parser.parse("$ra"), Ok((Register::RA, "")));
	assert_eq!(parser.parse("$lo"), Ok((Register::LO, "")));
	assert_eq!(parser.parse("$hi"), Ok((Register::HI, "")));
	assert_eq!(parser.parse("$sp"), Ok((Register::SP, "")));
	assert_eq!(parser.parse("$fp"), Ok((Register::FP, "")));
	assert_eq!(parser.parse("$a0"), Ok((Register::A0, "")));
	assert_eq!(parser.parse("$a1"), Ok((Register::A1, "")));
	assert_eq!(parser.parse("$a2"), Ok((Register::A2, "")));
	assert_eq!(parser.parse("$a3"), Ok((Register::A3, "")));
	assert_eq!(parser.parse("$a4"), Ok((Register::A4, "")));
	assert_eq!(parser.parse("$a5"), Ok((Register::A5, "")));
	assert_eq!(parser.parse("$t0"), Ok((Register::T0, "")));
	assert_eq!(parser.parse("$t1"), Ok((Register::T1, "")));
	assert_eq!(parser.parse("$t2"), Ok((Register::T2, "")));
	assert_eq!(parser.parse("$t3"), Ok((Register::T3, "")));
	assert_eq!(parser.parse("$t4"), Ok((Register::T4, "")));
	assert_eq!(parser.parse("$t5"), Ok((Register::T5, "")));
	assert_eq!(parser.parse("$s0"), Ok((Register::S0, "")));
	assert_eq!(parser.parse("$s1"), Ok((Register::S1, "")));
	assert_eq!(parser.parse("$s2"), Ok((Register::S2, "")));
	assert_eq!(parser.parse("$s3"), Ok((Register::S3, "")));
	assert_eq!(parser.parse("$s4"), Ok((Register::S4, "")));
	assert_eq!(parser.parse("$s5"), Ok((Register::S5, "")));
}
