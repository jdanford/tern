use Trit;
use ternary::*;

#[test]
fn ternary_from_str() {
	let a = Ternary { trits: [Trit::Pos, Trit::Neg, Trit::Zero, Trit::Zero] };
	let b = Ternary4::from_str("00T1");
	assert_eq!(a, b);
}

#[test]
fn ternary_add() {
	let a = Ternary4::from_str("0001");
	let b = Ternary4::from_str("00T0");
	let c = Ternary4::from_str("00T1");
	assert_eq!(a + b, c);
}

#[test]
fn ternary_partial_product() {
	let a = Ternary4::from_str("10T1");
	let b = Trit::Neg;
	let c = Ternary4::from_str("T01T");
	assert_eq!(a.partial_product(b), c);
}

#[test]
fn ternary_add_partial() {
	let mut a = Ternary8::from_str("0000TT01");
	let b = Ternary4::from_str("0T01");
	let c = Ternary8::from_str("000T111T");
	// assert!(a != c);
	a.add_partial(b, 0);
	assert_eq!(a, c);
}

#[test]
fn ternary_product() {
	let a = Ternary4::from_str("1010");
	let b = Ternary4::from_str("01T0");
	let c = Ternary8::from_str("001T1T00");
	assert_eq!(a.product(b), c);
}
