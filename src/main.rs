#![allow(warnings)]

use std::fmt;

#[derive(Debug, Copy, Clone)]
enum Instr {
    And(usize, usize, usize),
    Mov(usize, u8),
    Add(usize, usize, usize),
    Sub(usize, usize, usize),
    Ldr(usize, u8),
    Str(usize, u8),
}

impl From<u16> for Instr {
    fn from(v: u16) -> Self {
        let opcode = ((v >> 12) as u8) & 0xF;
        let params = v & 0xFFF;

        match opcode {
            1 => {
                let rs1 = (params & 0b11) as usize;
                let rs2 = ((params >> 2) & 0b11) as usize;
                let rd = ((params >> 4) & 0b11) as usize;
                Instr::And(rd, rs1, rs2)
            }
            2 => {
                let rd = ((params >> 8) & 0b11) as usize;
                let val = params as u8;
                Instr::Mov(rd, val)
            }
            3 => {
                let rs1 = (params & 0b11) as usize;
                let rs2 = ((params >> 2) & 0b11) as usize;
                let rd = ((params >> 4) & 0b11) as usize;
                Instr::Add(rd, rs1, rs2)
            }
            4 => {
                let rs1 = (params & 0b11) as usize;
                let rs2 = ((params >> 2) & 0b11) as usize;
                let rd = ((params >> 4) & 0b11) as usize;
                Instr::Sub(rd, rs1, rs2)
            }
            5 => {
                let rd = ((params >> 8) & 0b11) as usize;
                let addr = params as u8;
                Instr::Ldr(rd, addr)
            }
            6 => {
                let rd = ((params >> 8) & 0b11) as usize;
                let addr = params as u8;
                Instr::Str(rd, addr)
            }
            _ => panic!("Unknown opcode: {opcode}"),
        }
    }
}

impl From<Instr> for u16 {
    fn from(i: Instr) -> Self {
        match i {
            Instr::And(rd, rs1, rs2) => {
                let opcode = 1_u16 << 12;
                let params = (((rd as u16) & 0b11) << 4)
                    | (((rs2 as u16) & 0b11) << 2)
                    | ((rs1 as u16) & 0b11);

                opcode | params
            }
            Instr::Mov(rd, val) => {
                let opcode = 2_u16 << 12;
                let params = (((rd as u16) & 0b11) << 8) | (val as u16);
                opcode | params
            }
            Instr::Add(rd, rs1, rs2) => {
                let opcode = 3_u16 << 12;
                let params = (((rd as u16) & 0b11) << 4)
                    | (((rs2 as u16) & 0b11) << 2)
                    | ((rs1 as u16) & 0b11);

                opcode | params
            }
            Instr::Sub(rd, rs1, rs2) => {
                let opcode = 4_u16 << 12;
                let params = (((rd as u16) & 0b11) << 4)
                    | (((rs2 as u16) & 0b11) << 2)
                    | ((rs1 as u16) & 0b11);

                opcode | params
            }
            Instr::Ldr(rd, addr) => {
                let opcode = 5_u16 << 12;
                let params = (((rd as u16) & 0b11) << 8) | (addr as u16);
                opcode | params
            }
            Instr::Str(rd, addr) => {
                let opcode = 6_u16 << 12;
                let params = (((rd as u16) & 0b11) << 8) | (addr as u16);
                opcode | params
            }
        }
    }
}

#[derive(Debug)]
struct CPU<'a> {
    mem: &'a mut [u8],
    pc: u16,
    ir: u16,
    reg: [u8; 4],
}

impl<'a> CPU<'a> {
    fn new(mem: &'a mut [u8]) -> Self {
        Self {
            mem: mem,
            pc: 0,
            ir: 0,
            reg: [0; 4],
        }
    }

    fn tick(&mut self, repeat: usize) {
        println!("{self}");
        for _ in 0..repeat {
            self.fetch();
            let instr = self.ir.into();
            self.execute(instr);
            println!("{self}");
        }
    }

    fn fetch(&mut self) {
        self.ir =
            ((self.mem[self.pc as usize + 1] as u16) << 8) | (self.mem[self.pc as usize] as u16);
        self.pc += 2;
    }

    fn execute(&mut self, instr: Instr) {
        println!("{:?}", instr);

        match instr {
            Instr::And(rd, rs1, rs2) => {
                self.reg[rd] = self.reg[rs1] & self.reg[rs2];
            }
            Instr::Mov(rd, val) => self.reg[rd] = val,
            Instr::Add(rd, rs1, rs2) => {
                self.reg[rd] = self.reg[rs1].wrapping_add(self.reg[rs2]);
            }
            Instr::Sub(rd, rs1, rs2) => {
                self.reg[rd] = self.reg[rs1].wrapping_sub(self.reg[rs2]);
            }
            Instr::Ldr(rd, addr) => self.reg[rd] = self.mem[addr as usize],
            Instr::Str(rd, addr) => self.mem[addr as usize] = self.reg[rd],
        }
    }
}

impl<'a> fmt::Display for CPU<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "pc: {:>5?}, ir: 0x{:0>4X?}, regs: {:>3?}",
            self.pc, self.ir, self.reg
        )
    }
}

struct Memory<const SIZE: usize> {
    data: [u8; SIZE],
}

impl<const SIZE: usize> Memory<SIZE> {
    fn new() -> Self {
        Self { data: [0; SIZE] }
    }

    fn load_program(&mut self, program: Vec<Instr>) {
        for (i, instr) in program.iter().enumerate() {
            let instr_value = u16::from(*instr);
            self.data[i * 2] = instr_value as u8;
            self.data[i * 2 + 1] = (instr_value >> 8) as u8;
        }
    }
}

fn main() {
    let mut mem = Memory::<256>::new();
    let prog = vec![
        Instr::Mov(0, 0b11),
        Instr::Mov(1, 0b01),
        Instr::And(0, 1, 0),
        Instr::And(2, 0, 0),
        Instr::Add(0, 1, 2),
        Instr::Sub(1, 0, 0),
        Instr::Sub(1, 1, 2),
        Instr::Ldr(3, 255),
        Instr::Add(3, 3, 2),
        Instr::Str(3, 255),
    ];

    mem.load_program(prog);
    mem.data[255] = 55;

    let mut cpu = CPU::new(&mut mem.data);
    cpu.tick(10);
    println!("mem: {:?}", mem.data);
}
