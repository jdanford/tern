use std::fmt;
use std::fmt::Write;
use trit::Trit;

pub fn clear(trits: &mut [Trit], len: usize) {
    for i in 0..len {
        trits[i] = Trit::Zero;
    }
}

pub fn write_trits<I>(trits: &mut [Trit], rhs: I) where I: IntoIterator<Item=Trit> {
    for (i, b) in rhs.into_iter().enumerate() {
        trits[i] = b;
    }
}

pub fn write_str(trits: &mut [Trit], s: &str) {
    write_trits(trits, s.bytes().rev().map(Trit::from))
}

pub fn read_int(trits: &[Trit], len: usize) -> isize {
    let mut n = trits[len - 1] as isize;

    for i in (0..len - 1).rev() {
        let t = trits[i] as isize;
        n = n * 3 + t
    }

    n
}

pub fn get_lst(trits: &[Trit], len: usize) -> Trit {
    for i in 0..len - 1 {
        let trit = trits[i];
        if trit != Trit::Zero {
            return trit;
        }
    }

    Trit::Zero
}

pub fn get_mst(trits: &[Trit], len: usize) -> Trit {
    for i in (0..len - 1).rev() {
        let trit = trits[i];
        if trit != Trit::Zero {
            return trit;
        }
    }

    Trit::Zero
}

pub fn fmt(trits: &[Trit], f: &mut fmt::Formatter) {
    for i in (0..trits.len()).rev() {
        let trit = trits[i];
        let _ = f.write_char(trit.into());
    }
}

pub fn mutate<F>(trits: &mut [Trit], f: F) where F: Fn(Trit) -> Trit {
    for i in 0..trits.len() {
        trits[i] = f(trits[i]);
    }
}

pub fn mutate2<I, F>(trits: &mut [Trit], rhs: I, f: F) where I: IntoIterator<Item=Trit>, F: Fn(Trit, Trit) -> Trit {
    for (i, t) in rhs.into_iter().enumerate() {
        trits[i] = f(trits[i], t);
    }
}

pub fn add<I: IntoIterator<Item=Trit>>(trits: &mut [Trit], rhs: I) -> Trit {
    let mut carry = Trit::Zero;

    for (i, r) in rhs.into_iter().enumerate() {
        let (trit_i, carry_i) = trits[i].sum_with_carry(r, carry);
        trits[i] = trit_i;
        carry = carry_i;
    }

    carry
}

pub fn multiply(trits: &mut [Trit], lhs: &[Trit], rhs: &[Trit]) {
    let mut carry;
    let len = lhs.len();
    for i in 0..len {
        carry = add(&mut trits[i..], lhs.into_iter().map(|&l| l * rhs[i]));
        trits[i + len] = carry;
    }
}
