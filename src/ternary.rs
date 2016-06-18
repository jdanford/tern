use std::fmt;
use std::fmt::Write;
use std::ops;
use trit::Trit;

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ternary<T: Copy> {
    pub trits: T
}

macro_rules! define_ternary { ($n:expr) => {

impl Ternary<[Trit; $n]> {
    pub fn from_iter<T: Into<Trit>, I: Iterator<Item=T>>(i: I) -> Ternary<[Trit; $n]> {
        let mut trits = [Trit::Zero; $n];
        for (i, b) in i.take($n).enumerate() {
            trits[i] = b.into();
        }

        Ternary { trits: trits }
    }

    pub fn from_str(s: &str) -> Ternary<[Trit; $n]> {
        Ternary::<[Trit; $n]>::from_iter(s.bytes().rev())
    }

    pub fn map<F: Fn(Trit) -> Trit>(self, f: F) -> Ternary<[Trit; $n]> {
        let mut tern = self;
        for (i, &t) in self.trits.iter().enumerate() {
            tern.trits[i] = f(t);
        }

        tern
    }

    pub fn zip_with(self, rhs: Ternary<[Trit; $n]>, f: fn(Trit, Trit) -> Trit) -> Ternary<[Trit; $n]> {
        let mut tern = self;
        for (i, (&a, &b)) in self.trits.iter().zip(rhs.trits.iter()).enumerate() {
            tern.trits[i] = f(a, b);
        }

        tern
    }

    pub fn sum_with_carry(self, rhs: Ternary<[Trit; $n]>) -> (Ternary<[Trit; $n]>, Trit) {
        let mut tern = self;
        let mut carry = Trit::Zero;

        let trit_pairs = self.trits.iter().zip(rhs.trits.iter());
        for (i, (&a, &b)) in trit_pairs.enumerate() {
            let (trit_i, carry_i) = a.sum_with_carry(b, carry);
            tern.trits[i] = trit_i;
            carry = carry_i;
        }

        (tern, carry)
    }

    pub fn partial_product(self, rhs: Trit) -> Ternary<[Trit; $n]> {
        self.map(|t| Trit::product(t, rhs))
    }

    pub fn product(self, rhs: Ternary<[Trit; $n]>) -> Ternary<[Trit; ($n * 2)]> {
        let mut tern = Ternary { trits: [Trit::Zero; ($n * 2)] };

        for (i, &t) in rhs.trits.iter().enumerate() {
            tern.add_partial(self.partial_product(t), i);
        }

        tern
    }
}

impl Ternary<[Trit; ($n * 2)]> {
    pub fn add_partial(&mut self, rhs: Ternary<[Trit; $n]>, offset: usize) {
        let mut carry = Trit::Zero;
        for (i, &t) in rhs.trits.iter().enumerate() {
            let (trit_i, carry_i) = self.trits[offset + i].sum_with_carry(t, carry);
            self.trits[offset + i] = trit_i;
            carry = carry_i;
        }

        self.trits[offset + $n] = carry;
    }

    pub fn split_lo_hi(self) -> (Ternary<[Trit; $n]>, Ternary<[Trit; $n]>) {
        let mut lo = [Trit::Zero; $n];
        let mut hi = [Trit::Zero; $n];
        lo.copy_from_slice(&self.trits[0..$n]);
        hi.copy_from_slice(&self.trits[$n..($n * 2)]);
        (Ternary { trits: lo }, Ternary { trits: hi })
    }
}

impl ops::Neg for Ternary<[Trit; $n]> {
    type Output = Ternary<[Trit; $n]>;

    fn neg(self) -> Self::Output {
        self.map(&Trit::neg)
    }
}

impl ops::BitAnd for Ternary<[Trit; $n]> {
    type Output = Ternary<[Trit; $n]>;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.zip_with(rhs, Trit::bitand)
    }
}

impl ops::BitOr for Ternary<[Trit; $n]> {
    type Output = Ternary<[Trit; $n]>;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.zip_with(rhs, Trit::bitor)
    }
}

impl ops::Add for Ternary<[Trit; $n]> {
    type Output = Ternary<[Trit; $n]>;

    fn add(self, rhs: Self) -> Self::Output {
        let (sum, _) = self.sum_with_carry(rhs);
        sum
    }
}

impl ops::Sub for Ternary<[Trit; $n]> {
    type Output = Ternary<[Trit; $n]>;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl ops::Mul for Ternary<[Trit; $n]> {
    type Output = Ternary<[Trit; $n]>;

    fn mul(self, rhs: Self) -> Self::Output {
        let product = self.product(rhs);
        let (lo, _) = product.split_lo_hi();
        lo
    }
}

impl fmt::Debug for Ternary<[Trit; $n]> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let l = self.trits.len();
        for i in (0..l).rev() {
            let trit = self.trits[i];
            let _ = f.write_char(trit.into());
        }

        Ok(())
    }
}

impl fmt::Display for Ternary<[Trit; $n]> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

};}

define_ternary!(4);
pub type Ternary4 = Ternary<[Trit; 4]>;

define_ternary!(8);
pub type Ternary8 = Ternary<[Trit; 8]>;
