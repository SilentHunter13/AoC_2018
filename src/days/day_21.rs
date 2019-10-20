// 0   seti 123 0 4        a = 123
// 1   bani 4 456 4        a &= 456
// 2   eqri 4 72 4         if a == 72
// 3   addr 4 1 1          Sprung nach 5
// 4   seti 0 0 1          else Sprung nach 1
// 5   seti 0 0 4          a = 0
// 6   bori 4 65536 5      b = a | 0x10000
// 7   seti 10704114 0 4   a = 0xA354F2
// 8   bani 5 255 2        c = b & 0xFF
// 9   addr 4 2 4          a = a + c
// 10  bani 4 16777215 4   a = a & 0xFFFFFF
// 11  muli 4 65899 4      a = a * 0x1016B
// 12  bani 4 16777215 4   a = a & 0xFFFFFF
// 13  gtir 256 5 2        if b < 256
// 14  addr 2 1 1          Sprung nach 16
// 15  addi 1 1 1          else Sprung nach 17
// 16  seti 27 2 1         Sprung nach 28
// 17  seti 0 4 2          c = 0
// 18  addi 2 1 3          d = c + 1
// 19  muli 3 256 3        d = d * 256
// 20  gtrr 3 5 3          if d > b
// 21  addr 3 1 1          Sprung nach 23
// 22  addi 1 1 1          else Sprung nach 24
// 23  seti 25 5 1         Sprung nach 26
// 24  addi 2 1 2          c = c + 1
// 25  seti 17 5 1         Sprung nach 18
// 26  setr 2 6 5          b = c
// 27  seti 7 8 1          Sprung nach 8
// 28  eqrr 4 0 2          if a == e
// 29  addr 2 1 1          Ende
// 30  seti 5 3 1          else Sprung nach 6

pub fn star_1() -> usize {
    let mut a = 0;

    let mut b = a | 0x10000;
    a = 0x00A3_54F2;

    loop {
        let mut c = b & 0xFF;
        a += c;
        a &= 0x00FF_FFFF;
        a *= 0x1016b;
        a &= 0x00FF_FFFF;

        if b >= 256 {
            c = 0;
            loop {
                let mut d = c + 1;
                d *= 256;

                if d > b {
                    b = c;
                    break;
                } else {
                    c += 1;
                }
            }
        } else {
            break;
        }
    }

    a
}

pub fn star_2() -> usize {
    let mut a: u64 = 0;
    let e = 12_420_065;
    //let e = 0;
    let mut inst_counter = 0;

    loop {
        let mut b = a | 0x10000;
        a = 0x00A3_54F2;

        loop {
            let mut c = b & 0xFF;
            a += c;
            a &= 0x00FF_FFFF;
            a *= 0x1016b;
            a &= 0x00FF_FFFF;
            //println!("a:{:?}  b:{:?}  c:{:?}", a, b, c);

            inst_counter += 8;

            if b >= 256 {
                c = 0;
                loop {
                    let mut d = c + 1;
                    d *= 256;

                    inst_counter += 5;

                    if d > b {
                        //println!("b:{:?}  c:{:?}  d:{:?}", b, c, d);
                        b = c;
                        inst_counter += 4;

                        break;
                    } else {
                        c += 1;

                        inst_counter += 3;
                    }
                }
            } else {
                inst_counter += 3;
                break;
            }
        }
        //println!("Inst: {:?}, {:?}", inst_counter, a);
        if a == e {
            break;
        }
    }
    foo();
    inst_counter
}

fn foo() {
    let mut a: u64 = 0;
    let e = 12420065;
    //let e = 0;

    loop {
        let mut b = a | 0x10000;

        a = 0xA354F2;
        while b != 0 {
            let c = b & 0xFF;
            a = a + c;
            a = a * 0x1016b;
            a = a & 0xFFFFFF;

            println!("{:?}", a);
            b = b >> 8;
        }

        println!("Lösung: {:?}", a);
        if a == e {
            //println!("Lösung: {:?}", a);
            break;
        }
    }
}
