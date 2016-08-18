pub use trit::Trit;

pub const TRYTE_MIN: isize = -364;
pub const TRYTE_MAX: isize = 364;

pub const HALF_MIN: isize = -265_720;
pub const HALF_MAX: isize = 265_720;

pub const WORD_MIN: isize = -141_214_768_240;
pub const WORD_MAX: isize = 141_214_768_240;

pub const TRYTE_SIZE: usize = 6;
pub const HALF_SIZE: usize = 12;
pub const WORD_SIZE: usize = 24;

pub const TRYTE_ISIZE: isize = TRYTE_SIZE as isize;
pub const HALF_ISIZE: isize = HALF_SIZE as isize;
pub const WORD_ISIZE: isize = WORD_SIZE as isize;

pub type Tryte = [Trit; TRYTE_SIZE];
pub type Half = [Trit; HALF_SIZE];
pub type Word = [Trit; WORD_SIZE];

pub const EMPTY_TRYTE: Tryte = [Trit::Zero; TRYTE_SIZE];
pub const EMPTY_HALF: Half = [Trit::Zero; HALF_SIZE];
pub const EMPTY_WORD: Word = [Trit::Zero; WORD_SIZE];

pub type Addr = usize;
pub type RelAddr = isize;
