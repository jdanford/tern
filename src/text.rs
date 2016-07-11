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
// const DOUBLE_TRITS: usize = DOUBLE_START_TRITS + CONTINUATION_TRITS;
// const TRIPLE_TRITS: usize = TRIPLE_START_TRITS + CONTINUATION_TRITS * 2;

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

const DOUBLE_START: Tryte = [Trit::Zero, Trit::Zero, Trit::Zero, Trit::Zero, Trit::Zero, Trit::Pos];
const TRIPLE_START: Tryte = [Trit::Zero, Trit::Zero, Trit::Zero, Trit::Zero, Trit::Pos, Trit::Pos];
const CONTINUATION: Tryte = [Trit::Zero, Trit::Zero, Trit::Zero, Trit::Zero, Trit::Pos, Trit::Neg];

#[derive(Debug)]
pub enum CharTrytes {
	Single(Tryte),
	Double(Tryte, Tryte),
	Triple(Tryte, Tryte, Tryte),
	Invalid,
}

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

pub unsafe fn char_to_trytes(c: char) -> CharTrytes {
	let codepoint = c as u32;
	let mut word = EMPTY_WORD;

	match codepoint as usize {
		SINGLE_MIN...SINGLE_MAX => {
			let mut t0 = EMPTY_TRYTE;

			let codepoint_offset = SINGLE_OFFSET;
			let shifted_codepoint = shift_codepoint(codepoint, codepoint_offset);
			ternary::write_int(mut_ptr!(word), shifted_codepoint as isize, WORD_ISIZE);
			ternary::copy(mut_ptr!(t0), ptr!(word), SINGLE_TRITS as isize);

			CharTrytes::Single(t0)
		}

		DOUBLE_MIN...DOUBLE_MAX => {
			let mut t0 = DOUBLE_START;
			let mut t1 = CONTINUATION;

			let codepoint_offset = DOUBLE_OFFSET;
			let shifted_codepoint = shift_codepoint(codepoint, codepoint_offset);
			ternary::write_int(mut_ptr!(word), shifted_codepoint as isize, WORD_ISIZE);
			ternary::copy(mut_ptr!(t0), ptr!(word), DOUBLE_START_TRITS as isize);
			let offset = DOUBLE_START_TRITS as isize;
			ternary::copy(mut_ptr!(t1), ptr!(word).offset(offset), CONTINUATION_TRITS as isize);

			CharTrytes::Double(t0, t1)
		}

		TRIPLE_MIN...TRIPLE_MAX => {
			let mut t0 = TRIPLE_START;
			let mut t1 = CONTINUATION;
			let mut t2 = CONTINUATION;

			let codepoint_offset = TRIPLE_OFFSET;
			let shifted_codepoint = shift_codepoint(codepoint, codepoint_offset);
			ternary::write_int(mut_ptr!(word), shifted_codepoint as isize, WORD_ISIZE);
			ternary::copy(mut_ptr!(t0), ptr!(word), TRIPLE_START_TRITS as isize);
			let offset1 = TRIPLE_START_TRITS as isize;
			ternary::copy(mut_ptr!(t1), ptr!(word).offset(offset1), CONTINUATION_TRITS as isize);
			let offset2 = offset1 + CONTINUATION_TRITS as isize;
			ternary::copy(mut_ptr!(t2), ptr!(word).offset(offset2), CONTINUATION_TRITS as isize);

			CharTrytes::Triple(t0, t1, t2)
		}

		_ => CharTrytes::Invalid
	}
}

pub unsafe fn encode_char(trits: *mut Trit, c: char) -> usize {
	match char_to_trytes(c) {
		CharTrytes::Single(t0) => {
			ternary::copy(trits, ptr!(t0), TRYTE_ISIZE);
			1
		}

		CharTrytes::Double(t0, t1) => {
			ternary::copy(tryte_offset!(trits, 0), ptr!(t0), TRYTE_ISIZE);
			ternary::copy(tryte_offset!(trits, 1), ptr!(t1), TRYTE_ISIZE);
			2
		}

		CharTrytes::Triple(t0, t1, t2) => {
			ternary::copy(tryte_offset!(trits, 0), ptr!(t0), TRYTE_ISIZE);
			ternary::copy(tryte_offset!(trits, 1), ptr!(t1), TRYTE_ISIZE);
			ternary::copy(tryte_offset!(trits, 2), ptr!(t2), TRYTE_ISIZE);
			3
		}

		CharTrytes::Invalid => {
			0
		}
	}
}

pub unsafe fn decode_char(trits: *const Trit) -> (char, usize) {
	let mut word = EMPTY_WORD;

	let high_trit = *trits.offset(5);
	let next_high_trit = *trits.offset(4);
	let (codepoint_offset, len) = match (high_trit, next_high_trit) {
		(Trit::Zero, _) => {
			ternary::copy(mut_ptr!(word), trits, SINGLE_TRITS as isize);
			(SINGLE_OFFSET, 1)
		}

		(Trit::Pos, Trit::Zero) => {
			assert_eq!(*trits.offset(11), Trit::Neg);

			ternary::copy(mut_ptr!(word), tryte_offset!(trits, 0), DOUBLE_START_TRITS as isize);
			let offset = DOUBLE_START_TRITS as isize;
			ternary::copy(mut_ptr!(word).offset(offset), tryte_offset!(trits, 1), CONTINUATION_TRITS as isize);

			(DOUBLE_OFFSET, 2)
		}

		(Trit::Pos, Trit::Pos) => {
			assert_eq!(*trits.offset(11), Trit::Neg);
			assert_eq!(*trits.offset(17), Trit::Neg);

			ternary::copy(mut_ptr!(word), tryte_offset!(trits, 0), TRIPLE_START_TRITS as isize);
			let offset1 = TRIPLE_START_TRITS as isize;
			ternary::copy(mut_ptr!(word).offset(offset1), tryte_offset!(trits, 1), CONTINUATION_TRITS as isize);
			let offset2 = offset1 + CONTINUATION_TRITS as isize;
			ternary::copy(mut_ptr!(word).offset(offset2), tryte_offset!(trits, 2), CONTINUATION_TRITS as isize);

			(TRIPLE_OFFSET, 3)
		}

		_ => {
			panic!("invalid tryte")
		}
	};

	let shifted_codepoint = ternary::read_int(ptr!(word), WORD_ISIZE) as i32;
	let codepoint = unshift_codepoint(shifted_codepoint, codepoint_offset);

	if len > 1 {
		println!("{} + {} = {}", shifted_codepoint, codepoint_offset, codepoint);
	}

	let c = char::from_u32(codepoint).unwrap_or(char::REPLACEMENT_CHARACTER);
	(c, len)
}

fn shift_codepoint(codepoint: u32, offset: isize) -> i32 {
	(codepoint as i32).wrapping_sub(offset as i32)
}

fn unshift_codepoint(shifted_codepoint: i32, offset: isize) -> u32 {
	shifted_codepoint.wrapping_add(offset as i32) as u32
}
