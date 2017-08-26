use ternary;
use types::*;
use text;

#[derive(Clone, Debug)]
pub enum StaticData {
    Tryte(isize),
    Half(isize),
    Word(isize),
    String(String),
    Array(Box<StaticData>, usize),
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
            StaticData::Array(ref data, count) => data.size() * count,
        }
    }

    pub fn alignment(&self) -> usize {
        match *self {
            StaticData::Tryte(_) => TRYTE_SIZE,
            StaticData::Half(_) => HALF_SIZE,
            _ => WORD_SIZE,
        }
    }

    pub fn write(&self, memory: &mut [Trit]) -> usize {
        match *self {
            StaticData::Tryte(i) => {
                ternary::from_int(&mut memory[..TRYTE_SIZE], i);
                TRYTE_SIZE
            }

            StaticData::Half(i) => {
                ternary::from_int(&mut memory[..HALF_SIZE], i);
                HALF_SIZE
            }

            StaticData::Word(i) => {
                ternary::from_int(&mut memory[..WORD_SIZE], i);
                WORD_SIZE
            }
            
            StaticData::String(ref s) => text::encode_str(memory, s) * TRYTE_SIZE + WORD_SIZE,

            StaticData::Array(ref data, count) => {
                for _ in 0..count {
                    data.write(memory);
                }

                data.size() * count
            }
        }
    }
}
