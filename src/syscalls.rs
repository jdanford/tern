use std::mem::transmute;

#[repr(isize)]
#[derive(Debug)]
pub enum Syscall {
    PrintLine = 0,
    Exit = 1,
}

impl From<isize> for Syscall {
    fn from(i: isize) -> Syscall {
        unsafe { transmute(i) }
    }
}
