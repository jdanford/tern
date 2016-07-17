use libc::{malloc, free};
use std::mem::transmute;

use trit::Trit;
use ternary;
use types::*;
use opcodes::Opcode;
use registers::{Register, REGISTER_COUNT};
use syscalls::Syscall;

pub const PROGRAM_MAGIC_NUMBER: isize = 49425168884; // 1TTT1TTT1TTT1TTT1TTT1TTT

pub struct VM {
    pub registers: [Word; REGISTER_COUNT],
    pub memory: *mut Trit,
    pub memory_size: usize,
    pub pc: Addr,
    pub running: bool
}

impl VM {
    pub fn new(memory_size: usize) -> VM {
        let registers = [[Trit::Zero; WORD_SIZE]; REGISTER_COUNT];
        let memory = unsafe {
            let ptr = malloc(memory_size);
            transmute(ptr)
        };

        unsafe { ternary::clear(memory, memory_size as isize) };

        VM {
            registers: registers,
            memory: memory,
            memory_size: memory_size,
            pc: 0,
            running: false
        }
    }

    fn src(&self, r: Register) -> *const Trit {
        ptr!(self.registers[r as usize])
    }

    fn dest(&mut self, r: Register) -> *mut Trit {
        mut_ptr!(self.registers[r as usize])
    }

    pub fn read(&mut self, r: Register) -> isize {
        unsafe { ternary::to_int(self.src(r), WORD_ISIZE) }
    }

    pub fn write(&mut self, r: Register, value: isize) {
        unsafe { ternary::from_int(self.dest(r), value, WORD_ISIZE); }
    }

    pub fn clear(&mut self, r: Register) {
        unsafe { ternary::clear(self.dest(r), WORD_ISIZE) }
    }

    pub fn init(&mut self) {
        let magic_number = unsafe { ternary::to_int(self.memory, WORD_ISIZE) };
        assert_eq!(magic_number, PROGRAM_MAGIC_NUMBER);

        let pc_start = unsafe { ternary::to_int(self.memory.offset(WORD_ISIZE), WORD_ISIZE) } as Addr;
        self.pc = pc_start;

        self.running = true;
    }

    pub fn run(&mut self) {
        self.init();

        while self.running {
            unsafe { self.step() };
        }
    }

    unsafe fn next_inst(&mut self) -> Word {
        let mut inst = [Trit::Zero; WORD_SIZE];
        let location = self.memory.offset(self.pc as isize);
        ternary::copy(mut_ptr!(inst), location, WORD_ISIZE);

        self.pc += WORD_SIZE;
        inst
    }

    pub unsafe fn step(&mut self) {
        let inst = self.next_inst();
        let (t0, t1, t2, t3) = ternary::read_trytes(ptr!(inst));
        let opcode = Opcode::from(t0);

        match opcode {
            Opcode::Mov => {
                self.mov(Register::from(t1), Register::from(t2));
            }

            Opcode::Movi => {
                let half = inst_half(inst);
                self.movi(Register::from(t1), half);
            }

            Opcode::Movw => {
                let word = self.next_inst();
                self.movw(Register::from(t1), word);
            }

            Opcode::Mova => {
                let inst = self.next_inst();
                let addr = inst_addr(inst);
                self.mova(Register::from(t1), addr);
            }

            Opcode::Add => {
                self.add(Register::from(t1), Register::from(t2), Register::from(t3));
            }

            Opcode::Addi => {
                let half = inst_half(inst);
                self.addi(Register::from(t1), half);
            }

            Opcode::Mul => {
                self.mul(Register::from(t1), Register::from(t2));
            }

            Opcode::Not => {
                self.not(Register::from(t1), Register::from(t2));
            }

            Opcode::And => {
                self.and(Register::from(t1), Register::from(t2), Register::from(t3));
            }

            Opcode::Or => {
                self.or(Register::from(t1), Register::from(t2), Register::from(t3));
            }

            Opcode::Shf => {
                self.shf(Register::from(t1), Register::from(t2), Register::from(t3));
            }

            Opcode::Jmp => {
                let inst = self.next_inst();
                let addr = inst_addr(inst);
                self.jmp(addr);
            }

            Opcode::Call => {
                let inst = self.next_inst();
                let addr = inst_addr(inst);
                self.call(addr);
            }

            Opcode::Ret => {
                self.ret();
            }

            Opcode::Syscall => {
                let index = self.read(Register::S0);
                self.syscall(index);
            }

            Opcode::Halt => {
                self.running = false;
            }

            _ => {}
        }

        self.clear(Register::ZERO);
    }

    unsafe fn mov(&mut self, r_dest: Register, r_src: Register) {
        let dest = self.dest(r_dest);
        let src = self.src(r_src);
        ternary::copy(dest, src, WORD_ISIZE);
    }

    unsafe fn movi(&mut self, r_dest: Register, half: Half) {
        let dest = self.dest(r_dest);
        ternary::clear(dest, WORD_ISIZE);
        ternary::copy(dest, ptr!(half), HALF_ISIZE);
    }

    unsafe fn movw(&mut self, r_dest: Register, word: Word) {
        let dest = self.dest(r_dest);
        ternary::copy(dest, ptr!(word), WORD_ISIZE);
    }

    unsafe fn mova(&mut self, r_dest: Register, addr: Addr) {
        let dest = self.dest(r_dest);
        ternary::from_int(dest, addr as isize, WORD_ISIZE);
    }

    unsafe fn add(&mut self, r_dest: Register, r_lhs: Register, r_rhs: Register) {
        let dest = self.dest(r_dest);
        let lhs = self.src(r_lhs);
        let rhs = self.src(r_rhs);

        ternary::clear(dest, WORD_ISIZE);
        let carry = ternary::add(dest, lhs, rhs, WORD_ISIZE);
        self.clear(Register::HI);
        ternary::set_trit(self.dest(Register::HI), 0, carry);
    }

    unsafe fn addi(&mut self, r_dest: Register, half: Half) {
        let dest = self.dest(r_dest);
        let lhs = dest;

        let mut word = EMPTY_WORD;
        let rhs = mut_ptr!(word);

        ternary::copy(rhs, ptr!(half), HALF_ISIZE);
        let carry = ternary::add(dest, lhs, rhs, WORD_ISIZE);
        self.clear(Register::HI);
        ternary::set_trit(self.dest(Register::HI), 0, carry);
    }

    unsafe fn mul(&mut self, r_lhs: Register, r_rhs: Register) {
        let lhs = self.src(r_lhs);
        let rhs = self.src(r_rhs);

        self.clear(Register::LO);
        self.clear(Register::HI);
        ternary::multiply(self.dest(Register::LO), lhs, rhs, WORD_ISIZE);
    }

    unsafe fn not(&mut self, r_dest: Register, r_src: Register) {
        ternary::map(self.dest(r_dest), self.src(r_src), WORD_ISIZE, |t| -t);
    }

    unsafe fn and(&mut self, r_dest: Register, r_lhs: Register, r_rhs: Register) {
        ternary::zip(self.dest(r_dest), self.src(r_lhs), self.src(r_rhs), WORD_ISIZE, |t1, t2| t1 & t2);
    }

    unsafe fn or(&mut self, r_dest: Register, r_lhs: Register, r_rhs: Register) {
        ternary::zip(self.dest(r_dest), self.src(r_lhs), self.src(r_rhs), WORD_ISIZE, |t1, t2| t1 | t2);
    }

    unsafe fn shf(&mut self, r_dest: Register, r_lhs: Register, r_rhs: Register) {
        let mut word = EMPTY_WORD;
        ternary::copy(mut_ptr!(word), self.src(r_lhs), WORD_ISIZE);

        let offset = self.read(r_rhs);
        let shifted_offset = offset + WORD_ISIZE;
        if shifted_offset < 0 || shifted_offset > WORD_ISIZE * 3 {
            return;
        }

        self.clear(Register::LO);
        self.clear(r_dest);
        self.clear(Register::HI);

        let src = ptr!(word);
        let lo = self.dest(Register::LO);
        let mid = self.dest(r_dest);
        let hi = self.dest(Register::HI);

        ternary::copy_blocks(src, WORD_SIZE, shifted_offset as usize, vec![
            (lo, WORD_SIZE),
            (mid, WORD_SIZE),
            (hi, WORD_SIZE),
        ]);
    }

    fn jmp(&mut self, addr: Addr) {
        self.pc = addr;
    }

    unsafe fn call(&mut self, addr: Addr) {
        let pc = self.pc as isize;
        self.write(Register::RA, pc);
        self.jmp(addr);
    }

    unsafe fn ret(&mut self) {
        let addr = self.read(Register::RA) as Addr;
        self.jmp(addr);
    }

    unsafe fn syscall(&mut self, index: isize) {
        use text;

        let syscall = Syscall::from(index);
        match syscall {
            Syscall::PrintLine => {
                let addr = self.read(Register::A0);
                let local_memory = self.memory.offset(addr);
                let (s, _) = text::decode_str(local_memory);
                println!("{}", s);
            }

            Syscall::Exit => {
                use std::process::exit;
                let code = self.read(Register::A0) as i32;
                exit(code);
            }
        }
    }
}

fn inst_half(inst: Word) -> Half {
    let mut half = EMPTY_HALF;
    unsafe { ternary::copy(mut_ptr!(half), tryte_ptr!(inst, 2), HALF_ISIZE) };
    half
}

fn inst_addr(inst: Word) -> Addr {
    unsafe { ternary::to_int(ptr!(inst), WORD_ISIZE) as Addr }
}

impl Drop for VM {
    fn drop(&mut self) {
        unsafe { free(transmute(self.memory)) };
    }
}
