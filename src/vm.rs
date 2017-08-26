use ternary;
use types::*;
use opcodes::Opcode;
use registers::{Register, REGISTER_COUNT};
use syscalls::Syscall;

pub const PROGRAM_MAGIC_NUMBER: isize = 47330224520; // 1TTTTT1TTTTT1TTTTT1TTTTT

pub struct VM {
    pub registers: [Word; REGISTER_COUNT],
    pub memory: Vec<Trit>,
    pub pc: Addr,
    pub running: bool,
}

impl VM {
    pub fn new(memory_size: usize) -> VM {
        VM {
            registers: [EMPTY_WORD; REGISTER_COUNT],
            memory: vec![Trit::Zero; memory_size],
            pc: 0,
            running: false,
        }
    }

    pub fn src(&self, r: Register) -> &Word {
        &self.registers[r as usize]
    }

    pub fn dest(&mut self, r: Register) -> &mut Word {
        &mut self.registers[r as usize]
    }

    pub fn read(&mut self, r: Register) -> isize {
        ternary::to_int(self.src(r))
    }

    pub fn write(&mut self, r: Register, value: isize) {
        ternary::from_int(self.dest(r), value);
    }

    pub fn clear(&mut self, r: Register) {
        self.dest(r).copy_from_slice(&EMPTY_WORD);
    }

    pub fn init(&mut self) {
        let magic_number = ternary::to_int(&self.memory[..WORD_SIZE]);
        assert_eq!(magic_number, PROGRAM_MAGIC_NUMBER);

        let pc_start = ternary::to_int(&self.memory[WORD_SIZE..][..WORD_SIZE]) as Addr;
        self.pc = pc_start;

        self.running = true;
    }

    pub fn run(&mut self) {
        self.init();

        while self.running {
            unsafe { self.step() };
        }
    }

    fn next_inst(&mut self) -> Word {
        let inst = &mut EMPTY_WORD;
        let location = &self.memory[self.pc..][..WORD_SIZE];
        inst.copy_from_slice(location);

        self.pc += WORD_SIZE;
        *inst
    }

    pub unsafe fn step(&mut self) {
        let inst = self.next_inst();
        let (t0, t1, t2, t3) = ternary::read_trytes(&inst);
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

            Opcode::St => {
                self.op_store(Register::from(t1), Register::from(t2), t3, TRYTE_ISIZE);
            }

            Opcode::Sh => {
                self.op_store(Register::from(t1), Register::from(t2), t3, HALF_ISIZE);
            }

            Opcode::Sw => {
                self.op_store(Register::from(t1), Register::from(t2), t3, WORD_ISIZE);
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

    fn op_mov(&mut self, r_dest: Register, r_src: Register) {
        let src = self.src(r_src).to_vec();
        let dest = self.dest(r_dest);
        dest.copy_from_slice(&src)
    }

    unsafe fn op_movi(&mut self, r_dest: Register, half: Half) {
        let dest = self.dest(r_dest).as_mut_ptr();
        ternary::clear(dest, WORD_ISIZE);
        ternary::copy(dest, ptr!(half), HALF_ISIZE);
    }

    fn op_movw(&mut self, r_dest: Register, word: Word) {
        let dest = self.dest(r_dest);
        dest.copy_from_slice(&word);
    }

    fn op_mova(&mut self, r_dest: Register, addr: Addr) {
        let dest = self.dest(r_dest);
        ternary::from_int(dest, addr as isize);
    }

    unsafe fn op_load(&mut self, r_dest: Register, r_addr: Register, offset: isize, len: isize) {
        let dest = self.dest(r_dest).as_mut_ptr();

        let addr_src = self.src(r_addr);
        let addr = ternary::to_int(&addr_src[..len as usize]);
        let src = self.memory[(addr + offset) as usize..].as_ptr();

        ternary::clear(dest, WORD_ISIZE);
        ternary::copy(dest, src, len);
    }

    unsafe fn op_store(&mut self, r_addr: Register, r_src: Register, offset: isize, len: isize) {
        let src = self.src(r_src).as_ptr();

        let addr = {
            let addr_src = self.src(r_addr);
            ternary::to_int(&addr_src[..len as usize])
        };
        let dest = self.memory[(addr + offset) as usize..].as_mut_ptr();

        ternary::copy(dest, src, len);
    }

    unsafe fn op_add(&mut self, r_dest: Register, r_lhs: Register, r_rhs: Register) {
        let dest = self.dest(r_dest).as_mut_ptr();
        let lhs = self.src(r_lhs).as_ptr();
        let rhs = self.src(r_rhs).as_ptr();

        self.add(dest, lhs, rhs, WORD_ISIZE);
    }

    unsafe fn op_addi(&mut self, r: Register, half: Half) {
        let dest = self.dest(r).as_mut_ptr();
        let lhs = self.src(r).as_ptr();

        let mut word = EMPTY_WORD;
        let rhs = mut_ptr!(word);
        ternary::copy(rhs, ptr!(half), HALF_ISIZE);

        self.add(dest, lhs, rhs, WORD_ISIZE);
    }

    unsafe fn add(&mut self, dest: *mut Trit, lhs: *const Trit, rhs: *const Trit, len: isize) {
        let carry = ternary::add(dest, lhs, rhs, len);
        self.clear(Register::HI);
        *self.dest(Register::HI).as_mut_ptr().offset(0) = carry;
    }

    unsafe fn op_mul(&mut self, r_lhs: Register, r_rhs: Register) {
        let lhs = self.src(r_lhs).as_ptr();
        let rhs = self.src(r_rhs).as_ptr();
        self.multiply(lhs, rhs, WORD_ISIZE);
    }

    unsafe fn op_muli(&mut self, r: Register, half: Half) {
        let lhs = self.src(r).as_ptr();

        let mut word = EMPTY_WORD;
        let rhs = mut_ptr!(word);
        ternary::copy(rhs, ptr!(half), HALF_ISIZE);

        self.multiply(lhs, rhs, WORD_ISIZE);
    }

    unsafe fn multiply(&mut self, lhs: *const Trit, rhs: *const Trit, len: isize) {
        self.clear(Register::LO);
        self.clear(Register::HI);
        ternary::multiply(self.dest(Register::LO).as_mut_ptr(), lhs, rhs, len);
    }

    fn op_not(&mut self, r_dest: Register, r_src: Register) {
        let word = &mut EMPTY_WORD;
        word.copy_from_slice(self.dest(r_dest));
        ternary::map(word, self.src(r_src), |t| -t);
        self.dest(r_dest).copy_from_slice(word);
    }

    unsafe fn op_and(&mut self, r_dest: Register, r_lhs: Register, r_rhs: Register) {
        ternary::zip(self.dest(r_dest).as_mut_ptr(),
                     self.src(r_lhs).as_ptr(),
                     self.src(r_rhs).as_ptr(),
                     WORD_ISIZE,
                     |t1, t2| t1 & t2);
    }

    unsafe fn op_or(&mut self, r_dest: Register, r_lhs: Register, r_rhs: Register) {
        ternary::zip(self.dest(r_dest).as_mut_ptr(),
                     self.src(r_lhs).as_ptr(),
                     self.src(r_rhs).as_ptr(),
                     WORD_ISIZE,
                     |t1, t2| t1 | t2);
    }

    unsafe fn op_shf(&mut self, r_dest: Register, r_src: Register, r_offset: Register) {
        let dest = self.dest(r_dest).as_mut_ptr();
        let src = self.src(r_src).as_ptr();
        let offset = self.read(r_offset);
        self.shift(dest, src, offset);
    }

    unsafe fn op_shfi(&mut self, r: Register, offset: isize) {
        let dest = self.dest(r).as_mut_ptr();
        let src = self.src(r).as_ptr();
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
        let lo = self.dest(Register::LO).as_mut_ptr();
        let hi = self.dest(Register::HI).as_mut_ptr();

        let blocks = vec![(lo, WORD_SIZE), (dest, WORD_SIZE), (hi, WORD_SIZE)];
        ternary::copy_blocks(src, WORD_SIZE, shifted_offset as usize, blocks);
    }

    fn op_cmp(&mut self, r_dest: Register, r_lhs: Register, r_rhs: Register) {
        let word = &mut EMPTY_WORD;

        word.copy_from_slice(self.dest(r_dest));
        
        word[0] = ternary::compare(self.src(r_lhs), self.src(r_rhs));
        
        self.dest(r_dest).copy_from_slice(word);
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
        let src = self.src(r).as_ptr();
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
    ternary::to_int(&inst[2*TRYTE_SIZE..][..HALF_SIZE])
}

fn inst_reladdr(inst: Word) -> RelAddr {
    ternary::to_int(&inst[2*TRYTE_SIZE..][..HALF_SIZE]) as RelAddr
}

fn inst_addr(inst: Word) -> Addr {
    ternary::to_int(&inst[..WORD_SIZE]) as Addr
}
