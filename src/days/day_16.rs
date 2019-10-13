use regex::Regex;
use std::fs;

pub fn star_1() -> usize {
    //1 Before | After; 2 r0; 3 r1; 4 r2; 5 r3
    let pre_post_re =
        Regex::new("([a-zA-Z]+):.+([0-9]), ([0-9]), ([0-9]), ([0-9])").expect("error in regex");

    //1 op; 2 A; 3 B; 4 C
    let op_re = Regex::new("([0-9]+) ([0-9]) ([0-9]) ([0-9])").expect("error in regex");

    let contents =
        fs::read_to_string("./input/day_16_1.txt").expect("Something went wrong reading the file");

    let mut answer = 0;
    let mut possibilities = [[true; 16]; 16];

    let mut init_values = [0; 4];
    let mut op = (0, [0, 0, 0]);
    for line in contents.lines() {
        if let Some(capture) = pre_post_re.captures(line) {
            if capture[1].contains("Before") {
                //Init Werte speichern
                let r0 = capture[2].parse::<u32>().expect("cannot parse");
                let r1 = capture[3].parse::<u32>().expect("cannot parse");
                let r2 = capture[4].parse::<u32>().expect("cannot parse");
                let r3 = capture[5].parse::<u32>().expect("cannot parse");
                init_values = [r0, r1, r2, r3];
            } else {
                let r0 = capture[2].parse::<u32>().expect("cannot parse");
                let r1 = capture[3].parse::<u32>().expect("cannot parse");
                let r2 = capture[4].parse::<u32>().expect("cannot parse");
                let r3 = capture[5].parse::<u32>().expect("cannot parse");
                //Rechnungen ausprobieren
                let mut sum = 0;
                for i in 0..16 {
                    let possible = try_op(i as u8, init_values, op.1, [r0, r1, r2, r3]);
                    if possible {
                        sum += 1;
                    }
                    possibilities[op.0][i] &= possible;
                }
                if sum >= 3 {
                    answer += 1;
                }
            }
        } else if let Some(capture) = op_re.captures(line) {
            // Op Register speichern
            let op_code = capture[1].parse::<usize>().expect("cannot parse");
            let in1 = capture[2].parse::<u8>().expect("cannot parse");
            let in2 = capture[3].parse::<u8>().expect("cannot parse");
            let out = capture[4].parse::<u8>().expect("cannot parse");
            op = (op_code, [in1, in2, out]);
        }
    }
    //println!("{:?}", possibilities);
    answer
}

pub fn star_2() -> u32 {
    //1 op; 2 A; 3 B; 4 C
    let op_re = Regex::new("([0-9]+) ([0-9]) ([0-9]) ([0-9])").expect("error in regex");

    let contents =
        fs::read_to_string("./input/day_16_2.txt").expect("Something went wrong reading the file");

    let mut cpu = Cpu { registers: [0; 4] };

    for line in contents.lines() {
        if let Some(capture) = op_re.captures(line) {
            let op_code = capture[1].parse::<u8>().expect("cannot parse");
            let in1 = capture[2].parse::<u8>().expect("cannot parse");
            let in2 = capture[3].parse::<u8>().expect("cannot parse");
            let out = capture[4].parse::<u8>().expect("cannot parse");
            cpu.do_op(op_code, [in1, in2, out]);
        } else {
            panic!("unkown line")
        }
    }
    cpu.registers[0]
}

fn try_op(index: u8, init: [u32; 4], values: [u8; 3], result: [u32; 4]) -> bool {
    let mut device = Cpu { registers: init };

    device.do_op(index, values);
    device.state_is(result)
}

struct Cpu {
    registers: [u32; 4],
}

impl Cpu {
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
    fn state_is(&self, state: [u32; 4]) -> bool {
        self.registers == state
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
