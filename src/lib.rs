extern crate core;

pub mod trit;
pub mod ternary;

pub use trit::*;
pub use ternary::*;

#[cfg(test)]
mod test;
