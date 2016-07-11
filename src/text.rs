use std::char;

use trit::Trit;
use ternary;
use types::*;

// single: 0ttttt
// double: 10tttt Tttttt
// triple: 110ttt Tttttt Tttttt

const SINGLE_TRITS: usize = 5;
const DOUBLE_TRITS: usize = 9;
const TRIPLE_TRITS: usize = 13;

const DOUBLE_START_TRITS: usize = 4;
const TRIPLE_START_TRITS: usize = 3;
const CONTINUATION_TRITS: usize = 5;

const SINGLE_RANGE: usize = 243;
const DOUBLE_RANGE: usize = 19_683;
const TRIPLE_RANGE: usize = 1_594_323;

const SINGLE_MIN: usize = 0;
const SINGLE_MAX: usize = SINGLE_MIN + SINGLE_RANGE - 1;

const DOUBLE_MIN: usize = SINGLE_MAX + 1;
const DOUBLE_MAX: usize = DOUBLE_MIN + DOUBLE_RANGE - 1;

const TRIPLE_MIN: usize = DOUBLE_MAX + 1;
const TRIPLE_MAX: usize = TRIPLE_MIN + TRIPLE_RANGE - 1;

const DOUBLE_START: Tryte = [Trit::Zero, Trit::Zero, Trit::Zero, Trit::Zero, Trit::Zero, Trit::Pos];
const TRIPLE_START: Tryte = [Trit::Zero, Trit::Zero, Trit::Zero, Trit::Zero, Trit::Pos, Trit::Pos];
const CONTINUATION: Tryte = [Trit::Zero, Trit::Zero, Trit::Zero, Trit::Zero, Trit::Pos, Trit::Neg];

pub enum CharTrytes {
	Single(Tryte),
	Double(Tryte, Tryte),
	Triple(Tryte, Tryte, Tryte),
	Invalid,
}

unsafe fn char_to_trytes(c: char) -> CharTrytes {
	let i = c as usize;
	let mut word = EMPTY_WORD;
	ternary::write_int(mut_ptr!(word), i as isize, WORD_ISIZE);

	match i {
		SINGLE_MIN...SINGLE_MAX => {
			let mut t0 = EMPTY_TRYTE;
			ternary::copy(mut_ptr!(t0), ptr!(word), SINGLE_TRITS as isize);
			CharTrytes::Single(t0)
		}

		DOUBLE_MIN...DOUBLE_MAX => {
			let mut t0 = DOUBLE_START;
			let mut t1 = CONTINUATION;

			ternary::copy(mut_ptr!(t0), ptr!(word), DOUBLE_START_TRITS as isize);
			ternary::copy(mut_ptr!(t1), tryte_ptr!(word, 1), CONTINUATION_TRITS as isize);

			CharTrytes::Double(t0, t1)
		}

		TRIPLE_MIN...TRIPLE_MAX => {
			let mut t0 = DOUBLE_START;
			let mut t1 = CONTINUATION;
			let mut t2 = CONTINUATION;

			ternary::copy(mut_ptr!(t0), ptr!(word), DOUBLE_START_TRITS as isize);
			ternary::copy(mut_ptr!(t1), tryte_ptr!(word, 1), CONTINUATION_TRITS as isize);
			ternary::copy(mut_ptr!(t2), tryte_ptr!(word, 2), CONTINUATION_TRITS as isize);

			CharTrytes::Triple(t0, t1, t2)
		}

		_ => CharTrytes::Invalid
	}
}

unsafe fn trytes_to_char(trits: *const Trit) -> (char, usize) {
	let mut word = EMPTY_WORD;

	let high_trit = *trits.offset(5);
	let next_high_trit = *trits.offset(4);
	let len = match (high_trit, next_high_trit) {
		(Trit::Zero, _) => {
			ternary::copy(mut_ptr!(word), trits, SINGLE_TRITS as isize);
			1
		}

		(Trit::Pos, Trit::Zero) => {
			assert_eq!(*trits.offset(11), Trit::Neg);

			let offset = DOUBLE_START_TRITS as isize;
			ternary::copy(mut_ptr!(word), trits, DOUBLE_START_TRITS as isize);
			ternary::copy(mut_ptr!(word).offset(offset), tryte_offset!(trits, 1), CONTINUATION_TRITS as isize);

			2
		}

		(Trit::Pos, Trit::Pos) => {
			assert_eq!(*trits.offset(11), Trit::Neg);
			assert_eq!(*trits.offset(17), Trit::Neg);

			let offset = DOUBLE_START_TRITS as isize;
			let offset2 = offset + CONTINUATION_TRITS as isize;
			ternary::copy(mut_ptr!(word), trits, DOUBLE_START_TRITS as isize);
			ternary::copy(mut_ptr!(word).offset(offset), tryte_offset!(trits, 1), CONTINUATION_TRITS as isize);
			ternary::copy(mut_ptr!(word).offset(offset2), tryte_offset!(trits, 2), CONTINUATION_TRITS as isize);

			3
		}

		_ => panic!("invalid sequence"),
	};

	let i = ternary::read_int(ptr!(word), WORD_ISIZE) as u32;
	let c = char::from_u32(i).unwrap_or(char::REPLACEMENT_CHARACTER);
	(c, len)
}
