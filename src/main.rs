extern crate tern;

use std::mem::size_of;
use tern::Trit::*;
use tern::ternary::*;

fn main() {
	let t6 = Ternary { trits: [Neg, Zero, Zero, Pos, Pos, Neg] };
	println!("t6 = {}", t6);
	println!("sizeof(t6) = {}", size_of::<Ternary6>());
}
