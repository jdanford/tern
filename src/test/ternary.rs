use std::iter;
use ternary;
use trit::Trit;

#[test]
fn ternary_write_trits() {
	let mut trits = [Trit::Zero; 6];
	ternary::write_trits_iter(&mut trits, iter::repeat(Trit::Pos).take(6));
	assert_eq!(trits, [Trit::Pos; 6]);
}

#[test]
fn ternary_write_str() {
	let mut trits = [Trit::Zero; 6];
	ternary::write_str(&mut trits, "1T00T1");
	assert_eq!(trits, [Trit::Pos, Trit::Neg, Trit::Zero, Trit::Zero, Trit::Neg, Trit::Pos]);
}

#[test]
fn ternary_write_int() {
	let mut trits = [Trit::Zero; 6];
	ternary::write_int(&mut trits, 20, 6);
	assert_eq!(trits, [Trit::Neg, Trit::Pos, Trit::Neg, Trit::Pos, Trit::Zero, Trit::Zero]);
}

#[test]
fn ternary_read_int() {
	let mut trits = [Trit::Zero; 3];

	ternary::write_str(&mut trits, "00T");
	assert_eq!(ternary::read_int(&trits, 3), -1);

	ternary::write_str(&mut trits, "000");
	assert_eq!(ternary::read_int(&trits, 3), 0);

	ternary::write_str(&mut trits, "001");
	assert_eq!(ternary::read_int(&trits, 3), 1);

	ternary::write_str(&mut trits, "01T");
	assert_eq!(ternary::read_int(&trits, 3), 2);

	ternary::write_str(&mut trits, "010");
	assert_eq!(ternary::read_int(&trits, 3), 3);

	ternary::write_str(&mut trits, "011");
	assert_eq!(ternary::read_int(&trits, 3), 4);

	ternary::write_str(&mut trits, "1TT");
	assert_eq!(ternary::read_int(&trits, 3), 5);

	ternary::write_str(&mut trits, "1T0");
	assert_eq!(ternary::read_int(&trits, 3), 6);

	ternary::write_str(&mut trits, "1T1");
	assert_eq!(ternary::read_int(&trits, 3), 7);

	ternary::write_str(&mut trits, "10T");
	assert_eq!(ternary::read_int(&trits, 3), 8);
}

#[test]
fn ternary_get_lst() {
	let mut trits = [Trit::Zero; 6];

	ternary::write_str(&mut trits, "000000");
	assert_eq!(ternary::get_lst(&trits, 6), Trit::Zero);

	ternary::write_str(&mut trits, "0T0010");
	assert_eq!(ternary::get_lst(&trits, 6), Trit::Pos);

	ternary::write_str(&mut trits, "00000T");
	assert_eq!(ternary::get_lst(&trits, 6), Trit::Neg);
}

#[test]
fn ternary_get_mst() {
	let mut trits = [Trit::Zero; 6];

	ternary::write_str(&mut trits, "000000");
	assert_eq!(ternary::get_mst(&trits, 6), Trit::Zero);

	ternary::write_str(&mut trits, "0T0010");
	assert_eq!(ternary::get_mst(&trits, 6), Trit::Neg);

	ternary::write_str(&mut trits, "010000");
	assert_eq!(ternary::get_mst(&trits, 6), Trit::Pos);
}

#[test]
fn ternary_add() {
	let mut a = [Trit::Zero; 6];
	ternary::write_str(&mut a, "0T0001");

	let mut b = [Trit::Zero; 6];
	ternary::write_str(&mut b, "0T00T0");

	let mut c = [Trit::Zero; 6];
	ternary::write_str(&mut c, "T100T1");

	ternary::add1(&mut a, &b);
	assert_eq!(a, c);
}

#[test]
fn ternary_multiply() {
	let mut a = [Trit::Zero; 6];
	ternary::write_str(&mut a, "T01010");

	let mut b = [Trit::Zero; 6];
	ternary::write_str(&mut b, "0001T0");

	let mut c = [Trit::Zero; 12];

	let mut d = [Trit::Zero; 12];
	ternary::write_str(&mut d, "0000T11T1T00");

	ternary::multiply(&mut c, &a, &b);
	assert_eq!(c, d);
}
