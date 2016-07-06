#![feature(libc)]

extern crate core;
extern crate libc;
extern crate rlibc;

extern crate combine;

#[macro_use] mod macros;
pub mod trit;
pub mod ternary;
pub mod types;
pub mod opcodes;
pub mod registers;
pub mod instructions;
pub mod parser;
pub mod vm;

#[cfg(test)]
mod test;
