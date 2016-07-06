use std::mem::transmute;

#[repr(i16)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Opcode {
	Mov = 0,    // mov REG, REG
	Movi = 1,   // movi REG, HALF
	Movw = 2,   // movw REG ... WORD

	Lb = 3,     // lb REG, REG, OFF
	Lh = 4,     // lh REG, REG, OFF
	Lw = 5,     // lw REG, REG, OFF

	Sb = 6,     // sb REG, REG, OFF
	Sh = 7,     // sh REG, REG, OFF
	Sw = 8,     // sw REG, REG, OFF

	Add = 9,    // add REG, REG, REG
	Addi = 10,  // addi REG, HALF
	Mul = 11,   // mul REG, REG (writes to HI/LO)
	Muli = 12,  // muli REG, HALF

	Not = 13,   // not REG, REG
	And = 14,   // and REG, REG, REG
	Andi = 15,  // andi REG, HALF
	Or = 16,    // or REG, REG, REG
	Ori = 17,   // ori REG, HALF
	Shf = 18,   // shf REG, REG, REG
	Shfi = 19,  // shfi REG, HALF
	Cmp = 20,   // cmp REG, REG, REG

	Jmp = 21,   // jmp ... ADDR
	Jr = 22,    // jr REG
	JT = 23,    // jT REG, RELADDR
	J0 = 24,    // j0 REG, RELADDR
	J1 = 25,    // j1 REG, RELADDR
	JT0 = 26,   // jT0 REG, RELADDR
	JT1 = 27,   // jT1 REG, RELADDR
	J01 = 28,   // j01 REG, RELADDR

	Call = 29,  // call ... ADDR
	Callr = 30, // callr REG
	Ret = 31,   // ret

	Sys = 32,   // sys
	Break = 33, // break
	Halt = 34,  // halt
}

impl Opcode {
	pub fn index_is_valid(n: isize) -> bool {
		(Opcode::Mov as isize) <= n && n <= (Opcode::Halt as isize)
	}

	pub fn name_is_valid(s: &str) -> bool {
		match s {
			"mov" => true,
			"movi" => true,
			"movw" => true,
			"lb" => true,
			"lh" => true,
			"lw" => true,
			"sb" => true,
			"sh" => true,
			"sw" => true,
			"add" => true,
			"addi" => true,
			"mul" => true,
			"muli" => true,
			"not" => true,
			"and" => true,
			"andi" => true,
			"or" => true,
			"ori" => true,
			"shf" => true,
			"shfi" => true,
			"cmp" => true,
			"jmp" => true,
			"jr" => true,
			"jT" => true,
			"j0" => true,
			"j1" => true,
			"jT0" => true,
			"jT1" => true,
			"j01" => true,
			"call" => true,
			"callr" => true,
			"ret" => true,
			"sys" => true,
			"break" => true,
			"halt" => true,
			_ => false,
		}
	}
}

impl From<isize> for Opcode {
	fn from(n: isize) -> Opcode {
		if !Opcode::index_is_valid(n) {
			panic!("Invalid index: {}", n);
		}

		unsafe { transmute(n as i16) }
	}
}

impl<'a> From<&'a str> for Opcode {
	fn from(s: &str) -> Opcode {
		match s {
			"mov" => Opcode::Mov,
			"movi" => Opcode::Movi,
			"movw" => Opcode::Movw,
			"lb" => Opcode::Lb,
			"lh" => Opcode::Lh,
			"lw" => Opcode::Lw,
			"sb" => Opcode::Sb,
			"sh" => Opcode::Sh,
			"sw" => Opcode::Sw,
			"add" => Opcode::Add,
			"addi" => Opcode::Addi,
			"mul" => Opcode::Mul,
			"muli" => Opcode::Muli,
			"not" => Opcode::Not,
			"and" => Opcode::And,
			"andi" => Opcode::Andi,
			"or" => Opcode::Or,
			"ori" => Opcode::Ori,
			"shf" => Opcode::Shf,
			"shfi" => Opcode::Shfi,
			"cmp" => Opcode::Cmp,
			"jmp" => Opcode::Jmp,
			"jr" => Opcode::Jr,
			"jT" => Opcode::JT,
			"j0" => Opcode::J0,
			"j1" => Opcode::J1,
			"jT0" => Opcode::JT0,
			"jT1" => Opcode::JT1,
			"j01" => Opcode::J01,
			"call" => Opcode::Call,
			"callr" => Opcode::Callr,
			"ret" => Opcode::Ret,
			"sys" => Opcode::Sys,
			"break" => Opcode::Break,
			"halt" => Opcode::Halt,
			_ => panic!("Invalid opcode: {}", s),
		}
	}
}
