use std::mem::transmute;
use std::process;

use types::*;
use ternary;
use registers::Register;
use vm::VM;
use text;

#[derive(Debug)]
pub enum Syscall {
    PrintString = 0,
    PrintDecimal = 1,
    PrintTernary = 2,
    Exit = 3,
}

impl Syscall {
    pub unsafe fn perform(self, vm: &mut VM) {
        match self {
            Syscall::PrintString => {
                let addr = vm.read(Register::A0);
                let local_memory = vm.memory.offset(addr);
                let (s, _) = text::decode_str(local_memory);
                println!("{}", s);
            }

            Syscall::PrintDecimal => {
                let n = vm.read(Register::A0);
                println!("{}", n);
            }

            Syscall::PrintTernary => {
                let src = vm.src(Register::A0);
                println!("{}", ternary::to_str(src, WORD_ISIZE));
            }

            Syscall::Exit => {
                let code = vm.read(Register::A0) as i32;
                process::exit(code);
            }
        }
    }
}

impl From<isize> for Syscall {
    fn from(i: isize) -> Syscall {
        unsafe { transmute(i as u8) }
    }
}
