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
pub mod vm;
pub mod static_data;
pub mod parser;
pub mod program;
pub mod encoder;
pub mod text;
mod util;

#[cfg(test)]
mod test;
