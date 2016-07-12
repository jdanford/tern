use std::io;

use trit::Trit;
use types::*;

pub unsafe fn set_all(trits: *mut Trit, trit: Trit, len: isize) {
    for i in 0..len {
        set_trit(trits, i, trit);
    }
}

pub unsafe fn clear(trits: *mut Trit, len: isize) {
    set_all(trits, Trit::Zero, len);
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

pub unsafe fn map<F>(dest: *mut Trit, src: *const Trit, len: isize, f: F) where F: Fn(Trit) -> Trit {
    for i in 0..len {
        *dest.offset(i) = f(*src.offset(i));
    }
}

pub unsafe fn zip<F>(dest: *mut Trit, lhs: *const Trit, rhs: *const Trit, len: isize, f: F) where F: Fn(Trit, Trit) -> Trit {
    for i in 0..len {
        let l = *lhs.offset(i);
        let r = *rhs.offset(i);
        *dest.offset(i) = f(l, r);
    }
}

pub unsafe fn copy_blocks(src: *const Trit, size: usize, start: usize, blocks: Vec<(*mut Trit, usize)>) {
    let mut blocks = blocks.clone();

    let mut start = start;
    loop {
        let (_, block_size) = blocks[0];
        if start <= block_size {
            break;
        }

        start = start.saturating_sub(block_size);
        blocks.remove(0);
    }

    let mut i = 0;
    while i < size {
        let (block, block_size) = blocks.remove(0);
        let mut j = start;
        start = start.saturating_sub(block_size);
        while j < block_size && i < size {
            *block.offset(j as isize) = *src.offset(i as isize);
            i += 1;
            j += 1;
        }
    }
}

pub unsafe fn copy_from_iter<I>(dest: *mut Trit, iterable: I) where I: IntoIterator<Item=Trit> {
    for (i, trit) in iterable.into_iter().enumerate() {
        *dest.offset(i as isize) = trit;
    }
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

pub unsafe fn read_int(trits: *const Trit, len: isize) -> isize {
    let mut n = *trits.offset(len - 1) as isize;

    for i in (0..len - 1).rev() {
        let t = *trits.offset(i) as isize;
        n = n * 3 + t
    }

    n
}

pub fn write_trytes<I>(trits: *mut Trit, iterable: I) where I: IntoIterator<Item=isize> {
    for (i, tryte) in iterable.into_iter().enumerate() {
        let offset = TRYTE_ISIZE * (i as isize);
        unsafe { write_int(trits.offset(offset), tryte, TRYTE_ISIZE); }
    }
}

pub fn read_trytes(trits: *const Trit) -> (isize, isize, isize, isize) {
    unsafe { (
        read_int(tryte_offset!(trits, 0), TRYTE_ISIZE),
        read_int(tryte_offset!(trits, 1), TRYTE_ISIZE),
        read_int(tryte_offset!(trits, 2), TRYTE_ISIZE),
        read_int(tryte_offset!(trits, 3), TRYTE_ISIZE),
    ) }
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
