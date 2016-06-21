use trit::Trit;

pub const TRYTE_SIZE: usize = 6;
pub const IMMEDIATE_SIZE: usize = 12;
pub const WORD_SIZE: usize = 24;

pub type Tryte = [Trit; TRYTE_SIZE];
pub type Immediate = [Trit; IMMEDIATE_SIZE];
pub type Word = [Trit; WORD_SIZE];
