#![feature(libc)]

extern crate core;
extern crate libc;

#[macro_use]
extern crate lazy_static;

mod trit;
mod ternary;
mod types;
mod inst;
mod vm;

pub use trit::*;
pub use types::*;

#[cfg(test)]
mod test;
