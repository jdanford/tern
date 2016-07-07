use trit::Trit;

pub const TRYTE_SIZE: usize = 6;
pub const HALFWORD_SIZE: usize = 12;
pub const WORD_SIZE: usize = 24;

pub const TRYTE_ISIZE: isize = TRYTE_SIZE as isize;
pub const HALFWORD_ISIZE: isize = HALFWORD_SIZE as isize;
pub const WORD_ISIZE: isize = WORD_SIZE as isize;

pub type Tryte = [Trit; TRYTE_SIZE];
pub type Halfword = [Trit; HALFWORD_SIZE];
pub type Word = [Trit; WORD_SIZE];

pub const EMPTY_TRYTE: Tryte = [Trit::Zero; TRYTE_SIZE];
pub const EMPTY_HALFWORD: Halfword = [Trit::Zero; HALFWORD_SIZE];
pub const EMPTY_WORD: Word = [Trit::Zero; WORD_SIZE];

pub type Addr = usize;
pub type RelAddr = isize;
pub type Label = String;
