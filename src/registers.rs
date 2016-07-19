use std::mem::transmute;
use std::str::FromStr;

pub const REGISTER_COUNT: usize = 24;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Register {
    ZERO = 0,
    RA = 1,
    LO = 2,
    HI = 3,
    SP = 4,
    FP = 5,
    A0 = 6,
    A1 = 7,
    A2 = 8,
    A3 = 9,
    A4 = 10,
    A5 = 11,
    T0 = 12,
    T1 = 13,
    T2 = 14,
    T3 = 15,
    T4 = 16,
    T5 = 17,
    S0 = 18,
    S1 = 19,
    S2 = 20,
    S3 = 21,
    S4 = 22,
    S5 = 23,
}

impl Register {
    pub fn index_is_valid(n: isize) -> bool {
        0 <= n && n < REGISTER_COUNT as isize
    }

    pub fn name_is_valid(s: &str) -> bool {
        match s {
            "zero" => true,
            "ra" => true,
            "lo" => true,
            "hi" => true,
            "sp" => true,
            "fp" => true,
            "a0" => true,
            "a1" => true,
            "a2" => true,
            "a3" => true,
            "a4" => true,
            "a5" => true,
            "t0" => true,
            "t1" => true,
            "t2" => true,
            "t3" => true,
            "t4" => true,
            "t5" => true,
            "s0" => true,
            "s1" => true,
            "s2" => true,
            "s3" => true,
            "s4" => true,
            "s5" => true,
            _ => false,
        }
    }
}

impl From<isize> for Register {
    fn from(n: isize) -> Register {
        if !Register::index_is_valid(n) {
            panic!("Invalid index: {}", n);
        }

        unsafe { transmute(n as u8) }
    }
}

impl<'a> From<&'a str> for Register {
    fn from(s: &str) -> Register {
        match s {
            "zero" => Register::ZERO,
            "ra" => Register::RA,
            "lo" => Register::LO,
            "hi" => Register::HI,
            "sp" => Register::SP,
            "fp" => Register::FP,
            "a0" => Register::A0,
            "a1" => Register::A1,
            "a2" => Register::A2,
            "a3" => Register::A3,
            "a4" => Register::A4,
            "a5" => Register::A5,
            "t0" => Register::T0,
            "t1" => Register::T1,
            "t2" => Register::T2,
            "t3" => Register::T3,
            "t4" => Register::T4,
            "t5" => Register::T5,
            "s0" => Register::S0,
            "s1" => Register::S1,
            "s2" => Register::S2,
            "s3" => Register::S3,
            "s4" => Register::S4,
            "s5" => Register::S5,
            _ => panic!("Invalid register: {}", s),
        }
    }
}

impl FromStr for Register {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "$zero" | "$0" => Ok(Register::ZERO),
            "$ra" | "$1" => Ok(Register::RA),
            "$lo" | "$2" => Ok(Register::LO),
            "$hi" | "$3" => Ok(Register::HI),
            "$sp" | "$4" => Ok(Register::SP),
            "$fp" | "$5" => Ok(Register::FP),
            "$a0" | "$6" => Ok(Register::A0),
            "$a1" | "$7" => Ok(Register::A1),
            "$a2" | "$8" => Ok(Register::A2),
            "$a3" | "$9" => Ok(Register::A3),
            "$a4" | "$10" => Ok(Register::A4),
            "$a5" | "$11" => Ok(Register::A5),
            "$t0" | "$12" => Ok(Register::T0),
            "$t1" | "$13" => Ok(Register::T1),
            "$t2" | "$14" => Ok(Register::T2),
            "$t3" | "$15" => Ok(Register::T3),
            "$t4" | "$16" => Ok(Register::T4),
            "$t5" | "$17" => Ok(Register::T5),
            "$s0" | "$18" => Ok(Register::S0),
            "$s1" | "$19" => Ok(Register::S1),
            "$s2" | "$20" => Ok(Register::S2),
            "$s3" | "$21" => Ok(Register::S3),
            "$s4" | "$22" => Ok(Register::S4),
            "$s5" | "$23" => Ok(Register::S5),
            _ => Err(s.to_string()),
        }
    }
}
