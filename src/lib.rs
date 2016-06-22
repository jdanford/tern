#![feature(libc)]

extern crate core;
extern crate libc;

pub mod trit;
pub mod ternary;
pub mod types;
pub mod inst;
pub mod vm;

#[cfg(test)]
mod test;
