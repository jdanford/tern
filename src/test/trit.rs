use Trit::*;

#[test]
fn trit_negate() {
	assert_eq!(-Neg,  Pos);
	assert_eq!(-Zero, Zero);
	assert_eq!(-Pos,  Neg);
}

#[test]
fn trit_and() {
	assert_eq!(Pos  & Zero, Zero);
	assert_eq!(Neg  & Zero, Zero);
	assert_eq!(Zero & Zero, Zero);
	assert_eq!(Zero & Pos,  Zero);
	assert_eq!(Zero & Neg,  Zero);
	assert_eq!(Neg  & Neg,  Neg);
	assert_eq!(Neg  & Pos,  Neg);
	assert_eq!(Pos  & Neg,  Neg);
	assert_eq!(Pos  & Pos,  Pos);
}

#[test]
fn trit_or() {
	assert_eq!(Pos  | Zero, Pos);
	assert_eq!(Neg  | Zero, Neg);
	assert_eq!(Zero | Zero, Zero);
	assert_eq!(Zero | Pos,  Pos);
	assert_eq!(Zero | Neg,  Neg);
	assert_eq!(Neg  | Neg,  Neg);
	assert_eq!(Neg  | Pos,  Pos);
	assert_eq!(Pos  | Neg,  Pos);
	assert_eq!(Pos  | Pos,  Pos);
}

#[test]
fn trit_sum_with_carry() {
	assert_eq!(Pos.sum_with_carry(Zero, Zero),  (Pos,  Zero));
	assert_eq!(Neg.sum_with_carry(Zero, Zero),  (Neg,  Zero));
	assert_eq!(Zero.sum_with_carry(Zero, Zero), (Zero, Zero));
	assert_eq!(Zero.sum_with_carry(Pos, Zero),  (Pos,  Zero));
	assert_eq!(Zero.sum_with_carry(Neg, Zero),  (Neg,  Zero));
	assert_eq!(Neg.sum_with_carry(Neg, Zero),   (Pos,  Neg));
	assert_eq!(Pos.sum_with_carry(Pos, Zero),   (Neg,  Pos));
	assert_eq!(Neg.sum_with_carry(Pos, Zero),   (Zero, Zero));
	assert_eq!(Pos.sum_with_carry(Neg, Zero),   (Zero, Zero));
}
