use std::char;

use trit::Trit;
use ternary;
use types::*;

// UTF-12t
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

static DOUBLE_START: Tryte = [Trit::Zero, Trit::Zero, Trit::Zero, Trit::Zero, Trit::Zero, Trit::Pos];
static TRIPLE_START: Tryte = [Trit::Zero, Trit::Zero, Trit::Zero, Trit::Zero, Trit::Pos, Trit::Pos];
static CONTINUATION: Tryte = [Trit::Zero, Trit::Zero, Trit::Zero, Trit::Zero, Trit::Pos, Trit::Neg];

pub fn encode_str(trits: *mut Trit, s: &str) -> usize {
	let mut i = 0;
	for c in s.chars() {
		let offset = (i * TRYTE_SIZE + WORD_SIZE) as isize;
		i += unsafe { encode_char(trits.offset(offset), c) };
	}

	unsafe { ternary::write_int(trits, i as isize, WORD_ISIZE) };

	i
}

pub fn decode_str(trits: *const Trit) -> (String, usize) {
	let len = unsafe { ternary::read_int(trits, WORD_ISIZE) };
	let mut s = String::new();

	let mut i = 0;
	while (i as isize) < len {
		let offset = (i * TRYTE_SIZE + WORD_SIZE) as isize;
		let (c, j) = unsafe { decode_char(trits.offset(offset)) };
		s.push(c);
		i += j;
	}

	(s, i)
}

pub unsafe fn encode_char(trits: *mut Trit, c: char) -> usize {
	let codepoint = c as u32;
	let mut word = EMPTY_WORD;

	let (len, codepoint_offset) = match codepoint as usize {
		SINGLE_MIN...SINGLE_MAX => (1, SINGLE_OFFSET),
		DOUBLE_MIN...DOUBLE_MAX => (2, DOUBLE_OFFSET),
		TRIPLE_MIN...TRIPLE_MAX => (3, TRIPLE_OFFSET),
		_ => (0, 0),
	};

	let shifted_codepoint = shift_codepoint(codepoint, codepoint_offset);
	ternary::write_int(mut_ptr!(word), shifted_codepoint as isize, WORD_ISIZE);

	match len {
		1 => {
			ternary::copy(tryte_offset!(trits, 0), ptr!(word), SINGLE_TRITS as isize);
		}

		2 => {
			ternary::copy(tryte_offset!(trits, 0), ptr!(DOUBLE_START), TRYTE_ISIZE);
			ternary::copy(tryte_offset!(trits, 0), ptr!(word), DOUBLE_START_TRITS as isize);

			let offset = DOUBLE_START_TRITS as isize;
			ternary::copy(tryte_offset!(trits, 1), ptr!(CONTINUATION), TRYTE_ISIZE);
			ternary::copy(tryte_offset!(trits, 1), ptr!(word).offset(offset), CONTINUATION_TRITS as isize);
		}

		3 => {
			ternary::copy(tryte_offset!(trits, 0), ptr!(TRIPLE_START), TRYTE_ISIZE);
			ternary::copy(tryte_offset!(trits, 0), ptr!(word), TRIPLE_START_TRITS as isize);

			let offset1 = TRIPLE_START_TRITS as isize;
			ternary::copy(tryte_offset!(trits, 1), ptr!(CONTINUATION), TRYTE_ISIZE);
			ternary::copy(tryte_offset!(trits, 1), ptr!(word).offset(offset1), CONTINUATION_TRITS as isize);

			let offset2 = offset1 + CONTINUATION_TRITS as isize;
			ternary::copy(tryte_offset!(trits, 2), ptr!(CONTINUATION), TRYTE_ISIZE);
			ternary::copy(tryte_offset!(trits, 2), ptr!(word).offset(offset2), CONTINUATION_TRITS as isize);
		}

		_ => {
			encode_char(trits, char::REPLACEMENT_CHARACTER);
		}
	}

	len
}

pub unsafe fn decode_char(trits: *const Trit) -> (char, usize) {
	let mut word = EMPTY_WORD;
	let invalid_result = (char::REPLACEMENT_CHARACTER, 1);

	let high_trit = *trits.offset(5);
	let next_high_trit = *trits.offset(4);
	let (codepoint_offset, len) = match (high_trit, next_high_trit) {
		(Trit::Zero, _) => {
			ternary::copy(mut_ptr!(word), trits, SINGLE_TRITS as isize);
			(SINGLE_OFFSET, 1)
		}

		(Trit::Pos, Trit::Zero) => {
			if *trits.offset(11) != Trit::Neg {
				return invalid_result;
			}

			ternary::copy(mut_ptr!(word), tryte_offset!(trits, 0), DOUBLE_START_TRITS as isize);

			let offset = DOUBLE_START_TRITS as isize;
			ternary::copy(mut_ptr!(word).offset(offset), tryte_offset!(trits, 1), CONTINUATION_TRITS as isize);

			(DOUBLE_OFFSET, 2)
		}

		(Trit::Pos, Trit::Pos) => {
			if *trits.offset(11) != Trit::Neg || *trits.offset(17) != Trit::Neg {
				return invalid_result;
			}

			ternary::copy(mut_ptr!(word), tryte_offset!(trits, 0), TRIPLE_START_TRITS as isize);

			let offset1 = TRIPLE_START_TRITS as isize;
			ternary::copy(mut_ptr!(word).offset(offset1), tryte_offset!(trits, 1), CONTINUATION_TRITS as isize);

			let offset2 = offset1 + CONTINUATION_TRITS as isize;
			ternary::copy(mut_ptr!(word).offset(offset2), tryte_offset!(trits, 2), CONTINUATION_TRITS as isize);

			(TRIPLE_OFFSET, 3)
		}

		_ => {
			return invalid_result;
		}
	};

	let shifted_codepoint = ternary::read_int(ptr!(word), WORD_ISIZE) as i32;
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
