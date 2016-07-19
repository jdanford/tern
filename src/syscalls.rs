use std::mem::transmute;
use std::process;

use registers::Register;
use vm::VM;
use text;

#[derive(Debug)]
pub enum Syscall {
    PrintLine = 0,
    Exit = 1,
}

impl Syscall {
    pub unsafe fn perform(self, vm: &mut VM) {
        match self {
            Syscall::PrintLine => {
                let addr = vm.read(Register::A0);
                let local_memory = vm.memory.offset(addr);
                let (s, _) = text::decode_str(local_memory);
                println!("{}", s);
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
