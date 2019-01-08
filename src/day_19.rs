use regex::Regex;
use std::fs;

pub fn star_1() -> u32 {
    //1 op; 2 A; 3 B; 4 C
    let op_re = Regex::new("([a-z]+) ([0-9]+) ([0-9]+) ([0-9])").expect("error in regex");

    let contents =
        fs::read_to_string("./input/day_19.txt").expect("Something went wrong reading the file");

    let mut programm = Vec::new();

    for line in contents.lines() {
        if let Some(captures) = op_re.captures(line) {
            let opcode = get_opcode(&captures[1]);
            let in1 = captures[2].parse::<u8>().expect("cannot parse");
            let in2 = captures[3].parse::<u8>().expect("cannot parse");
            let out = captures[4].parse::<u8>().expect("cannot parse");
            programm.push((opcode, [in1, in2, out]))
        }
    }

    let mut cpu = Cpu {
        registers: [0; 6],
        pc: 0,
        pc_reg: 3,
    };
    cpu.run_program(&programm)
}

fn get_opcode(mnemonic: &str) -> u8 {
    match mnemonic {
        "addr" => 8,
        "addi" => 12,
        "mulr" => 0,
        "muli" => 5,
        "banr" => 9,
        "bani" => 7,
        "borr" => 6,
        "bori" => 15,
        "setr" => 2,
        "seti" => 14,
        "gtir" => 11,
        "gtri" => 13,
        "gtrr" => 4,
        "eqir" => 10,
        "eqri" => 1,
        "eqrr" => 3,
        &_ => 255,
    }
}

#[derive(Debug)]
struct Cpu {
    registers: [u32; 6],
    pc: u32,
    pc_reg: u8,
}

impl Cpu {
    fn run_program(&mut self, program: &Vec<(u8, [u8; 3])>) -> u32 {
        loop {
            if let Some(op) = program.get(self.pc as usize) {
                //pc in register schreiben
                self.registers[self.pc_reg as usize] = self.pc;
                self.do_op(op.0, op.1);
                //pc aus register laden
                self.pc = self.registers[self.pc_reg as usize];
                //pc incrementieren
                self.pc += 1;
            } else {
                break self.registers[0];
            }
        }
    }
    fn do_op(&mut self, index: u8, values: [u8; 3]) {
        match index {
            8 => self.addr(values[0], values[1], values[2]),
            12 => self.addi(values[0], values[1], values[2]),
            0 => self.mulr(values[0], values[1], values[2]),
            5 => self.muli(values[0], values[1], values[2]),
            9 => self.banr(values[0], values[1], values[2]),
            7 => self.bani(values[0], values[1], values[2]),
            6 => self.borr(values[0], values[1], values[2]),
            15 => self.bori(values[0], values[1], values[2]),
            2 => self.setr(values[0], values[2]),
            14 => self.seti(values[0], values[2]),
            11 => self.gtir(values[0], values[1], values[2]),
            13 => self.gtri(values[0], values[1], values[2]),
            4 => self.gtrr(values[0], values[1], values[2]),
            10 => self.eqir(values[0], values[1], values[2]),
            1 => self.eqri(values[0], values[1], values[2]),
            3 => self.eqrr(values[0], values[1], values[2]),
            _ => panic!("unknown opcode"),
        }
    }
    fn addr(&mut self, a: u8, b: u8, c: u8) {
        self.registers[c as usize] = self.registers[a as usize] + self.registers[b as usize];
    }
    fn addi(&mut self, a: u8, b: u8, c: u8) {
        self.registers[c as usize] = self.registers[a as usize] + b as u32;
    }
    fn mulr(&mut self, a: u8, b: u8, c: u8) {
        self.registers[c as usize] = self.registers[a as usize] * self.registers[b as usize];
    }
    fn muli(&mut self, a: u8, b: u8, c: u8) {
        self.registers[c as usize] = self.registers[a as usize] * b as u32;
    }
    fn banr(&mut self, a: u8, b: u8, c: u8) {
        self.registers[c as usize] = self.registers[a as usize] & self.registers[b as usize];
    }
    fn bani(&mut self, a: u8, b: u8, c: u8) {
        self.registers[c as usize] = self.registers[a as usize] & b as u32;
    }
    fn borr(&mut self, a: u8, b: u8, c: u8) {
        self.registers[c as usize] = self.registers[a as usize] | self.registers[b as usize];
    }
    fn bori(&mut self, a: u8, b: u8, c: u8) {
        self.registers[c as usize] = self.registers[a as usize] | b as u32;
    }
    fn setr(&mut self, a: u8, c: u8) {
        self.registers[c as usize] = self.registers[a as usize];
    }
    fn seti(&mut self, a: u8, c: u8) {
        self.registers[c as usize] = a as u32;
    }
    fn gtir(&mut self, a: u8, b: u8, c: u8) {
        if a as u32 > self.registers[b as usize] {
            self.registers[c as usize] = 1;
        } else {
            self.registers[c as usize] = 0;
        }
    }
    fn gtri(&mut self, a: u8, b: u8, c: u8) {
        if self.registers[a as usize] > b as u32 {
            self.registers[c as usize] = 1;
        } else {
            self.registers[c as usize] = 0;
        }
    }
    fn gtrr(&mut self, a: u8, b: u8, c: u8) {
        if self.registers[a as usize] > self.registers[b as usize] {
            self.registers[c as usize] = 1;
        } else {
            self.registers[c as usize] = 0;
        }
    }
    fn eqir(&mut self, a: u8, b: u8, c: u8) {
        if a as u32 == self.registers[b as usize] {
            self.registers[c as usize] = 1;
        } else {
            self.registers[c as usize] = 0;
        }
    }
    fn eqri(&mut self, a: u8, b: u8, c: u8) {
        if self.registers[a as usize] == b as u32 {
            self.registers[c as usize] = 1;
        } else {
            self.registers[c as usize] = 0;
        }
    }
    fn eqrr(&mut self, a: u8, b: u8, c: u8) {
        if self.registers[a as usize] == self.registers[b as usize] {
            self.registers[c as usize] = 1;
        } else {
            self.registers[c as usize] = 0;
        }
    }
}
