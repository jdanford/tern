use types::*;

pub unsafe fn set_all(trits: *mut Trit, trit: Trit, len: isize) {
    for i in 0..len {
        *trits.offset(i) = trit;
    }
}

pub unsafe fn clear(trits: *mut Trit, len: isize) {
    set_all(trits, Trit::Zero, len);
}

pub unsafe fn copy(dest: *mut Trit, src: *const Trit, len: isize) {
    for i in 0..len {
        *dest.offset(i) = *src.offset(i);
    }
}

pub fn map<F>(dest: &mut [Trit], src: &[Trit], f: F)
    where F: Fn(Trit) -> Trit
{
    for (d, s) in dest.iter_mut().zip(src) {
        *d = f(*s);
    }
}

pub unsafe fn zip<F>(dest: *mut Trit, lhs: *const Trit, rhs: *const Trit, len: isize, f: F)
    where F: Fn(Trit, Trit) -> Trit
{
    for i in 0..len {
        let l = *lhs.offset(i);
        let r = *rhs.offset(i);
        *dest.offset(i) = f(l, r);
    }
}

pub unsafe fn copy_blocks(src: *const Trit,
                          size: usize,
                          start: usize,
                          blocks: Vec<(*mut Trit, usize)>) {
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

pub fn copy_from_iter<I>(dest: &mut [Trit], iterable: I)
    where I: IntoIterator<Item = Trit>
{
    for (ptr, trit) in dest.iter_mut().zip(iterable) {
        *ptr = trit;
    }
}

pub fn from_str(dest: &mut [Trit], s: &str) {
    copy_from_iter(dest, s.bytes().rev().map(Trit::from))
}

pub fn to_str(trits: &[Trit]) -> String {
    let mut s = String::with_capacity(trits.len());

    for &t in trits {
        s.push(t.into());
    }

    s
}

pub fn from_int(trits: &mut [Trit], n: isize) {
    let negative = n < 0;
    let mut n = n.abs();

    for trit in trits {
        let t = match n % 3 {
            0 => Trit::Zero,
            1 => Trit::Pos,
            2 => {
                n += 1;
                Trit::Neg
            }
            _ => unreachable!()
        };

        *trit = if negative { -t } else { t };
        n /= 3;
    }
}

pub fn to_int(trits: &[Trit]) -> isize {
    let mut n = 0;

    for &trit in trits.iter().rev() {
        let t = trit as isize;
        n = n * 3 + t;
    }

    n
}

pub fn write_trytes<I>(trits: *mut Trit, iterable: I)
    where I: IntoIterator<Item = isize>
{
    for (i, tryte) in iterable.into_iter().enumerate() {
        let offset = TRYTE_SIZE * i;
        unsafe {
            use std::slice;
            let slice = slice::from_raw_parts_mut(trits, TRYTE_SIZE);
            from_int(&mut slice[offset..], tryte);
        }
    }
}

pub fn read_trytes(trits: &[Trit]) -> (isize, isize, isize, isize) {
    (to_int(&trits[0*TRYTE_SIZE..][..TRYTE_SIZE]),
     to_int(&trits[1*TRYTE_SIZE..][..TRYTE_SIZE]),
     to_int(&trits[2*TRYTE_SIZE..][..TRYTE_SIZE]),
     to_int(&trits[3*TRYTE_SIZE..][..TRYTE_SIZE]))
}

pub fn mutate<F>(trits: &mut [Trit], f: F)
    where F: Fn(Trit) -> Trit
{
    for trit in trits {
        *trit = f(*trit);
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

pub fn compare(lhs: &[Trit], rhs: &[Trit]) -> Trit {
    for (lt, rt) in lhs.iter().rev().zip(rhs.iter().rev()).skip(1) {
        if lt != rt {
            return Trit::from_ordering(lt.cmp(&rt));
        }
    }

    Trit::Zero
}

pub fn lowest_trit(trits: &[Trit]) -> Trit {
    for &trit in trits {
        if trit != Trit::Zero {
            return trit;
        }
    }

    Trit::Zero
}

pub fn highest_trit(trits: &[Trit]) -> Trit {
    for &trit in trits.iter().rev() {
        if trit != Trit::Zero {
            return trit;
        }
    }

    Trit::Zero
}

pub fn popcount(trits: &[Trit]) -> (isize, isize) {
    let mut hi_count = 0;
    let mut lo_count = 0;

    for &trit in trits {
        if trit == Trit::Neg {
            lo_count += 1;
        }
        else if trit == Trit::Pos {
            hi_count += 1;
        }
    }

    (hi_count, lo_count)
}
