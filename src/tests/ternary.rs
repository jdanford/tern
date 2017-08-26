use std::iter::repeat;

use ternary;
use trit::Trit::*;
use types::*;

#[test]
fn ternary_write_trits() {
    let mut trits = EMPTY_TRYTE;
    ternary::copy_from_iter(&mut trits, repeat(Pos).take(TRYTE_SIZE));
    assert_eq!(trits, [Pos; TRYTE_SIZE]);
}

#[test]
fn ternary_from_str() {
    let mut trits = EMPTY_TRYTE;
    ternary::from_str(&mut trits, "1T00T1");
    assert_eq!(trits, [Pos, Neg, Zero, Zero, Neg, Pos]);
}

#[test]
fn ternary_from_int() {
    let mut trits = EMPTY_TRYTE;
    ternary::from_int(&mut trits, 20);
    assert_eq!(trits, [Neg, Pos, Neg, Pos, Zero, Zero]);
}

#[test]
fn ternary_to_int() {
    let trits = &mut [Zero; 3];

    ternary::from_str(trits, "00T");
    assert_eq!(ternary::to_int(trits), -1);

    ternary::from_str(trits, "000");
    assert_eq!(ternary::to_int(trits), 0);

    ternary::from_str(trits, "001");
    assert_eq!(ternary::to_int(trits), 1);

    ternary::from_str(trits, "01T");
    assert_eq!(ternary::to_int(trits), 2);

    ternary::from_str(trits, "010");
    assert_eq!(ternary::to_int(trits), 3);

    ternary::from_str(trits, "011");
    assert_eq!(ternary::to_int(trits), 4);

    ternary::from_str(trits, "1TT");
    assert_eq!(ternary::to_int(trits), 5);

    ternary::from_str(trits, "1T0");
    assert_eq!(ternary::to_int(trits), 6);

    ternary::from_str(trits, "1T1");
    assert_eq!(ternary::to_int(trits), 7);

    ternary::from_str(trits, "10T");
    assert_eq!(ternary::to_int(trits), 8);
}

#[test]
fn ternary_copy_blocks() {
    unsafe {
        let trits = [Neg, Pos, Neg, Pos, Neg, Pos];
        let mut block1 = EMPTY_TRYTE;
        let mut block2 = EMPTY_TRYTE;
        let mut block3 = EMPTY_TRYTE;

        let blocks = vec![
            (mut_ptr!(block1), 6),
            (mut_ptr!(block2), 6),
            (mut_ptr!(block3), 6),
        ];
        ternary::copy_blocks(ptr!(trits), TRYTE_SIZE, 4, blocks);

        let block1_expected = [Zero, Zero, Zero, Zero, Neg, Pos];
        let block2_expected = [Neg, Pos, Neg, Pos, Zero, Zero];

        assert_eq!(block1, block1_expected);
        assert_eq!(block2, block2_expected);
    }
}

#[test]
fn ternary_add() {
    unsafe {
        let mut a = EMPTY_TRYTE;
        ternary::from_str(&mut a, "0T0001");

        let mut b = EMPTY_TRYTE;
        ternary::from_str(&mut b, "0T00T0");

        let mut c = EMPTY_TRYTE;
        ternary::from_str(&mut c, "T100T1");

        ternary::add(mut_ptr!(a), ptr!(a), ptr!(b), TRYTE_ISIZE);
        assert_eq!(a, c);
    }
}

#[test]
fn ternary_multiply() {
    unsafe {
        let mut a = EMPTY_TRYTE;
        ternary::from_str(&mut a, "T01010");

        let mut b = EMPTY_TRYTE;
        ternary::from_str(&mut b, "0001T0");

        let mut c = EMPTY_HALF;

        let mut d = EMPTY_HALF;
        ternary::from_str(&mut d, "0000T11T1T00");

        ternary::multiply(mut_ptr!(c), ptr!(a), ptr!(b), TRYTE_ISIZE);
        assert_eq!(c, d);
    }
}

#[test]
fn ternary_lowest_trit() {
    let trits = &mut EMPTY_TRYTE;

    ternary::from_str(trits, "000000");
    assert_eq!(ternary::lowest_trit(trits), Zero);

    ternary::from_str(trits, "0T0010");
    assert_eq!(ternary::lowest_trit(trits), Pos);

    ternary::from_str(trits, "00000T");
    assert_eq!(ternary::lowest_trit(trits), Neg);
}

#[test]
fn ternary_highest_trit() {
    let trits = &mut EMPTY_TRYTE;

    ternary::from_str(trits, "000000");
    assert_eq!(ternary::highest_trit(trits), Zero);

    ternary::from_str(trits, "0T0010");
    assert_eq!(ternary::highest_trit(trits), Neg);

    ternary::from_str(trits, "010000");
    assert_eq!(ternary::highest_trit(trits), Pos);
}

#[test]
fn ternary_popcount() {
    let a = &mut EMPTY_TRYTE;
    ternary::from_str(a, "000000");

    let b = &mut EMPTY_TRYTE;
    ternary::from_str(b, "1T11T1");

    assert_eq!(ternary::popcount(a), (0, 0));
    assert_eq!(ternary::popcount(b), (4, 2));
}
