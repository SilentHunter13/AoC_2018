//     #ip 1
// 0   seti 123 0 4        a = 123
// 1   bani 4 456 4        a &= 456
// 2   eqri 4 72 4         if a == 72
// 3   addr 4 1 1          Sprung nach 5
// 4   seti 0 0 1          else Sprung nach 1
// 5   seti 0 0 4          a = 0                    Beginn des eigentlichen Programm
// 6   bori 4 65536 5      b = a | 0x10000          b = 2^16
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
// 28  eqrr 4 0 2          if a == e                e ist Register 0
// 29  addr 2 1 1          Ende
// 30  seti 5 3 1          else Sprung nach 6

pub fn star_1() -> usize {
    let mut a = 0x00A3_54F2;
    let mut b = 0x10000;

    loop {
        a += b & 0xFF; //a += b % 256;
        a &= 0x00FF_FFFF; //a %= 16_777_216
        a *= 0x1016b; //a *= 65_899
        a &= 0x00FF_FFFF; //a %= 16_777_216

        if b < 256 {
            break;
        }
        b /= 256;
    }

    a
}

pub fn star_2() -> usize {
    let mut solutions = Vec::new();
    let mut a: usize = 0;

    loop {
        let mut b = a | 0x10000;

        a = 0x00A3_54F2;

        loop {
            a += b & 0xFF; //a += b % 256;
            a &= 0x00FF_FFFF; //a %= 16_777_216
            a *= 0x1016b; //a *= 65_899
            a &= 0x00FF_FFFF; //a %= 16_777_216

            if b < 256 {
                break;
            }
            b /= 256;
        }

        //Es wird ein Zyklus erreicht
        if solutions.contains(&a) {
            break;
        }
        solutions.push(a);
    }

    *solutions.last().expect("es muss ein letztes Element geben")
}
