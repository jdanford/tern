use rand;
use std::process;

use types::*;
use ternary;
use registers::Register;
use vm::VM;
use text;
use util;

#[derive(Debug)]
pub enum Syscall {
    PrintString = 0,
    PrintDecimal = 1,
    PrintTernary = 2,
    GetRand = 3,
    Exit = 4,
}

impl Syscall {
    pub fn perform(self, vm: &mut VM) {
        match self {
            Syscall::PrintString => {
                let addr = vm.read(Register::A0);
                let local_memory = &vm.memory[addr as usize..];
                let (s, _) = text::decode_str(local_memory);
                print!("{}", s);
            }

            Syscall::PrintDecimal => {
                let n = vm.read(Register::A0);
                print!("{}", n);
            }

            Syscall::PrintTernary => {
                let src = vm.src(Register::A0);
                print!("{}", ternary::to_str(src));
            }

            Syscall::GetRand => {
                let dest = vm.dest(Register::A0);
                let mut rng = rand::thread_rng();
                util::random_word(&mut dest[..WORD_SIZE], &mut rng);
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
        match i {
            0 => Syscall::PrintString,
            1 => Syscall::PrintDecimal,
            2 => Syscall::PrintTernary,
            3 => Syscall::GetRand,
            4 => Syscall::Exit,
            _ => unimplemented!()
        }
    }
}
