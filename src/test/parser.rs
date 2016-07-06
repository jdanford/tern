use combine;
use combine::{Parser};

use trit::Trit;
use opcodes::Opcode;
use registers::Register;
use instructions::Instruction;
use parser;

macro_rules! assert_parse {
	($parser: expr, $left: expr, $right: expr) => {
		assert_eq!($parser.parse($left), Ok(($right, "")));
	}
}

macro_rules! assert_parse_err {
	($parser: expr, $expr: expr) => {
		assert!($parser.parse($expr).is_err());
	}
}

#[test]
fn parser_trit() {
	let mut parser = combine::parser(parser::trit);
	assert_parse!(parser, "T", Trit::Neg);
	assert_parse!(parser, "0", Trit::Zero);
	assert_parse!(parser, "1", Trit::Pos);
	assert_parse_err!(parser, "t");
	assert_parse_err!(parser, "2");
}

#[test]
fn parser_trits() {
	let mut parser = combine::parser(parser::trits);
	assert_parse!(parser, "T01", vec![Trit::Neg, Trit::Zero, Trit::Pos]);
}

#[test]
fn parser_ternary_literal() {
	let mut parser = combine::parser(parser::ternary_literal);
	assert_parse!(parser, "0tT01", vec![Trit::Pos, Trit::Zero, Trit::Neg]);
}

#[test]
fn parser_decimal_literal() {
	let mut parser = combine::parser(parser::decimal_literal);
	assert_parse!(parser, "12345", 12345);
	assert_parse!(parser, "+12345", 12345);
	assert_parse!(parser, "-12345", -12345);
}

#[test]
fn parser_int_register() {
	let mut parser = combine::parser(parser::register);
	assert_parse!(parser, "$0", Register::ZERO);
	assert_parse!(parser, "$1", Register::RA);
	assert_parse!(parser, "$2", Register::LO);
	assert_parse!(parser, "$3", Register::HI);
	assert_parse!(parser, "$4", Register::SP);
	assert_parse!(parser, "$5", Register::FP);
	assert_parse!(parser, "$6", Register::A0);
	assert_parse!(parser, "$7", Register::A1);
	assert_parse!(parser, "$8", Register::A2);
	assert_parse!(parser, "$9", Register::A3);
	assert_parse!(parser, "$10", Register::A4);
	assert_parse!(parser, "$11", Register::A5);
	assert_parse!(parser, "$12", Register::T0);
	assert_parse!(parser, "$13", Register::T1);
	assert_parse!(parser, "$14", Register::T2);
	assert_parse!(parser, "$15", Register::T3);
	assert_parse!(parser, "$16", Register::T4);
	assert_parse!(parser, "$17", Register::T5);
	assert_parse!(parser, "$18", Register::S0);
	assert_parse!(parser, "$19", Register::S1);
	assert_parse!(parser, "$20", Register::S2);
	assert_parse!(parser, "$21", Register::S3);
	assert_parse!(parser, "$22", Register::S4);
	assert_parse!(parser, "$23", Register::S5);
}

#[test]
fn parser_named_register() {
	let mut parser = combine::parser(parser::register);
	assert_parse!(parser, "$zero", Register::ZERO);
	assert_parse!(parser, "$ra", Register::RA);
	assert_parse!(parser, "$lo", Register::LO);
	assert_parse!(parser, "$hi", Register::HI);
	assert_parse!(parser, "$sp", Register::SP);
	assert_parse!(parser, "$fp", Register::FP);
	assert_parse!(parser, "$a0", Register::A0);
	assert_parse!(parser, "$a1", Register::A1);
	assert_parse!(parser, "$a2", Register::A2);
	assert_parse!(parser, "$a3", Register::A3);
	assert_parse!(parser, "$a4", Register::A4);
	assert_parse!(parser, "$a5", Register::A5);
	assert_parse!(parser, "$t0", Register::T0);
	assert_parse!(parser, "$t1", Register::T1);
	assert_parse!(parser, "$t2", Register::T2);
	assert_parse!(parser, "$t3", Register::T3);
	assert_parse!(parser, "$t4", Register::T4);
	assert_parse!(parser, "$t5", Register::T5);
	assert_parse!(parser, "$s0", Register::S0);
	assert_parse!(parser, "$s1", Register::S1);
	assert_parse!(parser, "$s2", Register::S2);
	assert_parse!(parser, "$s3", Register::S3);
	assert_parse!(parser, "$s4", Register::S4);
	assert_parse!(parser, "$s5", Register::S5);
}

#[test]
fn parser_opcode() {
	let mut parser = combine::parser(parser::opcode);
	assert_parse!(parser, "mov", Opcode::Mov);
	assert_parse!(parser, "movi", Opcode::Movi);
	assert_parse!(parser, "movw", Opcode::Movw);
	assert_parse!(parser, "lb", Opcode::Lb);
	assert_parse!(parser, "lh", Opcode::Lh);
	assert_parse!(parser, "lw", Opcode::Lw);
	assert_parse!(parser, "sb", Opcode::Sb);
	assert_parse!(parser, "sh", Opcode::Sh);
	assert_parse!(parser, "sw", Opcode::Sw);
	assert_parse!(parser, "add", Opcode::Add);
	assert_parse!(parser, "addi", Opcode::Addi);
	assert_parse!(parser, "mul", Opcode::Mul);
	assert_parse!(parser, "muli", Opcode::Muli);
	assert_parse!(parser, "not", Opcode::Not);
	assert_parse!(parser, "and", Opcode::And);
	assert_parse!(parser, "andi", Opcode::Andi);
	assert_parse!(parser, "or", Opcode::Or);
	assert_parse!(parser, "ori", Opcode::Ori);
	assert_parse!(parser, "shf", Opcode::Shf);
	assert_parse!(parser, "shfi", Opcode::Shfi);
	assert_parse!(parser, "cmp", Opcode::Cmp);
	assert_parse!(parser, "jmp", Opcode::Jmp);
	assert_parse!(parser, "jr", Opcode::Jr);
	assert_parse!(parser, "jT", Opcode::JT);
	assert_parse!(parser, "j0", Opcode::J0);
	assert_parse!(parser, "j1", Opcode::J1);
	assert_parse!(parser, "jT0", Opcode::JT0);
	assert_parse!(parser, "jT1", Opcode::JT1);
	assert_parse!(parser, "j01", Opcode::J01);
	assert_parse!(parser, "call", Opcode::Call);
	assert_parse!(parser, "callr", Opcode::Callr);
	assert_parse!(parser, "ret", Opcode::Ret);
	assert_parse!(parser, "sys", Opcode::Sys);
	assert_parse!(parser, "break", Opcode::Break);
	assert_parse!(parser, "halt", Opcode::Halt);
}

#[test]
fn parser_inst_mov() {
	let mut parser = combine::parser(parser::inst_mov);
	assert_parse!(parser, "mov $a0, $zero", Instruction::Mov(Register::A0, Register::ZERO));
	assert_parse_err!(parser, "mov $a0 $zero");
	assert_parse_err!(parser, "mov $a0 $zero");
	assert_parse_err!(parser, "mov, $a0, $zero");
	assert_parse_err!(parser, "mov $a0 zero");
}
