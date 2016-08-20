use libc::{malloc, free};
use std::mem::transmute;

use ternary;
use types::*;
use opcodes::Opcode;
use registers::{Register, REGISTER_COUNT};
use syscalls::Syscall;

pub const PROGRAM_MAGIC_NUMBER: isize = 47330224520; // 1TTTTT1TTTTT1TTTTT1TTTTT

pub struct VM {
    pub registers: [Word; REGISTER_COUNT],
    pub memory: *mut Trit,
    pub memory_size: usize,
    pub pc: Addr,
    pub running: bool,
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
            running: false,
        }
    }

    pub fn src(&self, r: Register) -> *const Trit {
        ptr!(self.registers[r as usize])
    }

    pub fn dest(&mut self, r: Register) -> *mut Trit {
        mut_ptr!(self.registers[r as usize])
    }

    pub fn read(&mut self, r: Register) -> isize {
        unsafe { ternary::to_int(self.src(r), WORD_ISIZE) }
    }

    pub fn write(&mut self, r: Register, value: isize) {
        unsafe {
            ternary::from_int(self.dest(r), value, WORD_ISIZE);
        }
    }

    pub fn clear(&mut self, r: Register) {
        unsafe { ternary::clear(self.dest(r), WORD_ISIZE) }
    }

    pub fn init(&mut self) {
        let magic_number = unsafe { ternary::to_int(self.memory, WORD_ISIZE) };
        assert_eq!(magic_number, PROGRAM_MAGIC_NUMBER);

        let pc_start =
            unsafe { ternary::to_int(self.memory.offset(WORD_ISIZE), WORD_ISIZE) } as Addr;
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
                self.op_mov(Register::from(t1), Register::from(t2));
            }

            Opcode::Movi => {
                let half = inst_half(inst);
                self.op_movi(Register::from(t1), half);
            }

            Opcode::Movw => {
                let word = self.next_inst();
                self.op_movw(Register::from(t1), word);
            }

            Opcode::Mova => {
                let inst = self.next_inst();
                let addr = inst_addr(inst);
                self.op_mova(Register::from(t1), addr);
            }

            Opcode::Lt => {
                self.op_load(Register::from(t1), Register::from(t2), t3, TRYTE_ISIZE);
            }

            Opcode::Lh => {
                self.op_load(Register::from(t1), Register::from(t2), t3, HALF_ISIZE);
            }

            Opcode::Lw => {
                self.op_load(Register::from(t1), Register::from(t2), t3, WORD_ISIZE);
            }

            Opcode::Add => {
                self.op_add(Register::from(t1), Register::from(t2), Register::from(t3));
            }

            Opcode::Addi => {
                let half = inst_half(inst);
                self.op_addi(Register::from(t1), half);
            }

            Opcode::Mul => {
                self.op_mul(Register::from(t1), Register::from(t2));
            }

            Opcode::Muli => {
                let half = inst_half(inst);
                self.op_muli(Register::from(t1), half);
            }

            Opcode::Not => {
                self.op_not(Register::from(t1), Register::from(t2));
            }

            Opcode::And => {
                self.op_and(Register::from(t1), Register::from(t2), Register::from(t3));
            }

            Opcode::Or => {
                self.op_or(Register::from(t1), Register::from(t2), Register::from(t3));
            }

            Opcode::Shf => {
                self.op_shf(Register::from(t1), Register::from(t2), Register::from(t3));
            }

            Opcode::Shfi => {
                let offset = inst_half_isize(inst);
                self.op_shfi(Register::from(t1), offset);
            }

            Opcode::Cmp => {
                self.op_cmp(Register::from(t1), Register::from(t2), Register::from(t3));
            }

            Opcode::Jmp => {
                let addr = inst_addr(self.next_inst());
                self.op_jmp(addr);
            }

            Opcode::JT => {
                let addr = inst_reladdr(inst);
                self.op_jmp_conditional(Register::from(t1), addr, |t| t == Trit::Neg);
            }

            Opcode::J0 => {
                let addr = inst_reladdr(inst);
                self.op_jmp_conditional(Register::from(t1), addr, |t| t == Trit::Zero);
            }

            Opcode::J1 => {
                let addr = inst_reladdr(inst);
                self.op_jmp_conditional(Register::from(t1), addr, |t| t == Trit::Pos);
            }

            Opcode::JT0 => {
                let addr = inst_reladdr(inst);
                self.op_jmp_conditional(Register::from(t1), addr, |t| t != Trit::Pos);
            }

            Opcode::JT1 => {
                let addr = inst_reladdr(inst);
                self.op_jmp_conditional(Register::from(t1), addr, |t| t != Trit::Zero);
            }

            Opcode::J01 => {
                let addr = inst_reladdr(inst);
                self.op_jmp_conditional(Register::from(t1), addr, |t| t != Trit::Neg);
            }

            Opcode::Call => {
                let addr = inst_addr(self.next_inst());
                self.op_call(addr);
            }

            Opcode::Ret => {
                self.op_ret();
            }

            Opcode::Syscall => {
                self.op_syscall(Register::T0);
            }

            Opcode::Halt => {
                self.running = false;
            }

            _ => {}
        }

        self.clear(Register::ZERO);
    }

    unsafe fn op_mov(&mut self, r_dest: Register, r_src: Register) {
        let dest = self.dest(r_dest);
        let src = self.src(r_src);
        ternary::copy(dest, src, WORD_ISIZE);
    }

    unsafe fn op_movi(&mut self, r_dest: Register, half: Half) {
        let dest = self.dest(r_dest);
        ternary::clear(dest, WORD_ISIZE);
        ternary::copy(dest, ptr!(half), HALF_ISIZE);
    }

    unsafe fn op_movw(&mut self, r_dest: Register, word: Word) {
        let dest = self.dest(r_dest);
        ternary::copy(dest, ptr!(word), WORD_ISIZE);
    }

    unsafe fn op_mova(&mut self, r_dest: Register, addr: Addr) {
        let dest = self.dest(r_dest);
        ternary::from_int(dest, addr as isize, WORD_ISIZE);
    }

    unsafe fn op_load(&mut self, r_dest: Register, r_src: Register, offset: isize, len: isize) {
        let dest = self.dest(r_dest);

        let addr_src = self.src(r_src);
        let base_addr = ternary::to_int(addr_src, len);
        let addr = base_addr + offset;
        let src = self.memory.offset(addr);

        ternary::clear(dest, WORD_ISIZE);
        ternary::copy(dest, src, len);
    }

    unsafe fn op_add(&mut self, r_dest: Register, r_lhs: Register, r_rhs: Register) {
        let dest = self.dest(r_dest);
        let lhs = self.src(r_lhs);
        let rhs = self.src(r_rhs);

        self.add(dest, lhs, rhs, WORD_ISIZE);
    }

    unsafe fn op_addi(&mut self, r: Register, half: Half) {
        let dest = self.dest(r);
        let lhs = self.src(r);

        let mut word = EMPTY_WORD;
        let rhs = mut_ptr!(word);
        ternary::copy(rhs, ptr!(half), HALF_ISIZE);

        self.add(dest, lhs, rhs, WORD_ISIZE);
    }

    unsafe fn add(&mut self, dest: *mut Trit, lhs: *const Trit, rhs: *const Trit, len: isize) {
        let carry = ternary::add(dest, lhs, rhs, len);
        self.clear(Register::HI);
        *self.dest(Register::HI).offset(0) = carry;
    }

    unsafe fn op_mul(&mut self, r_lhs: Register, r_rhs: Register) {
        let lhs = self.src(r_lhs);
        let rhs = self.src(r_rhs);
        self.multiply(lhs, rhs, WORD_ISIZE);
    }

    unsafe fn op_muli(&mut self, r: Register, half: Half) {
        let lhs = self.src(r);

        let mut word = EMPTY_WORD;
        let rhs = mut_ptr!(word);
        ternary::copy(rhs, ptr!(half), HALF_ISIZE);

        self.multiply(lhs, rhs, WORD_ISIZE);
    }

    unsafe fn multiply(&mut self, lhs: *const Trit, rhs: *const Trit, len: isize) {
        self.clear(Register::LO);
        self.clear(Register::HI);
        ternary::multiply(self.dest(Register::LO), lhs, rhs, len);
    }

    unsafe fn op_not(&mut self, r_dest: Register, r_src: Register) {
        ternary::map(self.dest(r_dest), self.src(r_src), WORD_ISIZE, |t| -t);
    }

    unsafe fn op_and(&mut self, r_dest: Register, r_lhs: Register, r_rhs: Register) {
        ternary::zip(self.dest(r_dest),
                     self.src(r_lhs),
                     self.src(r_rhs),
                     WORD_ISIZE,
                     |t1, t2| t1 & t2);
    }

    unsafe fn op_or(&mut self, r_dest: Register, r_lhs: Register, r_rhs: Register) {
        ternary::zip(self.dest(r_dest),
                     self.src(r_lhs),
                     self.src(r_rhs),
                     WORD_ISIZE,
                     |t1, t2| t1 | t2);
    }

    unsafe fn op_shf(&mut self, r_dest: Register, r_src: Register, r_offset: Register) {
        let dest = self.dest(r_dest);
        let src = self.src(r_src);
        let offset = self.read(r_offset);
        self.shift(dest, src, offset);
    }

    unsafe fn op_shfi(&mut self, r: Register, offset: isize) {
        let dest = self.dest(r);
        let src = self.src(r);
        self.shift(dest, src, offset);
    }

    unsafe fn shift(&mut self, dest: *mut Trit, src: *const Trit, offset: isize) {
        let mut word = EMPTY_WORD;
        ternary::copy(mut_ptr!(word), src, WORD_ISIZE);

        let shifted_offset = offset + WORD_ISIZE;
        if shifted_offset < 0 || shifted_offset > WORD_ISIZE * 3 {
            return;
        }

        ternary::clear(dest, WORD_ISIZE);
        self.clear(Register::LO);
        self.clear(Register::HI);

        let src = ptr!(word);
        let lo = self.dest(Register::LO);
        let hi = self.dest(Register::HI);

        let blocks = vec![(lo, WORD_SIZE), (dest, WORD_SIZE), (hi, WORD_SIZE)];
        ternary::copy_blocks(src, WORD_SIZE, shifted_offset as usize, blocks);
    }

    fn op_cmp(&mut self, r_dest: Register, r_lhs: Register, r_rhs: Register) {
        let dest = self.dest(r_dest);
        let lhs = self.src(r_lhs);
        let rhs = self.src(r_rhs);

        unsafe {
            ternary::clear(dest, WORD_ISIZE);
            *dest.offset(0) = ternary::compare(lhs, rhs, WORD_ISIZE);
        }
    }

    fn op_jmp(&mut self, addr: Addr) {
        self.jump(addr);
    }

    fn jump(&mut self, addr: Addr) {
        self.pc = addr;
    }

    fn op_jmp_conditional<F>(&mut self, r: Register, addr: RelAddr, f: F)
        where F: Fn(Trit) -> bool
    {
        let src = self.src(r);
        let trit = unsafe { *src.offset(0) };
        if f(trit) {
            self.jump_relative(addr);
        }
    }

    fn jump_relative(&mut self, addr: RelAddr) {
        self.pc = (self.pc as RelAddr + addr) as Addr;
    }

    unsafe fn op_call(&mut self, addr: Addr) {
        let pc = self.pc as isize;
        self.write(Register::RA, pc);
        self.jump(addr);
    }

    unsafe fn op_ret(&mut self) {
        let addr = self.read(Register::RA) as Addr;
        self.jump(addr);
    }

    unsafe fn op_syscall(&mut self, r: Register) {
        let index = self.read(r);
        let syscall = Syscall::from(index);
        syscall.perform(self);
    }
}

fn inst_half(inst: Word) -> Half {
    let mut half = EMPTY_HALF;
    unsafe { ternary::copy(mut_ptr!(half), tryte_ptr!(inst, 2), HALF_ISIZE) };
    half
}

fn inst_half_isize(inst: Word) -> isize {
    unsafe { ternary::to_int(tryte_ptr!(inst, 2), HALF_ISIZE) }
}

fn inst_reladdr(inst: Word) -> RelAddr {
    unsafe { ternary::to_int(tryte_ptr!(inst, 2), HALF_ISIZE) as RelAddr }
}

fn inst_addr(inst: Word) -> Addr {
    unsafe { ternary::to_int(ptr!(inst), WORD_ISIZE) as Addr }
}

impl Drop for VM {
    fn drop(&mut self) {
        unsafe { free(transmute(self.memory)) };
    }
}
