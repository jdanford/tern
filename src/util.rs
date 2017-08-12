use rand::Rng;
use std::io::prelude::*;

use ternary;
use types::*;
use vm::VM;
use program::DecodedProgram;
use program::EncodedProgram;

pub fn next_aligned_addr(addr: Addr, alignment: usize) -> Addr {
    let rem = addr % alignment;
    if rem == 0 {
        addr
    } else {
        addr - rem + alignment
    }
}

pub fn random_word<R: Rng>(trits: *mut Trit, rng: &mut R, len: isize) {
    unsafe { ternary::clear(trits, len) };

    for (i, trit) in rng.gen_iter().take(len as usize).enumerate() {
        unsafe { *trits.offset(i as isize) = trit };
    }
}

pub fn vm_from_code(code: &str) -> Result<VM, String> {
    let mut program = DecodedProgram::new();
    program.read_str(code).map_err(|e| format!("{:?}", e))?;

    let mut vm = VM::new(program.size());

    let mut encoder = EncodedProgram::new(vm.memory.as_mut_ptr(), vm.memory.len());
    encoder.encode(program).map_err(|e| format!("{:?}", e))?;

    Ok(vm)
}

pub fn vm_from_reader<R: Read>(reader: R) -> Result<VM, String> {
    let mut program = DecodedProgram::new();
    program.read(reader).map_err(|e| format!("{:?}", e))?;

    let mut vm = VM::new(program.size());

    let mut encoder = EncodedProgram::new(vm.memory.as_mut_ptr(), vm.memory.len());
    encoder.encode(program).map_err(|e| format!("{:?}", e))?;

    Ok(vm)
}
