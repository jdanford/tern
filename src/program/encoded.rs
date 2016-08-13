use std::collections::HashMap;

use trit::Trit;
use ternary;
use types::*;
use opcodes::Opcode;
use registers::Register;
use program::instructions::Instruction;
use program::DecodedProgram;
use program::parser::{CodeDecl, DataDecl};
use util::next_aligned_addr;
use vm::PROGRAM_MAGIC_NUMBER;

#[derive(Debug)]
pub enum EncodeError {
    InsufficientMemory(usize, usize),
    InvalidLabel(String),
    IntOutOfRange(isize, isize, isize),
}

#[derive(Debug)]
enum Patch {
    Relative(Addr, String),
    Absolute(String),
}

pub struct EncodedProgram {
    memory: *mut Trit,
    memory_size: usize,
    labels: HashMap<String, Addr>,
    patches: HashMap<*mut Trit, Patch>,
    pc: Addr,
}

impl EncodedProgram {
    pub fn new(memory: *mut Trit, memory_size: usize) -> EncodedProgram {
        EncodedProgram {
            memory: memory,
            memory_size: memory_size,
            labels: HashMap::new(),
            patches: HashMap::new(),
            pc: 0,
        }
    }

    pub fn insert_label(&mut self, label: &String) {
        self.pc = next_aligned_addr(self.pc, WORD_SIZE);
        let addr = self.pc;
        self.labels.insert(label.clone(), addr);
    }

    pub fn encode(&mut self, program: DecodedProgram) -> Result<usize, EncodeError> {
        let required_size = program.size();
        if required_size > self.memory_size {
            return Err(EncodeError::InsufficientMemory(required_size, self.memory_size))
        }

        unsafe { ternary::from_int(self.memory, PROGRAM_MAGIC_NUMBER, WORD_ISIZE) };
        self.pc += WORD_SIZE;

        // save space for pc start
        let pc_start_offset = self.pc;
        self.pc += WORD_SIZE;

        let _ = try!(self.encode_data_section(&program.data[..]));

        self.pc = next_aligned_addr(self.pc, WORD_SIZE);
        let pc_start = self.pc;

        let _ = try!(self.encode_code_section(&program.code[..]));

        unsafe {
            let local_memory = self.memory.offset(pc_start_offset as isize);
            ternary::from_int(local_memory, pc_start as isize, WORD_ISIZE);
        }

        self.patch_addrs();

        Ok(self.pc)
    }

    fn patch_addrs(&mut self) {
        for (&ptr, ref patch) in self.patches.iter() {
            let addr = match **patch {
                Patch::Absolute(ref label) => self.label_addr(label).unwrap() as isize,
                Patch::Relative(pc, ref label) => self.relative_addr(pc, label).unwrap(),
            };

            unsafe { ternary::from_int(ptr, addr, WORD_ISIZE) };
        }
    }

    pub fn encode_data_section(&mut self, all_data: &[DataDecl]) -> Result<usize, EncodeError> {
        let mut total_size = 0;

        for ref data_decl in all_data {
            match **data_decl {
                DataDecl::Label(ref label) => {
                    self.insert_label(label);
                }

                DataDecl::Data(ref data) => {
                    self.pc = next_aligned_addr(self.pc, data.alignment());

                    let size = unsafe {
                        let local_memory = self.memory.offset(self.pc as isize);
                        data.write(local_memory)
                    };

                    total_size += size;
                    self.pc += size;
                }
            }
        }

        Ok(total_size)
    }

    pub fn encode_code_section(&mut self, all_code: &[CodeDecl]) -> Result<usize, EncodeError> {
        let mut total_size = 0;

        for ref code_decl in all_code {
            match **code_decl {
                CodeDecl::Label(ref label) => {
                    self.insert_label(label);
                }

                CodeDecl::Instruction(ref instruction) => {
                    assert_eq!(self.pc % WORD_SIZE, 0);

                    unsafe {
                        let local_memory = self.memory.offset(self.pc as isize);
                        try!(self.encode_instruction(local_memory, instruction));
                    }

                    let size = instruction.size();
                    total_size += size;
                    self.pc += size;
                }
            }
        }

        Ok(total_size)
    }

    unsafe fn encode_instruction(&mut self, memory: *mut Trit, instruction: &Instruction) -> Result<(), EncodeError> {
        ternary::clear(memory, instruction.size() as isize);

        match *instruction {
            Instruction::Mov(r1, r2) => {
                try!(self.encode_opcode(memory, Opcode::Mov));
                try!(self.encode_register(tryte_offset!(memory, 1), r1));
                try!(self.encode_register(tryte_offset!(memory, 2), r2));
            }

            Instruction::Movi(r, half) => {
                try!(self.encode_opcode(memory, Opcode::Movi));
                try!(self.encode_register(tryte_offset!(memory, 1), r));
                try!(self.encode_half(tryte_offset!(memory, 2), half));
            }

            Instruction::Movw(r, word) => {
                try!(self.encode_opcode(memory, Opcode::Movw));
                try!(self.encode_register(tryte_offset!(memory, 1), r));
                try!(self.encode_word(tryte_offset!(memory, 4), word));
            }

            Instruction::Mova(r, ref label) => {
                try!(self.encode_opcode(memory, Opcode::Mova));
                try!(self.encode_register(tryte_offset!(memory, 1), r));
                try!(self.encode_label(tryte_offset!(memory, 4), label));
            }

            Instruction::Lb(r1, r2, offset) => {
                try!(self.encode_opcode(memory, Opcode::Lb));
                try!(self.encode_register(tryte_offset!(memory, 1), r1));
                try!(self.encode_register(tryte_offset!(memory, 2), r2));
                try!(self.encode_tryte(tryte_offset!(memory, 3), offset));
            }

            Instruction::Lh(r1, r2, offset) => {
                try!(self.encode_opcode(memory, Opcode::Lh));
                try!(self.encode_register(tryte_offset!(memory, 1), r1));
                try!(self.encode_register(tryte_offset!(memory, 2), r2));
                try!(self.encode_tryte(tryte_offset!(memory, 3), offset));
            }

            Instruction::Lw(r1, r2, offset) => {
                try!(self.encode_opcode(memory, Opcode::Lw));
                try!(self.encode_register(tryte_offset!(memory, 1), r1));
                try!(self.encode_register(tryte_offset!(memory, 2), r2));
                try!(self.encode_tryte(tryte_offset!(memory, 3), offset));
            }

            Instruction::Sb(r1, r2, offset) => {
                try!(self.encode_opcode(memory, Opcode::Sb));
                try!(self.encode_register(tryte_offset!(memory, 1), r1));
                try!(self.encode_register(tryte_offset!(memory, 2), r2));
                try!(self.encode_tryte(tryte_offset!(memory, 3), offset));
            }

            Instruction::Sh(r1, r2, offset) => {
                try!(self.encode_opcode(memory, Opcode::Sh));
                try!(self.encode_register(tryte_offset!(memory, 1), r1));
                try!(self.encode_register(tryte_offset!(memory, 2), r2));
                try!(self.encode_tryte(tryte_offset!(memory, 3), offset));
            }

            Instruction::Sw(r1, r2, offset) => {
                try!(self.encode_opcode(memory, Opcode::Sw));
                try!(self.encode_register(tryte_offset!(memory, 1), r1));
                try!(self.encode_register(tryte_offset!(memory, 2), r2));
                try!(self.encode_tryte(tryte_offset!(memory, 3), offset));
            }

            Instruction::Add(r1, r2, r3) => {
                try!(self.encode_opcode(memory, Opcode::Add));
                try!(self.encode_register(tryte_offset!(memory, 1), r1));
                try!(self.encode_register(tryte_offset!(memory, 2), r2));
                try!(self.encode_register(tryte_offset!(memory, 3), r3));
            }

            Instruction::Addi(r, half) => {
                try!(self.encode_opcode(memory, Opcode::Addi));
                try!(self.encode_register(tryte_offset!(memory, 1), r));
                try!(self.encode_half(tryte_offset!(memory, 2), half));
            }

            Instruction::Mul(r1, r2) => {
                try!(self.encode_opcode(memory, Opcode::Mul));
                try!(self.encode_register(tryte_offset!(memory, 1), r1));
                try!(self.encode_register(tryte_offset!(memory, 2), r2));
            }

            Instruction::Muli(r, half) => {
                try!(self.encode_opcode(memory, Opcode::Muli));
                try!(self.encode_register(tryte_offset!(memory, 1), r));
                try!(self.encode_half(tryte_offset!(memory, 2), half));
            }

            Instruction::Not(r1, r2) => {
                try!(self.encode_opcode(memory, Opcode::Not));
                try!(self.encode_register(tryte_offset!(memory, 1), r1));
                try!(self.encode_register(tryte_offset!(memory, 2), r2));
            }

            Instruction::And(r1, r2, r3) => {
                try!(self.encode_opcode(memory, Opcode::And));
                try!(self.encode_register(tryte_offset!(memory, 1), r1));
                try!(self.encode_register(tryte_offset!(memory, 2), r2));
                try!(self.encode_register(tryte_offset!(memory, 3), r3));
            }

            Instruction::Andi(r, half) => {
                try!(self.encode_opcode(memory, Opcode::Andi));
                try!(self.encode_register(tryte_offset!(memory, 1), r));
                try!(self.encode_half(tryte_offset!(memory, 2), half));
            }

            Instruction::Or(r1, r2, r3) => {
                try!(self.encode_opcode(memory, Opcode::Or));
                try!(self.encode_register(tryte_offset!(memory, 1), r1));
                try!(self.encode_register(tryte_offset!(memory, 2), r2));
                try!(self.encode_register(tryte_offset!(memory, 3), r3));
            }

            Instruction::Ori(r, half) => {
                try!(self.encode_opcode(memory, Opcode::Ori));
                try!(self.encode_register(tryte_offset!(memory, 1), r));
                try!(self.encode_half(tryte_offset!(memory, 2), half));
            }

            Instruction::Shf(r1, r2, r3) => {
                try!(self.encode_opcode(memory, Opcode::Shf));
                try!(self.encode_register(tryte_offset!(memory, 1), r1));
                try!(self.encode_register(tryte_offset!(memory, 2), r2));
                try!(self.encode_register(tryte_offset!(memory, 3), r3));
            }

            Instruction::Shfi(r, half) => {
                try!(self.encode_opcode(memory, Opcode::Shfi));
                try!(self.encode_register(tryte_offset!(memory, 1), r));
                try!(self.encode_half(tryte_offset!(memory, 2), half));
            }

            Instruction::Cmp(r1, r2, r3) => {
                try!(self.encode_opcode(memory, Opcode::Cmp));
                try!(self.encode_register(tryte_offset!(memory, 1), r1));
                try!(self.encode_register(tryte_offset!(memory, 2), r2));
                try!(self.encode_register(tryte_offset!(memory, 3), r3));
            }

            Instruction::Jmp(ref label) => {
                try!(self.encode_opcode(memory, Opcode::Jmp));
                try!(self.encode_label(tryte_offset!(memory, 4), label));
            }

            Instruction::JT(r, ref label) => {
                try!(self.encode_opcode(memory, Opcode::JT));
                try!(self.encode_register(tryte_offset!(memory, 1), r));
                try!(self.encode_relative_label(tryte_offset!(memory, 2), instruction, label));
            }

            Instruction::J0(r, ref label) => {
                try!(self.encode_opcode(memory, Opcode::J0));
                try!(self.encode_register(tryte_offset!(memory, 1), r));
                try!(self.encode_relative_label(tryte_offset!(memory, 2), instruction, label));
            }

            Instruction::J1(r, ref label) => {
                try!(self.encode_opcode(memory, Opcode::J1));
                try!(self.encode_register(tryte_offset!(memory, 1), r));
                try!(self.encode_relative_label(tryte_offset!(memory, 2), instruction, label));
            }

            Instruction::JT0(r, ref label) => {
                try!(self.encode_opcode(memory, Opcode::JT0));
                try!(self.encode_register(tryte_offset!(memory, 1), r));
                try!(self.encode_relative_label(tryte_offset!(memory, 2), instruction, label));
            }

            Instruction::JT1(r, ref label) => {
                try!(self.encode_opcode(memory, Opcode::JT1));
                try!(self.encode_register(tryte_offset!(memory, 1), r));
                try!(self.encode_relative_label(tryte_offset!(memory, 2), instruction, label));
            }

            Instruction::J01(r, ref label) => {
                try!(self.encode_opcode(memory, Opcode::J01));
                try!(self.encode_register(tryte_offset!(memory, 1), r));
                try!(self.encode_relative_label(tryte_offset!(memory, 2), instruction, label));
            }

            Instruction::Call(ref label) => {
                try!(self.encode_opcode(memory, Opcode::Call));
                try!(self.encode_label(tryte_offset!(memory, 4), label));
            }

            Instruction::Ret => {
                try!(self.encode_opcode(memory, Opcode::Ret));
            }

            Instruction::Syscall => {
                try!(self.encode_opcode(memory, Opcode::Syscall));
            }

            Instruction::Break => {
                try!(self.encode_opcode(memory, Opcode::Break));
            }

            Instruction::Halt => {
                try!(self.encode_opcode(memory, Opcode::Halt));
            }
        }

        Ok(())
    }

    unsafe fn encode_opcode(&self, memory: *mut Trit, opcode: Opcode) -> Result<(), EncodeError> {
        ternary::from_int(memory, opcode as isize, WORD_ISIZE);
        Ok(())
    }

    unsafe fn encode_register(&self, memory: *mut Trit, register: Register) -> Result<(), EncodeError> {
        ternary::from_int(memory, register as isize, WORD_ISIZE);
        Ok(())
    }

    unsafe fn encode_tryte(&self, memory: *mut Trit, tryte: Tryte) -> Result<(), EncodeError> {
        ternary::copy(memory, ptr!(tryte), TRYTE_ISIZE);
        Ok(())
    }

    unsafe fn encode_half(&self, memory: *mut Trit, half: Half) -> Result<(), EncodeError> {
        ternary::copy(memory, ptr!(half), HALF_ISIZE);
        Ok(())
    }

    unsafe fn encode_word(&self, memory: *mut Trit, word: Word) -> Result<(), EncodeError> {
        ternary::copy(memory, ptr!(word), WORD_ISIZE);
        Ok(())
    }

    unsafe fn encode_label(&mut self, memory: *mut Trit, label: &String) -> Result<(), EncodeError> {
        self.patches.insert(memory, Patch::Absolute(label.clone()));
        Ok(())
    }

    unsafe fn encode_relative_label(&mut self, memory: *mut Trit, instruction: &Instruction, label: &String) -> Result<(), EncodeError> {
        self.patches.insert(memory, Patch::Relative(self.pc + instruction.size(), label.clone()));
        Ok(())
    }

    pub fn label_addr(&self, label: &String) -> Result<Addr, EncodeError> {
        match self.labels.get(label) {
            Some(&addr) => Ok(addr),
            _ => Err(EncodeError::InvalidLabel(label.clone()))
        }
    }

    pub fn relative_addr(&self, pc: Addr, label: &String) -> Result<RelAddr, EncodeError> {
        let addr = try!(self.label_addr(label));
        Ok(addr as RelAddr - pc as RelAddr)
    }
}
