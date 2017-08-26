use std::char;

use trit::Trit::*;
use ternary;
use types::*;

// UTF-6t
// single: 0ttttt
// double: 10tttt Tttttt
// triple: 110ttt Tttttt Tttttt

const SINGLE_TRITS: usize = 5;
const DOUBLE_START_TRITS: usize = 4;
const TRIPLE_START_TRITS: usize = 3;
const CONTINUATION_TRITS: usize = 5;

const SINGLE_RANGE: usize = 243;
const DOUBLE_RANGE: usize = 19_683;
const TRIPLE_RANGE: usize = 1_594_323;

const SINGLE_OFFSET: isize = (SINGLE_RANGE as isize - 1) / 2;
const DOUBLE_OFFSET: isize = (DOUBLE_RANGE as isize - 1) / 2;
const TRIPLE_OFFSET: isize = (TRIPLE_RANGE as isize - 1) / 2;

const SINGLE_MIN: usize = 0;
const SINGLE_MAX: usize = SINGLE_MIN + SINGLE_RANGE - 1;

const DOUBLE_MIN: usize = SINGLE_MAX + 1;
const DOUBLE_MAX: usize = DOUBLE_MIN + DOUBLE_RANGE - 1;

const TRIPLE_MIN: usize = DOUBLE_MAX + 1;
const TRIPLE_MAX: usize = TRIPLE_MIN + TRIPLE_RANGE - 1;

static DOUBLE_START: Tryte = [Zero, Zero, Zero, Zero, Zero, Pos];
static TRIPLE_START: Tryte = [Zero, Zero, Zero, Zero, Pos, Pos];
static CONTINUATION: Tryte = [Zero, Zero, Zero, Zero, Pos, Neg];

pub fn encode_str(trits: &mut [Trit], s: &str) -> usize {
    let mut i = 0;
    for c in s.chars() {
        let offset = i * TRYTE_SIZE + WORD_SIZE;
        i += encode_char(&mut trits[offset..], c);
    }

    ternary::from_int(&mut trits[..WORD_SIZE], i as isize);

    i
}

pub fn decode_str(trits: &[Trit]) -> (String, usize) {
    let len = ternary::to_int(&trits[..WORD_SIZE]);
    let mut s = String::new();

    let mut i = 0;
    while (i as isize) < len {
        let offset = i * TRYTE_SIZE + WORD_SIZE;
        let (c, j) = decode_char(&trits[offset..]);
        s.push(c);
        i += j;
    }

    (s, i)
}

pub fn encode_char(trits: &mut [Trit], c: char) -> usize {
    let codepoint = c as u32;
    let mut word = EMPTY_WORD;

    let (len, codepoint_offset) = match codepoint as usize {
        SINGLE_MIN...SINGLE_MAX => (1, SINGLE_OFFSET),
        DOUBLE_MIN...DOUBLE_MAX => (2, DOUBLE_OFFSET),
        TRIPLE_MIN...TRIPLE_MAX => (3, TRIPLE_OFFSET),
        _ => (0, 0),
    };

    if len > 0 {
        let shifted_codepoint = shift_codepoint(codepoint, codepoint_offset);
        ternary::from_int(&mut word, shifted_codepoint as isize);
    }

    match len {
        1 => {
            trits[..SINGLE_TRITS].copy_from_slice(&word[..SINGLE_TRITS]);
        }

        2 => {
            trits[..TRYTE_SIZE].copy_from_slice(&DOUBLE_START);
            trits[..DOUBLE_START_TRITS].copy_from_slice(&word[..DOUBLE_START_TRITS]);

            let offset = DOUBLE_START_TRITS;
            trits[TRYTE_SIZE..][..TRYTE_SIZE].copy_from_slice(&CONTINUATION);
            trits[TRYTE_SIZE..][..CONTINUATION_TRITS].copy_from_slice(&word[offset..][..CONTINUATION_TRITS]);
        }

        3 => {
            trits[..TRYTE_SIZE].copy_from_slice(&TRIPLE_START);
            trits[..TRIPLE_START_TRITS].copy_from_slice(&word[..TRIPLE_START_TRITS]);

            let offset1 = TRIPLE_START_TRITS;
            trits[TRYTE_SIZE..][..TRYTE_SIZE].copy_from_slice(&CONTINUATION);
            trits[TRYTE_SIZE..][..CONTINUATION_TRITS].copy_from_slice(&word[offset1..][..CONTINUATION_TRITS]);

            let offset2 = offset1 + CONTINUATION_TRITS;
            trits[2*TRYTE_SIZE..][..TRYTE_SIZE].copy_from_slice(&CONTINUATION);
            trits[2*TRYTE_SIZE..][..CONTINUATION_TRITS].copy_from_slice(&word[offset2..][..CONTINUATION_TRITS]);
        }

        _ => {
            encode_char(trits, char::REPLACEMENT_CHARACTER);
        }
    }

    len
}

pub fn decode_char(trits: &[Trit]) -> (char, usize) {
    let word = &mut EMPTY_WORD;
    let invalid_result = (char::REPLACEMENT_CHARACTER, 1);

    let high_trit = trits[5];
    let next_high_trit = trits[4];
    let (codepoint_offset, len) = match (high_trit, next_high_trit) {
        (Trit::Zero, _) => {
            word[..SINGLE_TRITS].copy_from_slice(&trits[..SINGLE_TRITS]);
            (SINGLE_OFFSET, 1)
        }

        (Trit::Pos, Trit::Zero) => {
            if trits[11] != Trit::Neg {
                return invalid_result;
            }
            
            let len = DOUBLE_START_TRITS;
            word[..len].copy_from_slice(&trits[..len]);

            let offset = DOUBLE_START_TRITS;
            let len = CONTINUATION_TRITS;
            word[offset..][..len].copy_from_slice(&trits[TRYTE_SIZE..][..len]);

            (DOUBLE_OFFSET, 2)
        }

        (Trit::Pos, Trit::Pos) => {
            if trits[11] != Trit::Neg || trits[17] != Trit::Neg {
                return invalid_result;
            }
            
            let len = TRIPLE_START_TRITS;
            word[..len].copy_from_slice(&trits[..len]);
            
            let offset = len;
            let len = CONTINUATION_TRITS;
            word[offset..][..len].copy_from_slice(&trits[TRYTE_SIZE..][..len]);
            
            let offset = offset + len;
            //let len = CONTINUATION_TRITS;
            word[offset..][..len].copy_from_slice(&trits[2*TRYTE_SIZE..][..len]);

            (TRIPLE_OFFSET, 3)
        }

        _ => {
            return invalid_result;
        }
    };

    let shifted_codepoint = ternary::to_int(word) as i32;
    let codepoint = unshift_codepoint(shifted_codepoint, codepoint_offset);
    let c = char::from_u32(codepoint).unwrap_or(char::REPLACEMENT_CHARACTER);
    (c, len)
}

fn shift_codepoint(codepoint: u32, offset: isize) -> i32 {
    (codepoint as i32).wrapping_sub(offset as i32)
}

fn unshift_codepoint(shifted_codepoint: i32, offset: isize) -> u32 {
    shifted_codepoint.wrapping_add(offset as i32) as u32
}
