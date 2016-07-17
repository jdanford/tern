use std::mem::transmute;

#[repr(i16)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Opcode {
    Mov = 0,      // mov REG, REG
    Movi = 1,     // movi REG, HALF
    Movw = 2,     // movw REG ... WORD
    Mova = 3,     // mova REG ... ADDR

    Lb = 4,       // lb REG, REG, OFF
    Lh = 5,       // lh REG, REG, OFF
    Lw = 6,       // lw REG, REG, OFF

    Sb = 7,       // sb REG, REG, OFF
    Sh = 8,       // sh REG, REG, OFF
    Sw = 9,       // sw REG, REG, OFF

    Add = 10,     // add REG, REG, REG
    Addi = 11,    // addi REG, HALF
    Mul = 12,     // mul REG, REG (writes to HI/LO)
    Muli = 13,    // muli REG, HALF

    Not = 14,     // not REG, REG
    And = 15,     // and REG, REG, REG
    Andi = 16,    // andi REG, HALF
    Or = 17,      // or REG, REG, REG
    Ori = 18,     // ori REG, HALF
    Shf = 19,     // shf REG, REG, REG
    Shfi = 20,    // shfi REG, HALF
    Cmp = 21,     // cmp REG, REG, REG

    Jmp = 22,     // jmp REG
    JT = 23,      // jT REG, RELADDR
    J0 = 24,      // j0 REG, RELADDR
    J1 = 25,      // j1 REG, RELADDR
    JT0 = 26,     // jT0 REG, RELADDR
    JT1 = 27,     // jT1 REG, RELADDR
    J01 = 28,     // j01 REG, RELADDR

    Call = 29,    // call REG
    Ret = 30,     // ret

    Syscall = 31, // syscall
    Break = 32,   // break
    Halt = 33,    // halt
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
            "mova" => true,
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
            "jT" => true,
            "j0" => true,
            "j1" => true,
            "jT0" => true,
            "jT1" => true,
            "j01" => true,
            "call" => true,
            "ret" => true,
            "syscall" => true,
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
            "mova" => Opcode::Mova,
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
            "jT" => Opcode::JT,
            "j0" => Opcode::J0,
            "j1" => Opcode::J1,
            "jT0" => Opcode::JT0,
            "jT1" => Opcode::JT1,
            "j01" => Opcode::J01,
            "call" => Opcode::Call,
            "ret" => Opcode::Ret,
            "syscall" => Opcode::Syscall,
            "break" => Opcode::Break,
            "halt" => Opcode::Halt,
            _ => panic!("Invalid opcode: {}", s),
        }
    }
}
