use std::iter;
use ternary;
use trit::Trit;

macro_rules! ptr {
	($e:expr) => (&$e[0] as *const _)
}

macro_rules! mut_ptr {
	($e:expr) => (&mut $e[0] as *mut _)
}

#[test]
fn ternary_write_trits() { unsafe {
	let mut trits = [Trit::Zero; 6];
	ternary::copy_from_iter(mut_ptr!(trits), iter::repeat(Trit::Pos).take(6));
	assert_eq!(trits, [Trit::Pos; 6]);
} }

#[test]
fn ternary_write_str() { unsafe {
	let mut trits = [Trit::Zero; 6];
	ternary::write_str(mut_ptr!(trits), "1T00T1");
	assert_eq!(trits, [Trit::Pos, Trit::Neg, Trit::Zero, Trit::Zero, Trit::Neg, Trit::Pos]);
} }

#[test]
fn ternary_write_int() { unsafe {
	let mut trits = [Trit::Zero; 6];
	ternary::write_int(mut_ptr!(trits), 20, 6);
	assert_eq!(trits, [Trit::Neg, Trit::Pos, Trit::Neg, Trit::Pos, Trit::Zero, Trit::Zero]);
} }

#[test]
fn ternary_read_int() { unsafe {
	let mut trits = [Trit::Zero; 3];

	ternary::write_str(mut_ptr!(trits), "00T");
	assert_eq!(ternary::read_int(ptr!(trits), 3), -1);

	ternary::write_str(mut_ptr!(trits), "000");
	assert_eq!(ternary::read_int(ptr!(trits), 3), 0);

	ternary::write_str(mut_ptr!(trits), "001");
	assert_eq!(ternary::read_int(ptr!(trits), 3), 1);

	ternary::write_str(mut_ptr!(trits), "01T");
	assert_eq!(ternary::read_int(ptr!(trits), 3), 2);

	ternary::write_str(mut_ptr!(trits), "010");
	assert_eq!(ternary::read_int(ptr!(trits), 3), 3);

	ternary::write_str(mut_ptr!(trits), "011");
	assert_eq!(ternary::read_int(ptr!(trits), 3), 4);

	ternary::write_str(mut_ptr!(trits), "1TT");
	assert_eq!(ternary::read_int(ptr!(trits), 3), 5);

	ternary::write_str(mut_ptr!(trits), "1T0");
	assert_eq!(ternary::read_int(ptr!(trits), 3), 6);

	ternary::write_str(mut_ptr!(trits), "1T1");
	assert_eq!(ternary::read_int(ptr!(trits), 3), 7);

	ternary::write_str(mut_ptr!(trits), "10T");
	assert_eq!(ternary::read_int(ptr!(trits), 3), 8);
} }

#[test]
fn ternary_get_lst() { unsafe {
	let mut trits = [Trit::Zero; 6];

	ternary::write_str(mut_ptr!(trits), "000000");
	assert_eq!(ternary::get_lst(ptr!(trits), 6), Trit::Zero);

	ternary::write_str(mut_ptr!(trits), "0T0010");
	assert_eq!(ternary::get_lst(ptr!(trits), 6), Trit::Pos);

	ternary::write_str(mut_ptr!(trits), "00000T");
	assert_eq!(ternary::get_lst(ptr!(trits), 6), Trit::Neg);
} }

#[test]
fn ternary_get_mst() { unsafe {
	let mut trits = [Trit::Zero; 6];

	ternary::write_str(mut_ptr!(trits), "000000");
	assert_eq!(ternary::get_mst(ptr!(trits), 6), Trit::Zero);

	ternary::write_str(mut_ptr!(trits), "0T0010");
	assert_eq!(ternary::get_mst(ptr!(trits), 6), Trit::Neg);

	ternary::write_str(mut_ptr!(trits), "010000");
	assert_eq!(ternary::get_mst(ptr!(trits), 6), Trit::Pos);
} }

#[test]
fn ternary_add() { unsafe {
	let mut a = [Trit::Zero; 6];
	ternary::write_str(mut_ptr!(a), "0T0001");

	let mut b = [Trit::Zero; 6];
	ternary::write_str(mut_ptr!(b), "0T00T0");

	let mut c = [Trit::Zero; 6];
	ternary::write_str(mut_ptr!(c), "T100T1");

	ternary::add(mut_ptr!(a), ptr!(a), ptr!(b), 6);
	assert_eq!(a, c);
} }

#[test]
fn ternary_multiply() { unsafe {
	let mut a = [Trit::Zero; 6];
	ternary::write_str(mut_ptr!(a), "T01010");

	let mut b = [Trit::Zero; 6];
	ternary::write_str(mut_ptr!(b), "0001T0");

	let mut c = [Trit::Zero; 12];

	let mut d = [Trit::Zero; 12];
	ternary::write_str(mut_ptr!(d), "0000T11T1T00");

	ternary::multiply(mut_ptr!(c), ptr!(a), ptr!(b), 6);
	assert_eq!(c, d);
} }
