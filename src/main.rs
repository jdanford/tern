extern crate tern;

use std::mem::size_of;
use tern::Trit::*;
use tern::ternary::*;

fn main() {
	let t4 = Ternary { trits: [Neg, Zero, Zero, Pos] };
	println!("t4 = {}", t4);
	println!("sizeof(t4) = {}", size_of::<Ternary4>());
}
