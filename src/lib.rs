#![feature(libc)]

extern crate core;
extern crate libc;

macro_rules! ptr {
	($e:expr) => (&$e[0] as *const _)
}

macro_rules! mut_ptr {
	($e:expr) => (&mut $e[0] as *mut _)
}

macro_rules! tryte_ptr {
	($e:expr,$n:expr) => (ptr!($e).offset(TRYTE_ISIZE * $n))
}

pub mod trit;
pub mod ternary;
pub mod types;
pub mod opcodes;
pub mod registers;
pub mod vm;

#[cfg(test)]
mod test;
