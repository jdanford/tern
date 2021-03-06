use std::fmt;
use std::mem::transmute;

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum Opcode {
    Mov = 0, // mov REG, REG
    Movi = 1, // movi REG, HALF
    Movw = 2, // movw REG ... WORD
    Mova = 3, // mova REG ... ADDR

    Lt = 4, // lt REG, REG, OFF
    Lh = 5, // lh REG, REG, OFF
    Lw = 6, // lw REG, REG, OFF

    St = 7, // st REG, REG, OFF
    Sh = 8, // sh REG, REG, OFF
    Sw = 9, // sw REG, REG, OFF

    Add = 10, // add REG, REG, REG
    Addi = 11, // addi REG, HALF
    Mul = 12, // mul REG, REG (writes to HI/LO)
    Muli = 13, // muli REG, HALF

    Not = 14, // not REG, REG
    And = 15, // and REG, REG, REG
    Andi = 16, // andi REG, HALF
    Or = 17, // or REG, REG, REG
    Ori = 18, // ori REG, HALF
    Shf = 19, // shf REG, REG, REG
    Shfi = 20, // shfi REG, HALF
    Cmp = 21, // cmp REG, REG, REG

    Jmp = 22, // jmp REG
    JT = 23, // jT REG, RELADDR
    J0 = 24, // j0 REG, RELADDR
    J1 = 25, // j1 REG, RELADDR
    JT0 = 26, // jT0 REG, RELADDR
    JT1 = 27, // jT1 REG, RELADDR
    J01 = 28, // j01 REG, RELADDR

    Call = 29, // call REG
    Ret = 30, // ret

    Syscall = 31, // syscall
    Break = 32, // break
    Halt = 33, // halt
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
            "lt" => true,
            "lh" => true,
            "lw" => true,
            "st" => true,
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

    pub fn name(&self) -> &'static str {
        match *self {
            Opcode::Mov => "mov",
            Opcode::Movi => "movi",
            Opcode::Movw => "movw",
            Opcode::Mova => "mova",
            Opcode::Lt => "lt",
            Opcode::Lh => "lh",
            Opcode::Lw => "lw",
            Opcode::St => "st",
            Opcode::Sh => "sh",
            Opcode::Sw => "sw",
            Opcode::Add => "add",
            Opcode::Addi => "addi",
            Opcode::Mul => "mul",
            Opcode::Muli => "muli",
            Opcode::Not => "not",
            Opcode::And => "and",
            Opcode::Andi => "andi",
            Opcode::Or => "or",
            Opcode::Ori => "ori",
            Opcode::Shf => "shf",
            Opcode::Shfi => "shfi",
            Opcode::Cmp => "cmp",
            Opcode::Jmp => "jmp",
            Opcode::JT => "jT",
            Opcode::J0 => "j0",
            Opcode::J1 => "j1",
            Opcode::JT0 => "jT0",
            Opcode::JT1 => "jT1",
            Opcode::J01 => "j01",
            Opcode::Call => "call",
            Opcode::Ret => "ret",
            Opcode::Syscall => "syscall",
            Opcode::Break => "break",
            Opcode::Halt => "halt",
        }
    }

    pub fn arity(&self) -> usize {
        match *self {
            Opcode::Mov => 2,
            Opcode::Movi => 2,
            Opcode::Movw => 2,
            Opcode::Mova => 2,
            Opcode::Lt => 3,
            Opcode::Lh => 3,
            Opcode::Lw => 3,
            Opcode::St => 3,
            Opcode::Sh => 3,
            Opcode::Sw => 3,
            Opcode::Add => 3,
            Opcode::Addi => 2,
            Opcode::Mul => 2,
            Opcode::Muli => 2,
            Opcode::Not => 2,
            Opcode::And => 3,
            Opcode::Andi => 2,
            Opcode::Or => 3,
            Opcode::Ori => 2,
            Opcode::Shf => 3,
            Opcode::Shfi => 2,
            Opcode::Cmp => 3,
            Opcode::Jmp => 1,
            Opcode::JT => 2,
            Opcode::J0 => 2,
            Opcode::J1 => 2,
            Opcode::JT0 => 2,
            Opcode::JT1 => 2,
            Opcode::J01 => 2,
            Opcode::Call => 1,
            Opcode::Ret => 0,
            Opcode::Syscall => 0,
            Opcode::Break => 0,
            Opcode::Halt => 0,
        }
    }
}

impl From<isize> for Opcode {
    fn from(n: isize) -> Opcode {
        if !Opcode::index_is_valid(n) {
            panic!("Invalid index: {}", n);
        }

        unsafe { transmute(n as u8) }
    }
}

impl<'a> From<&'a str> for Opcode {
    fn from(s: &str) -> Opcode {
        match s {
            "mov" => Opcode::Mov,
            "movi" => Opcode::Movi,
            "movw" => Opcode::Movw,
            "mova" => Opcode::Mova,
            "lt" => Opcode::Lt,
            "lh" => Opcode::Lh,
            "lw" => Opcode::Lw,
            "st" => Opcode::St,
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

impl Into<&'static str> for Opcode {
    fn into(self) -> &'static str {
        self.name()
    }
}

impl fmt::Debug for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.name())
    }
}
