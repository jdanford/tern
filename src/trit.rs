use std::cmp::Ordering;
use std::fmt;
use std::fmt::Write;
use std::ops;

use trit::Trit::*;

#[repr(i8)]
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum Trit {
    Neg = -1,
    Zero = 0,
    Pos = 1,
}

impl Trit {
    pub fn from_ordering(o: Ordering) -> Trit {
        match o {
            Ordering::Less => Neg,
            Ordering::Equal => Zero,
            Ordering::Greater => Pos,
        }
    }
}

impl Default for Trit {
    fn default() -> Self {
        Trit::Zero
    }
}

impl From<i8> for Trit {
    fn from(n: i8) -> Trit {
        match n {
            1 => Pos,
            -1 => Neg,
            _ => Zero,
        }
    }
}

impl From<Trit> for u8 {
    fn from(t: Trit) -> u8 {
        match t {
            Neg => b'T',
            Zero => b'0',
            Pos => b'1',
        }
    }
}

impl From<u8> for Trit {
    fn from(b: u8) -> Trit {
        match b {
            b'T' => Neg,
            b'1' => Pos,
            _ => Zero,
        }
    }
}

impl From<Trit> for char {
    fn from(t: Trit) -> char {
        match t {
            Neg => 'T',
            Zero => '0',
            Pos => '1',
        }
    }
}

impl From<char> for Trit {
    fn from(c: char) -> Trit {
        match c {
            'T' => Neg,
            '1' => Pos,
            _ => Zero,
        }
    }
}

impl ops::Neg for Trit {
    type Output = Trit;

    fn neg(self) -> Self::Output {
        match self {
            Neg => Pos,
            Zero => Zero,
            Pos => Neg,
        }
    }
}

impl ops::Mul for Trit {
    type Output = Trit;

    fn mul(self, rhs: Trit) -> Self::Output {
        match self {
            Pos => rhs,
            Zero => Zero,
            Neg => -rhs,
        }
    }
}

impl ops::BitAnd for Trit {
    type Output = Trit;

    fn bitand(self, rhs: Trit) -> Self::Output {
        match (self, rhs) {
            (Pos, Pos) => Pos,
            (_, Zero) => Zero,
            (Zero, _) => Zero,
            _ => Neg,
        }
    }
}

impl ops::BitOr for Trit {
    type Output = Trit;

    fn bitor(self, rhs: Trit) -> Self::Output {
        match (self, rhs) {
            (Neg, Neg) => Neg,
            (a, Zero) => a,
            (Zero, b) => b,
            _ => Pos,
        }
    }
}

impl Trit {
    pub fn sum_with_carry(self, rhs: Trit, carry_in: Trit) -> (Trit, Trit) {
        let isum = (self as i8) + (rhs as i8) + (carry_in as i8);
        match isum {
            -3 => (Zero, Neg),
            -2 => (Pos, Neg),
            -1 => (Neg, Zero),
            1 => (Pos, Zero),
            2 => (Neg, Pos),
            3 => (Zero, Pos),
            _ => (Zero, Zero),
        }
    }
}

impl fmt::Debug for Trit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = f.write_char(char::from(*self));
        Ok(())
    }
}

impl fmt::Display for Trit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
