use std::iter::repeat;

use ternary;
use trit::Trit;
use types::*;

#[test]
fn ternary_write_trits() { unsafe {
    let mut trits = EMPTY_TRYTE;
    ternary::copy_from_iter(mut_ptr!(trits), repeat(Trit::Pos).take(6));
    assert_eq!(trits, [Trit::Pos; 6]);
} }

#[test]
fn ternary_from_str() { unsafe {
    let mut trits = EMPTY_TRYTE;
    ternary::from_str(mut_ptr!(trits), "1T00T1");
    assert_eq!(trits, [Trit::Pos, Trit::Neg, Trit::Zero, Trit::Zero, Trit::Neg, Trit::Pos]);
} }

#[test]
fn ternary_from_int() { unsafe {
    let mut trits = EMPTY_TRYTE;
    ternary::from_int(mut_ptr!(trits), 20, 6);
    assert_eq!(trits, [Trit::Neg, Trit::Pos, Trit::Neg, Trit::Pos, Trit::Zero, Trit::Zero]);
} }

#[test]
fn ternary_to_int() { unsafe {
    let mut trits = [Trit::Zero; 3];

    ternary::from_str(mut_ptr!(trits), "00T");
    assert_eq!(ternary::to_int(ptr!(trits), 3), -1);

    ternary::from_str(mut_ptr!(trits), "000");
    assert_eq!(ternary::to_int(ptr!(trits), 3), 0);

    ternary::from_str(mut_ptr!(trits), "001");
    assert_eq!(ternary::to_int(ptr!(trits), 3), 1);

    ternary::from_str(mut_ptr!(trits), "01T");
    assert_eq!(ternary::to_int(ptr!(trits), 3), 2);

    ternary::from_str(mut_ptr!(trits), "010");
    assert_eq!(ternary::to_int(ptr!(trits), 3), 3);

    ternary::from_str(mut_ptr!(trits), "011");
    assert_eq!(ternary::to_int(ptr!(trits), 3), 4);

    ternary::from_str(mut_ptr!(trits), "1TT");
    assert_eq!(ternary::to_int(ptr!(trits), 3), 5);

    ternary::from_str(mut_ptr!(trits), "1T0");
    assert_eq!(ternary::to_int(ptr!(trits), 3), 6);

    ternary::from_str(mut_ptr!(trits), "1T1");
    assert_eq!(ternary::to_int(ptr!(trits), 3), 7);

    ternary::from_str(mut_ptr!(trits), "10T");
    assert_eq!(ternary::to_int(ptr!(trits), 3), 8);
} }

#[test]
fn ternary_copy_blocks() { unsafe {
    let trits = [Trit::Neg, Trit::Pos, Trit::Neg, Trit::Pos, Trit::Neg, Trit::Pos];
    let mut block1 = [Trit::Zero; 6];
    let mut block2 = [Trit::Zero; 6];
    let mut block3 = [Trit::Zero; 6];

    ternary::copy_blocks(ptr!(trits), 6, 4, vec![
        (mut_ptr!(block1), 6),
        (mut_ptr!(block2), 6),
        (mut_ptr!(block3), 6),
    ]);

    let block1_expected = [Trit::Zero, Trit::Zero, Trit::Zero, Trit::Zero, Trit::Neg, Trit::Pos];
    let block2_expected = [Trit::Neg, Trit::Pos, Trit::Neg, Trit::Pos, Trit::Zero, Trit::Zero];

    assert_eq!(block1, block1_expected);
    assert_eq!(block2, block2_expected);
} }

#[test]
fn ternary_get_lst() { unsafe {
    let mut trits = EMPTY_TRYTE;

    ternary::from_str(mut_ptr!(trits), "000000");
    assert_eq!(ternary::get_lst(ptr!(trits), 6), Trit::Zero);

    ternary::from_str(mut_ptr!(trits), "0T0010");
    assert_eq!(ternary::get_lst(ptr!(trits), 6), Trit::Pos);

    ternary::from_str(mut_ptr!(trits), "00000T");
    assert_eq!(ternary::get_lst(ptr!(trits), 6), Trit::Neg);
} }

#[test]
fn ternary_get_mst() { unsafe {
    let mut trits = EMPTY_TRYTE;

    ternary::from_str(mut_ptr!(trits), "000000");
    assert_eq!(ternary::get_mst(ptr!(trits), 6), Trit::Zero);

    ternary::from_str(mut_ptr!(trits), "0T0010");
    assert_eq!(ternary::get_mst(ptr!(trits), 6), Trit::Neg);

    ternary::from_str(mut_ptr!(trits), "010000");
    assert_eq!(ternary::get_mst(ptr!(trits), 6), Trit::Pos);
} }

#[test]
fn ternary_add() { unsafe {
    let mut a = EMPTY_TRYTE;
    ternary::from_str(mut_ptr!(a), "0T0001");

    let mut b = EMPTY_TRYTE;
    ternary::from_str(mut_ptr!(b), "0T00T0");

    let mut c = EMPTY_TRYTE;
    ternary::from_str(mut_ptr!(c), "T100T1");

    ternary::add(mut_ptr!(a), ptr!(a), ptr!(b), 6);
    assert_eq!(a, c);
} }

#[test]
fn ternary_multiply() { unsafe {
    let mut a = EMPTY_TRYTE;
    ternary::from_str(mut_ptr!(a), "T01010");

    let mut b = EMPTY_TRYTE;
    ternary::from_str(mut_ptr!(b), "0001T0");

    let mut c = EMPTY_HALF;

    let mut d = EMPTY_HALF;
    ternary::from_str(mut_ptr!(d), "0000T11T1T00");

    ternary::multiply(mut_ptr!(c), ptr!(a), ptr!(b), 6);
    assert_eq!(c, d);
} }
