use std::mem::transmute;

#[repr(i16)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Opcode {
	Mov,   // mov REG, REG
	Movi,  // movi REG, HALF
	Movw,  // movw REG ... WORD

	Lb,    // lb REG, REG, OFF
	Lh,    // lh REG, REG, OFF
	Lw,    // lw REG, REG, OFF

	Sb,    // sb REG, REG, OFF
	Sh,    // sh REG, REG, OFF
	Sw,    // sw REG, REG, OFF

	Add,   // add REG, REG, REG
	Addi,  // addi REG, HALF
	Mul,   // mul REG, REG (writes to HI/LO)
	Muli,  // muli REG, HALF

	Not,   // not REG, REG
	And,   // and REG, REG, REG
	Andi,  // andi REG, HALF
	Or,    // or REG, REG, REG
	Ori,   // ori REG, HALF
	Shf,   // shf REG, REG, REG
	Shfi,  // shfi REG, HALF

	Cmp,   // cmp REG, REG, REG
	Jmp,   // jmp ... ADDR
	Jr,    // jr REG
	JT,    // jT REG, RELADDR
	J0,    // j0 REG, RELADDR
	J1,    // j1 REG, RELADDR
	JT0,   // jT0 REG, RELADDR
	JT1,   // jT1 REG, RELADDR
	J01,   // j01 REG, RELADDR

	Call,  // call ... ADDR
	Callr, // callr REG
	Ret,   // ret

	Sys,   // sys
	Brk,   // brk
	Halt,  // halt
}

impl From<isize> for Opcode {
	fn from(n: isize) -> Opcode {
		// if !Opcode::index_is_valid(n) {
		// 	panic!()
		// }

		unsafe { transmute(n as i16) }
	}
}
