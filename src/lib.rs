extern crate core;
extern crate libc;
extern crate rand;
extern crate regex;

#[macro_use]
mod macros;
pub mod trit;
pub mod ternary;
pub mod types;
pub mod opcodes;
pub mod registers;
pub mod syscalls;
pub mod vm;
pub mod program;
pub mod text;
pub mod util;

#[cfg(test)]
mod tests;
