use types::*;
use registers::Register;
use opcodes::Opcode;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
	Mov(Register, Register),
	Movi(Register, Half),
	Movw(Register, Word),
	Lb(Register, Register, Tryte),
	Lh(Register, Register, Tryte),
	Lw(Register, Register, Tryte),
	Sb(Register, Register, Tryte),
	Sh(Register, Register, Tryte),
	Sw(Register, Register, Tryte),
	Add(Register, Register, Register),
	Addi(Register, Half),
	Mul(Register, Register),
	Muli(Register, Half),
	Not(Register, Register),
	And(Register, Register, Register),
	Andi(Register, Half),
	Or(Register, Register, Register),
	Ori(Register, Half),
	Shf(Register, Register, Register),
	Shfi(Register, Half),
	Cmp(Register, Register, Register),
	Jmp(String),
	Jr(Register),
	JT(Register, String),
	J0(Register, String),
	J1(Register, String),
	JT0(Register, String),
	JT1(Register, String),
	J01(Register, String),
	Call(String),
	Callr(Register),
	Ret,
	Sys,
	Break,
	Halt,
}

impl Instruction {
	pub fn size(&self) -> usize {
		match *self {
			Instruction::Movw(_, _) => WORD_SIZE * 2,
			Instruction::Jmp(_) => WORD_SIZE * 2,
			Instruction::Call(_) => WORD_SIZE * 2,
			_ => WORD_SIZE,
		}
	}
}

impl Into<Opcode> for Instruction {
	fn into(self) -> Opcode {
		match self {
			Instruction::Mov(_, _) => Opcode::Mov,
			Instruction::Movi(_, _) => Opcode::Movi,
			Instruction::Movw(_, _) => Opcode::Movw,
			Instruction::Lb(_, _, _) => Opcode::Lb,
			Instruction::Lh(_, _, _) => Opcode::Lh,
			Instruction::Lw(_, _, _) => Opcode::Lw,
			Instruction::Sb(_, _, _) => Opcode::Sb,
			Instruction::Sh(_, _, _) => Opcode::Sh,
			Instruction::Sw(_, _, _) => Opcode::Sw,
			Instruction::Add(_, _, _) => Opcode::Add,
			Instruction::Addi(_, _) => Opcode::Addi,
			Instruction::Mul(_, _) => Opcode::Mul,
			Instruction::Muli(_, _) => Opcode::Muli,
			Instruction::Not(_, _) => Opcode::Not,
			Instruction::And(_, _, _) => Opcode::And,
			Instruction::Andi(_, _) => Opcode::Andi,
			Instruction::Or(_, _, _) => Opcode::Or,
			Instruction::Ori(_, _) => Opcode::Ori,
			Instruction::Shf(_, _, _) => Opcode::Shf,
			Instruction::Shfi(_, _) => Opcode::Shfi,
			Instruction::Cmp(_, _, _) => Opcode::Cmp,
			Instruction::Jmp(_) => Opcode::Jmp,
			Instruction::Jr(_) => Opcode::Jr,
			Instruction::JT(_, _) => Opcode::JT,
			Instruction::J0(_, _) => Opcode::J0,
			Instruction::J1(_, _) => Opcode::J1,
			Instruction::JT0(_, _) => Opcode::JT0,
			Instruction::JT1(_, _) => Opcode::JT1,
			Instruction::J01(_, _) => Opcode::J01,
			Instruction::Call(_) => Opcode::Call,
			Instruction::Callr(_) => Opcode::Callr,
			Instruction::Ret => Opcode::Ret,
			Instruction::Sys => Opcode::Sys,
			Instruction::Break => Opcode::Break,
			Instruction::Halt => Opcode::Halt,
		}
	}
}
