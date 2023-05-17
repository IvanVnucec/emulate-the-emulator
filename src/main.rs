#![allow(warnings)]

#[derive(Debug)]
enum Instr {
    Nop,
    Jmp,
    Ldr,
}

impl From<u8> for Instr {
    fn from(value: u8) -> Self {
        match value {
            0 => Instr::Nop,
            1 => Instr::Jmp,
            2 => Instr::Ldr,
            _ => panic!("Invalid value for conversion to Instr"),
        }
    }
}

#[derive(Debug)]
struct CPU<'a> {
    mem: &'a [u8],
    pc: u8,
    ir: u8,
    r0: u8,
    r1: u8,
    r2: u8,
}

impl<'a> CPU<'a> {
    fn new(mem: &'a [u8]) -> Self {
        Self {
            mem: mem,
            pc: 0,
            ir: 0,
            r0: 0,
            r1: 0,
            r2: 0,
        }
    }

    fn tick(&mut self, repeat: usize) {
        for _ in 0..repeat {
            self.fetch();
            let instr = self.decode();
            self.execute(instr);
        }
    }

    fn fetch(&mut self) {
        self.ir = self.mem[self.pc as usize];
        self.pc += 1;
    }

    fn decode(&self) -> Instr {
        self.ir.into()
    }

    fn execute(&mut self, instr: Instr) {
        println!("{:?}", instr);

        match instr {
            Instr::Nop => (),
            Instr::Jmp => {
                self.pc = self.r0;
            }
            Instr::Ldr => {
                self.r0 = self.mem[self.pc as usize + 2];
                self.pc += 1;
            }
        }
    }
}

fn main() {
    let mut mem: [u8; 1024] = [0; 1024];
    mem[0..4].copy_from_slice(&[
        // while true
        Instr::Nop as u8,
        Instr::Ldr as u8,
        0,
        Instr::Jmp as u8,
    ]);

    let mut cpu = CPU::new(&mem);
    cpu.tick(10);
}
