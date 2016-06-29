#[allow(dead_code)]
#[repr(i16)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Opcode {
	Mov,   // mov REG, REG
	Movw,  // movi REG ... WORD
	Ld,    // ld REG, ADDR
	Ldr,   // ldr REG, REG
	St,    // st ADDR, REG
	Str,   // str REG, REG

	Add,   // add REG, REG, REG
	Addi,  // addi REG, IMM
	Mul,   // mul REG, REG (writes to HI/LO)
	Muli,  // muli REG, IMM

	Not,   // not REG, REG
	And,   // and REG, REG, REG
	Andi,  // andi REG, IMM
	Or,    // or REG, REG, REG
	Ori,   // ori REG, IMM
	Shf,   // shf REG, REG, REG
	Shfi,  // shfi REG, IMM

	Cmp,   // cmp REG, REG, REG
	Jmp,   // jmp ADDR
	Jr,    // jr REG
	JT,    // jT REG, OFF
	J0,    // j0 REG, OFF
	J1,    // j1 REG, OFF
	JT0,   // jT0 REG, OFF
	JT1,   // jT1 REG, OFF
	J01,   // j01 REG, OFF

	Call,  // call ADDR
	Callr, // call REG
	Ret,   // ret
	Sys,   // sys

	Halt,  // halt
}
