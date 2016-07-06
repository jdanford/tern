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
}

impl From<isize> for Opcode {
	fn from(n: isize) -> Opcode {
		if !Opcode::index_is_valid(n) {
			panic!()
		}

		unsafe { transmute(n as i16) }
	}
}
