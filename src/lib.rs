#![feature(libc)]

extern crate core;
extern crate libc;

macro_rules! ptr {
	($e:expr) => (&$e[0] as *const _)
}

macro_rules! mut_ptr {
	($e:expr) => (&mut $e[0] as *mut _)
}

macro_rules! tryte_offset {
	($e:expr,$n:expr) => ($e.offset(TRYTE_ISIZE * $n))
}

macro_rules! tryte_ptr {
	($e:expr,$n:expr) => (tryte_offset!(ptr!($e), $n))
}

pub mod trit;
pub mod ternary;
pub mod types;
pub mod opcodes;
pub mod registers;
pub mod vm;

#[cfg(test)]
mod test;
