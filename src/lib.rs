#![feature(libc)]

extern crate core;
extern crate libc;
extern crate regex;
extern crate rlibc;

#[macro_use] mod macros;
pub mod trit;
pub mod ternary;
pub mod types;
pub mod opcodes;
pub mod registers;
pub mod instructions;
pub mod reader;
pub mod vm;

#[cfg(test)]
mod test;
