use regex::Regex;
use std::fs;

struct Device {
    registers: [u8; 4],
}

pub fn star_1() -> usize {
    //1 Before | After; 2 r0; 3 r1; 4 r2; 5 r3
    let pre_post_re =
        Regex::new("([a-zA-Z]+):.+([0-9]), ([0-9]), ([0-9]), ([0-9])").expect("error in regex");

    //1 op; 2 A; 3 B; 4 C
    let op_re = Regex::new("([0-9]+) ([0-9]) ([0-9]) ([0-9])").expect("error in regex");

    let contents =
        fs::read_to_string("./input/day_16_1.txt").expect("Something went wrong reading the file");

    let mut answer = 0;

    let mut init_values = [0; 4];
    let mut op = (0, [0, 0, 0]);
    for line in contents.lines() {
        if let Some(capture) = pre_post_re.captures(line) {
            if capture[1].contains("Before") {
                //Init Werte speichern
                let r0 = capture[2].parse::<u8>().expect("cannot parse");
                let r1 = capture[3].parse::<u8>().expect("cannot parse");
                let r2 = capture[4].parse::<u8>().expect("cannot parse");
                let r3 = capture[5].parse::<u8>().expect("cannot parse");
                init_values = [r0, r1, r2, r3];
            } else {
                let r0 = capture[2].parse::<u8>().expect("cannot parse");
                let r1 = capture[3].parse::<u8>().expect("cannot parse");
                let r2 = capture[4].parse::<u8>().expect("cannot parse");
                let r3 = capture[5].parse::<u8>().expect("cannot parse");
                //Rechnungen ausprobieren
                let mut sum = 0;
                for i in 0..16 {
                    if try_op(i as u8, init_values, op.1, [r0, r1, r2, r3]) {
                        sum += 1;
                    }
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
    answer
}

fn try_op(index: u8, init: [u8; 4], values: [u8; 3], result: [u8; 4]) -> bool {
    let mut device = Device { registers: init };

    device.do_op(index, values);
    device.state_is(result)
}

impl Device {
    fn do_op(&mut self, index: u8, values: [u8; 3]) {
        match index {
            0 => self.addr(values[0], values[1], values[2]),
            1 => self.addi(values[0], values[1], values[2]),
            2 => self.mulr(values[0], values[1], values[2]),
            3 => self.muli(values[0], values[1], values[2]),
            4 => self.banr(values[0], values[1], values[2]),
            5 => self.bani(values[0], values[1], values[2]),
            6 => self.borr(values[0], values[1], values[2]),
            7 => self.bori(values[0], values[1], values[2]),
            8 => self.setr(values[0], values[2]),
            9 => self.seti(values[0], values[2]),
            10 => self.gtir(values[0], values[1], values[2]),
            11 => self.gtri(values[0], values[1], values[2]),
            12 => self.gtrr(values[0], values[1], values[2]),
            13 => self.eqir(values[0], values[1], values[2]),
            14 => self.eqri(values[0], values[1], values[2]),
            15 => self.eqrr(values[0], values[1], values[2]),
            _ => panic!("unknown opcode"),
        }
    }
    fn state_is(&self, state: [u8; 4]) -> bool {
        self.registers == state
    }
    fn addr(&mut self, a: u8, b: u8, c: u8) {
        self.registers[c as usize] = self.registers[a as usize] + self.registers[b as usize];
    }
    fn addi(&mut self, a: u8, b: u8, c: u8) {
        self.registers[c as usize] = self.registers[a as usize] + b;
    }
    fn mulr(&mut self, a: u8, b: u8, c: u8) {
        self.registers[c as usize] = self.registers[a as usize] * self.registers[b as usize];
    }
    fn muli(&mut self, a: u8, b: u8, c: u8) {
        self.registers[c as usize] = self.registers[a as usize] * b;
    }
    fn banr(&mut self, a: u8, b: u8, c: u8) {
        self.registers[c as usize] = self.registers[a as usize] & self.registers[b as usize];
    }
    fn bani(&mut self, a: u8, b: u8, c: u8) {
        self.registers[c as usize] = self.registers[a as usize] & b;
    }
    fn borr(&mut self, a: u8, b: u8, c: u8) {
        self.registers[c as usize] = self.registers[a as usize] | self.registers[b as usize];
    }
    fn bori(&mut self, a: u8, b: u8, c: u8) {
        self.registers[c as usize] = self.registers[a as usize] | b;
    }
    fn setr(&mut self, a: u8, c: u8) {
        self.registers[c as usize] = self.registers[a as usize];
    }
    fn seti(&mut self, a: u8, c: u8) {
        self.registers[c as usize] = a;
    }
    fn gtir(&mut self, a: u8, b: u8, c: u8) {
        if a > self.registers[b as usize] {
            self.registers[c as usize] = 1;
        } else {
            self.registers[c as usize] = 0;
        }
    }
    fn gtri(&mut self, a: u8, b: u8, c: u8) {
        if self.registers[a as usize] > b {
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
        if a == self.registers[b as usize] {
            self.registers[c as usize] = 1;
        } else {
            self.registers[c as usize] = 0;
        }
    }
    fn eqri(&mut self, a: u8, b: u8, c: u8) {
        if self.registers[a as usize] == b {
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
