use trit::Trit;
use ternary;
use types::*;
use text;

#[derive(Clone, Debug)]
pub enum StaticData {
	Tryte(isize),
	Half(isize),
	Word(isize),
	String(String),
}

#[derive(Debug)]
pub enum StaticDataError {

}

impl StaticData {
	pub fn size(&self) -> usize {
		match *self {
			StaticData::Tryte(_) => TRYTE_SIZE,
			StaticData::Half(_) => HALF_SIZE,
			StaticData::Word(_) => WORD_SIZE,
			StaticData::String(ref s) => WORD_SIZE + s.len() * TRYTE_SIZE,
		}
	}

	pub fn alignment(&self) -> usize {
		match *self {
			StaticData::Tryte(_) => TRYTE_SIZE,
			StaticData::Half(_) => HALF_SIZE,
			_ => WORD_SIZE,
		}
	}

	pub unsafe fn write(&self, memory: *mut Trit) -> usize {
		match *self {
			StaticData::Tryte(i) => {
				ternary::from_int(memory, i, TRYTE_ISIZE);
				TRYTE_SIZE
			}

			StaticData::Half(i) => {
				ternary::from_int(memory, i, HALF_ISIZE);
				HALF_SIZE
			}

			StaticData::Word(i) => {
				ternary::from_int(memory, i, WORD_ISIZE);
				WORD_SIZE
			}

			StaticData::String(ref s) => {
				text::encode_str(memory, &s[..])
			}
		}
	}
}
