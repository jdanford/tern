use trit::Trit;

pub const TRYTE_SIZE: usize = 6;
pub const IMMEDIATE_SIZE: usize = 12;
pub const WORD_SIZE: usize = 24;

pub const TRYTE_ISIZE: isize = TRYTE_SIZE as isize;
pub const IMMEDIATE_ISIZE: isize = IMMEDIATE_SIZE as isize;
pub const WORD_ISIZE: isize = WORD_SIZE as isize;

pub type Tryte = [Trit; TRYTE_SIZE];
pub type Immediate = [Trit; IMMEDIATE_SIZE];
pub type Word = [Trit; WORD_SIZE];
