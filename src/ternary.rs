use std::io;
use std::mem::transmute;
use std::slice::from_raw_parts;
use trit::Trit;
use types::*;

pub unsafe fn clear(trits: *mut Trit, len: isize) {
    for i in 0..len {
        *trits.offset(i) = Trit::Zero;
    }
}

pub unsafe fn get_trit(trits: *mut Trit, i: isize) -> Trit {
    *trits.offset(i)
}

pub unsafe fn set_trit(trits: *mut Trit, i: isize, trit: Trit) {
    *trits.offset(i) = trit;
}

pub unsafe fn copy(dest: *mut Trit, src: *const Trit, len: isize) {
    for i in 0..len {
        *dest.offset(i) = *src.offset(i);
    }
}

pub unsafe fn copy_from_iter<I>(dest: *mut Trit, iterable: I) where I: IntoIterator<Item=Trit> {
    for (i, trit) in iterable.into_iter().enumerate() {
        *dest.offset(i as isize) = trit;
    }
}

pub unsafe fn with_bytes<F: Fn(&[u8])>(trits: *const Trit, len: isize, f: F) {
    let bytes = from_raw_parts(transmute(trits), len as usize);
    f(bytes);
}

pub unsafe fn write_str(dest: *mut Trit, s: &str) {
    copy_from_iter(dest, s.bytes().rev().map(Trit::from))
}

pub unsafe fn print<W: io::Write>(trits: *const Trit, mut writer: W, len: isize) {
    for i in (0..len).rev() {
        let trit = *trits.offset(i);
        let c: char = trit.into();
        let container = [c as u8; 1];
        let _ = writer.write(&container);
    }

    let _ = writer.write(b"\n");
}

pub unsafe fn write_int(trits: *mut Trit, n: isize, len: isize) {
    let negative = n < 0;
    let mut n = n.abs();

    for i in 0..len {
        let trit = match n % 3 {
            1 => Trit::Pos,
            0 => Trit::Zero,
            _ => {
                n += 1;
                Trit::Neg
            }
        };

        *trits.offset(i) = if negative { -trit } else { trit };
        n /= 3;
    }
}

pub fn write_trytes<I>(trits: *mut Trit, iterable: I) where I: IntoIterator<Item=isize> {
    for (i, tryte) in iterable.into_iter().enumerate() {
        let offset = TRYTE_ISIZE * (i as isize);
        unsafe {
            write_int(trits.offset(offset), tryte, TRYTE_ISIZE);
        }
    }
}

pub unsafe fn read_int(trits: *const Trit, len: isize) -> isize {
    let mut n = *trits.offset(len - 1) as isize;

    for i in (0..len - 1).rev() {
        let t = *trits.offset(i) as isize;
        n = n * 3 + t
    }

    n
}

pub unsafe fn get_lst(trits: *const Trit, len: isize) -> Trit {
    for i in 0..len - 1 {
        let trit = *trits.offset(i);
        if trit != Trit::Zero {
            return trit;
        }
    }

    Trit::Zero
}

pub unsafe fn get_mst(trits: *const Trit, len: isize) -> Trit {
    for i in (0..len - 1).rev() {
        let trit = *trits.offset(i);
        if trit != Trit::Zero {
            return trit;
        }
    }

    Trit::Zero
}

pub unsafe fn mutate<F>(trits: *mut Trit, len: isize, f: F) where F: Fn(Trit) -> Trit {
    for i in 0..len {
        *trits.offset(i) = f(*trits.offset(i));
    }
}

pub unsafe fn addmul(dest: *mut Trit, rhs: *const Trit, t: Trit, len: isize) -> Trit {
    let mut carry = Trit::Zero;

    for i in 0..len {
        let l = *dest.offset(i);
        let r = *rhs.offset(i);
        let (trit_i, carry_i) = l.sum_with_carry(r * t, carry);
        *dest.offset(i) = trit_i;
        carry = carry_i;
    }

    carry
}

pub unsafe fn add(dest: *mut Trit, lhs: *const Trit, rhs: *const Trit, len: isize) -> Trit {
    let mut carry = Trit::Zero;

    for i in 0..len {
        let l = *lhs.offset(i);
        let r = *rhs.offset(i);
        let (trit_i, carry_i) = l.sum_with_carry(r, carry);
        *dest.offset(i) = trit_i;
        carry = carry_i;
    }

    carry
}

pub unsafe fn multiply(dest: *mut Trit, lhs: *const Trit, rhs: *const Trit, len: isize) {
    let mut carry;
    for i in 0..len {
        carry = addmul(dest.offset(i), lhs, *rhs.offset(i), len);
        *dest.offset(i + len) = carry;
    }
}
