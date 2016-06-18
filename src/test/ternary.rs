use Trit::*;
use ternary::*;

#[test]
fn ternary_from_str() {
	let a = Ternary { trits: [Pos, Neg, Zero, Zero, Neg, Pos] };
	let b = Ternary6::from_str("1T00T1");
	assert_eq!(a, b);
}

#[test]
fn ternary_add() {
	let a = Ternary6::from_str("0T0001");
	let b = Ternary6::from_str("0T00T0");
	let c = Ternary6::from_str("T100T1");
	assert_eq!(a + b, c);
}

#[test]
fn ternary_partial_product() {
	let a = Ternary6::from_str("0110T1");
	let b = Neg;
	let c = Ternary6::from_str("0TT01T");
	assert_eq!(a.partial_product(b), c);
}

#[test]
fn ternary_add_partial() {
	let mut a = Ternary12::from_str("00000000TT01");
	let b = Ternary6::from_str("000T01");
	let c = Ternary12::from_str("0000000T111T");
	// assert!(a != c);
	a.add_partial(b, 0);
	assert_eq!(a, c);
}

#[test]
fn ternary_product() {
	let a = Ternary6::from_str("T01010");
	let b = Ternary6::from_str("0001T0");
	let c = Ternary12::from_str("0000T11T1T00");
	assert_eq!(a.product(b), c);
}
