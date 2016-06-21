use std::ptr::copy_nonoverlapping;
use types::*;
use vm::VM;

#[allow(non_snake_case)]
impl VM {
	fn mov(&mut self, dest: &mut Word, src: &Word) {
		unsafe { copy_nonoverlapping(src, dest, WORD_SIZE) }
	}

	// fn movw(&mut self, dest: &mut Word, src: &Word) {
	// 	self.mov(dest, src)
	// }

	// fn ld(&mut self, dest: &mut Word, src: &Word) {}

	// fn ldr(&mut self, dest: &mut Word, src: &Word) {}

	// fn st(&mut self, dest: &mut Word, src: &Word) {}

	// fn str(&mut self, dest: &mut Word, src: &Word) {}

	// fn add(&mut self, dest: &mut Word, lhs: &Word, rhs: &Word) {}

	// fn addi(&mut self, dest: &mut Word, src: &Immediate) {}

	// fn mul(&mut self, dest: &mut Word, lhs: &Word, rhs: &Word) {}

	// fn muli(&mut self, dest: &mut Word, src: &Immediate) {}

	// fn not(&mut self, dest: &mut Word, src: &Word) {}

	// fn and(&mut self, dest: &mut Word, lhs: &Word, rhs: &Word) {}

	// fn andi(&mut self, dest: &mut Word, src: &Immediate) {}

	// fn or(&mut self, dest: &mut Word, lhs: &Word, rhs: &Word) {}

	// fn ori(&mut self, dest: &mut Word, src: &Immediate) {}

	// fn shf(&mut self, dest: &mut Word, lhs: &Word, rhs: &Word) {}

	// fn shfi(&mut self, dest: &mut Word, src: &Immediate) {}

	// fn cmp(&mut self, dest: &mut Word, lhs: &Word, rhs: &Word) {}

	// fn jmp(&mut self, dest: &mut Word) {}

	// fn jr(&mut self, dest: &mut Word) {}

	// fn jT(&mut self, dest: &mut Word, shift: usize) {}

	// fn j0(&mut self, dest: &mut Word, shift: usize) {}

	// fn j1(&mut self, dest: &mut Word, shift: usize) {}

	// fn jT0(&mut self, dest: &mut Word, shift: usize) {}

	// fn jT1(&mut self, dest: &mut Word, shift: usize) {}

	// fn j01(&mut self, dest: &mut Word, shift: usize) {}

	// fn call(&mut self, dest: &mut Word) {}

	// fn callr(&mut self, dest: &mut Word) {}

	// fn ret(&mut self) {}

	// fn sys(&mut self) {}
}
