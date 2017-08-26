use std::collections::HashMap;

use ternary;
use types::*;
use opcodes::Opcode;
use registers::Register;
use program::instructions::Instruction;
use program::DecodedProgram;
use program::parser::{CodeDecl, DataDecl};
use util::next_aligned_addr;
use vm::PROGRAM_MAGIC_NUMBER;

static START_LABEL: &str = "__start";

#[derive(Debug)]
pub enum EncodeError {
    InsufficientMemory(usize, usize),
    InvalidLabel(String),
    MissingRequiredLabel(String),
    IntOutOfRange(isize, isize, isize),
}

#[derive(Debug)]
enum Patch {
    Relative(Addr, String),
    Absolute(String),
}

impl Patch {
    fn size(&self) -> isize {
        match *self {
            Patch::Relative(_, _) => HALF_ISIZE,
            Patch::Absolute(_) => WORD_ISIZE,
        }
    }
}

pub struct EncodedProgram<'a> {
    memory: &'a mut [Trit],
    labels: HashMap<String, Addr>,
    patches: HashMap<*mut Trit, Patch>,
    pc: Addr,
}

pub type EncodeResult<T> = Result<T, EncodeError>;

impl<'a> EncodedProgram<'a> {
    pub fn new(memory: &mut [Trit]) -> EncodedProgram {
        EncodedProgram {
            memory: memory,
            labels: HashMap::new(),
            patches: HashMap::new(),
            pc: 0,
        }
    }

    pub fn insert_label(&mut self, label: String) {
        self.pc = next_aligned_addr(self.pc, WORD_SIZE);
        let addr = self.pc;
        self.labels.insert(label, addr);
    }

    pub fn encode(&mut self, program: DecodedProgram) -> EncodeResult<usize> {
        let required_size = program.size();
        if required_size > self.memory.len() {
            return Err(EncodeError::InsufficientMemory(required_size, self.memory.len()));
        }

        ternary::from_int(&mut self.memory[..WORD_SIZE], PROGRAM_MAGIC_NUMBER);
        self.pc += WORD_SIZE;

        // save space for pc start
        let pc_start_offset = self.pc;
        self.pc += WORD_SIZE;

        let _ = self.encode_data_section(&program.data)?;

        self.pc = next_aligned_addr(self.pc, WORD_SIZE);

        let _ = self.encode_code_section(&program.code)?;

        let pc_start = self.labels
            .get(START_LABEL)
            .cloned()
            .ok_or_else(|| EncodeError::MissingRequiredLabel(START_LABEL.to_string()))?;
        
        {
            let local_memory = &mut self.memory[pc_start_offset..][..WORD_SIZE];
            ternary::from_int(local_memory, pc_start as isize);
        }

        self.patch_addrs();

        Ok(self.pc)
    }

    fn patch_addrs(&mut self) {
        for (&ptr, patch) in &self.patches {
            let addr = match *patch {
                Patch::Absolute(ref label) => self.label_addr(label).unwrap() as isize,
                Patch::Relative(pc, ref label) => self.relative_addr(pc, label).unwrap(),
            };

            unsafe {
                use std::slice;
                let slice = slice::from_raw_parts_mut(ptr, patch.size() as usize);
                ternary::from_int(slice, addr);
            };
        }
    }

    pub fn encode_data_section(&mut self, all_data: &[DataDecl]) -> EncodeResult<usize> {
        let mut total_size = 0;

        for data_decl in all_data {
            match *data_decl {
                DataDecl::Label(ref label) => {
                    self.insert_label(label.clone());
                }

                DataDecl::Data(ref data) => {
                    self.pc = next_aligned_addr(self.pc, data.alignment());

                    let size = {
                        let local_memory = &mut self.memory[self.pc..];
                        data.write(local_memory)
                    };

                    total_size += size;
                    self.pc += size;
                }
            }
        }

        Ok(total_size)
    }

    pub fn encode_code_section(&mut self, all_code: &[CodeDecl]) -> EncodeResult<usize> {
        let mut total_size = 0;

        for code_decl in all_code {
            match *code_decl {
                CodeDecl::Label(ref label) => {
                    self.insert_label(label.clone());
                }

                CodeDecl::Instruction(ref instruction) => {
                    assert_eq!(self.pc % WORD_SIZE, 0);

                    let local_memory = self.memory[self.pc..].as_mut_ptr();
                    unsafe {
                        self.encode_instruction(local_memory, instruction)?;
                    }

                    let size = instruction.size();
                    total_size += size;
                    self.pc += size;
                }
            }
        }

        Ok(total_size)
    }

    unsafe fn encode_instruction(&mut self,
                                 memory: *mut Trit,
                                 instruction: &Instruction)
                                 -> EncodeResult<()> {
        ternary::clear(memory, instruction.size() as isize);

        match *instruction {
            Instruction::Mov(r1, r2) => {
                self.encode_opcode(memory, Opcode::Mov)?;
                self.encode_register(tryte_offset!(memory, 1), r1)?;
                self.encode_register(tryte_offset!(memory, 2), r2)?;
            }

            Instruction::Movi(r, half) => {
                self.encode_opcode(memory, Opcode::Movi)?;
                self.encode_register(tryte_offset!(memory, 1), r)?;
                self.encode_half(tryte_offset!(memory, 2), half)?;
            }

            Instruction::Movw(r, word) => {
                self.encode_opcode(memory, Opcode::Movw)?;
                self.encode_register(tryte_offset!(memory, 1), r)?;
                self.encode_word(tryte_offset!(memory, 4), word)?;
            }

            Instruction::Mova(r, ref label) => {
                self.encode_opcode(memory, Opcode::Mova)?;
                self.encode_register(tryte_offset!(memory, 1), r)?;
                self.encode_label(tryte_offset!(memory, 4), label)?;
            }

            Instruction::Lt(r1, r2, offset) => {
                self.encode_opcode(memory, Opcode::Lt)?;
                self.encode_register(tryte_offset!(memory, 1), r1)?;
                self.encode_register(tryte_offset!(memory, 2), r2)?;
                self.encode_tryte(tryte_offset!(memory, 3), offset)?;
            }

            Instruction::Lh(r1, r2, offset) => {
                self.encode_opcode(memory, Opcode::Lh)?;
                self.encode_register(tryte_offset!(memory, 1), r1)?;
                self.encode_register(tryte_offset!(memory, 2), r2)?;
                self.encode_tryte(tryte_offset!(memory, 3), offset)?;
            }

            Instruction::Lw(r1, r2, offset) => {
                self.encode_opcode(memory, Opcode::Lw)?;
                self.encode_register(tryte_offset!(memory, 1), r1)?;
                self.encode_register(tryte_offset!(memory, 2), r2)?;
                self.encode_tryte(tryte_offset!(memory, 3), offset)?;
            }

            Instruction::St(r1, r2, offset) => {
                self.encode_opcode(memory, Opcode::St)?;
                self.encode_register(tryte_offset!(memory, 1), r1)?;
                self.encode_register(tryte_offset!(memory, 2), r2)?;
                self.encode_tryte(tryte_offset!(memory, 3), offset)?;
            }

            Instruction::Sh(r1, r2, offset) => {
                self.encode_opcode(memory, Opcode::Sh)?;
                self.encode_register(tryte_offset!(memory, 1), r1)?;
                self.encode_register(tryte_offset!(memory, 2), r2)?;
                self.encode_tryte(tryte_offset!(memory, 3), offset)?;
            }

            Instruction::Sw(r1, r2, offset) => {
                self.encode_opcode(memory, Opcode::Sw)?;
                self.encode_register(tryte_offset!(memory, 1), r1)?;
                self.encode_register(tryte_offset!(memory, 2), r2)?;
                self.encode_tryte(tryte_offset!(memory, 3), offset)?;
            }

            Instruction::Add(r1, r2, r3) => {
                self.encode_opcode(memory, Opcode::Add)?;
                self.encode_register(tryte_offset!(memory, 1), r1)?;
                self.encode_register(tryte_offset!(memory, 2), r2)?;
                self.encode_register(tryte_offset!(memory, 3), r3)?;
            }

            Instruction::Addi(r, half) => {
                self.encode_opcode(memory, Opcode::Addi)?;
                self.encode_register(tryte_offset!(memory, 1), r)?;
                self.encode_half(tryte_offset!(memory, 2), half)?;
            }

            Instruction::Mul(r1, r2) => {
                self.encode_opcode(memory, Opcode::Mul)?;
                self.encode_register(tryte_offset!(memory, 1), r1)?;
                self.encode_register(tryte_offset!(memory, 2), r2)?;
            }

            Instruction::Muli(r, half) => {
                self.encode_opcode(memory, Opcode::Muli)?;
                self.encode_register(tryte_offset!(memory, 1), r)?;
                self.encode_half(tryte_offset!(memory, 2), half)?;
            }

            Instruction::Not(r1, r2) => {
                self.encode_opcode(memory, Opcode::Not)?;
                self.encode_register(tryte_offset!(memory, 1), r1)?;
                self.encode_register(tryte_offset!(memory, 2), r2)?;
            }

            Instruction::And(r1, r2, r3) => {
                self.encode_opcode(memory, Opcode::And)?;
                self.encode_register(tryte_offset!(memory, 1), r1)?;
                self.encode_register(tryte_offset!(memory, 2), r2)?;
                self.encode_register(tryte_offset!(memory, 3), r3)?;
            }

            Instruction::Andi(r, half) => {
                self.encode_opcode(memory, Opcode::Andi)?;
                self.encode_register(tryte_offset!(memory, 1), r)?;
                self.encode_half(tryte_offset!(memory, 2), half)?;
            }

            Instruction::Or(r1, r2, r3) => {
                self.encode_opcode(memory, Opcode::Or)?;
                self.encode_register(tryte_offset!(memory, 1), r1)?;
                self.encode_register(tryte_offset!(memory, 2), r2)?;
                self.encode_register(tryte_offset!(memory, 3), r3)?;
            }

            Instruction::Ori(r, half) => {
                self.encode_opcode(memory, Opcode::Ori)?;
                self.encode_register(tryte_offset!(memory, 1), r)?;
                self.encode_half(tryte_offset!(memory, 2), half)?;
            }

            Instruction::Shf(r1, r2, r3) => {
                self.encode_opcode(memory, Opcode::Shf)?;
                self.encode_register(tryte_offset!(memory, 1), r1)?;
                self.encode_register(tryte_offset!(memory, 2), r2)?;
                self.encode_register(tryte_offset!(memory, 3), r3)?;
            }

            Instruction::Shfi(r, half) => {
                self.encode_opcode(memory, Opcode::Shfi)?;
                self.encode_register(tryte_offset!(memory, 1), r)?;
                self.encode_half(tryte_offset!(memory, 2), half)?;
            }

            Instruction::Cmp(r1, r2, r3) => {
                self.encode_opcode(memory, Opcode::Cmp)?;
                self.encode_register(tryte_offset!(memory, 1), r1)?;
                self.encode_register(tryte_offset!(memory, 2), r2)?;
                self.encode_register(tryte_offset!(memory, 3), r3)?;
            }

            Instruction::Jmp(ref label) => {
                self.encode_opcode(memory, Opcode::Jmp)?;
                self.encode_label(tryte_offset!(memory, 4), label)?;
            }

            Instruction::JT(r, ref label) => {
                self.encode_opcode(memory, Opcode::JT)?;
                self.encode_register(tryte_offset!(memory, 1), r)?;
                self.encode_relative_label(tryte_offset!(memory, 2), instruction, label)?;
            }

            Instruction::J0(r, ref label) => {
                self.encode_opcode(memory, Opcode::J0)?;
                self.encode_register(tryte_offset!(memory, 1), r)?;
                self.encode_relative_label(tryte_offset!(memory, 2), instruction, label)?;
            }

            Instruction::J1(r, ref label) => {
                self.encode_opcode(memory, Opcode::J1)?;
                self.encode_register(tryte_offset!(memory, 1), r)?;
                self.encode_relative_label(tryte_offset!(memory, 2), instruction, label)?;
            }

            Instruction::JT0(r, ref label) => {
                self.encode_opcode(memory, Opcode::JT0)?;
                self.encode_register(tryte_offset!(memory, 1), r)?;
                self.encode_relative_label(tryte_offset!(memory, 2), instruction, label)?;
            }

            Instruction::JT1(r, ref label) => {
                self.encode_opcode(memory, Opcode::JT1)?;
                self.encode_register(tryte_offset!(memory, 1), r)?;
                self.encode_relative_label(tryte_offset!(memory, 2), instruction, label)?;
            }

            Instruction::J01(r, ref label) => {
                self.encode_opcode(memory, Opcode::J01)?;
                self.encode_register(tryte_offset!(memory, 1), r)?;
                self.encode_relative_label(tryte_offset!(memory, 2), instruction, label)?;
            }

            Instruction::Call(ref label) => {
                self.encode_opcode(memory, Opcode::Call)?;
                self.encode_label(tryte_offset!(memory, 4), label)?;
            }

            Instruction::Ret => {
                self.encode_opcode(memory, Opcode::Ret)?;
            }

            Instruction::Syscall => {
                self.encode_opcode(memory, Opcode::Syscall)?;
            }

            Instruction::Break => {
                self.encode_opcode(memory, Opcode::Break)?;
            }

            Instruction::Halt => {
                self.encode_opcode(memory, Opcode::Halt)?;
            }
        }

        Ok(())
    }

    unsafe fn encode_opcode(&self, memory: *mut Trit, opcode: Opcode) -> EncodeResult<()> {
        use std::slice;
        let slice = slice::from_raw_parts_mut(memory, WORD_SIZE);
        ternary::from_int(slice, opcode as isize);
        Ok(())
    }

    unsafe fn encode_register(&self, memory: *mut Trit, register: Register) -> EncodeResult<()> {
        use std::slice;
        let slice = slice::from_raw_parts_mut(memory, WORD_SIZE);
        ternary::from_int(slice, register as isize);
        Ok(())
    }

    unsafe fn encode_tryte(&self, memory: *mut Trit, tryte: Tryte) -> EncodeResult<()> {
        ternary::copy(memory, ptr!(tryte), TRYTE_ISIZE);
        
        Ok(())
    }

    unsafe fn encode_half(&self, memory: *mut Trit, half: Half) -> EncodeResult<()> {
        ternary::copy(memory, ptr!(half), HALF_ISIZE);
        Ok(())
    }

    unsafe fn encode_word(&self, memory: *mut Trit, word: Word) -> EncodeResult<()> {
        ternary::copy(memory, ptr!(word), WORD_ISIZE);
        Ok(())
    }

    unsafe fn encode_label(&mut self, memory: *mut Trit, label: &str) -> EncodeResult<()> {
        self.patches.insert(memory, Patch::Absolute(label.to_string()));
        Ok(())
    }

    unsafe fn encode_relative_label(&mut self,
                                    memory: *mut Trit,
                                    instruction: &Instruction,
                                    label: &str)
                                    -> EncodeResult<()> {
        let pc = self.pc + instruction.size();
        self.patches.insert(memory, Patch::Relative(pc, label.to_string()));
        Ok(())
    }

    pub fn label_addr(&self, label: &str) -> EncodeResult<Addr> {
        match self.labels.get(label) {
            Some(&addr) => Ok(addr),
            _ => Err(EncodeError::InvalidLabel(label.to_string())),
        }
    }

    pub fn relative_addr(&self, pc: Addr, label: &str) -> EncodeResult<RelAddr> {
        let addr = self.label_addr(label)?;
        Ok(addr as RelAddr - pc as RelAddr)
    }
}
