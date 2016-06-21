extern crate core;

mod trit;
mod ternary;
mod types;
mod ops;
mod vm;

pub use trit::*;
pub use types::*;

#[cfg(test)]
mod test;
