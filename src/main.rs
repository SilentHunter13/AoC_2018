extern crate regex;

mod day_1;
mod day_10;
mod day_2;
mod day_22;
mod day_3;
mod day_4;
mod day_5;
mod day_7;
mod day_8;

fn main() {
    println!("Day 1: {}, {}", day_1::star_1(), day_1::star_2());
    println!("Day 2: {}, {}", day_2::star_1(), day_2::star_2());
    println!("Day 3: {}, {}", day_3::star_1(), day_3::star_2());
    println!("Day 4: {}, {}", day_4::star_1(), day_4::star_2());
    println!("Day 5: {}, {}", day_5::star_1(), day_5::star_2());
    println!("Day 7: {}, {}", day_7::star_1(), day_7::star_2());
    println!("Day 8: {}, {}", day_8::star_1(), day_8::star_2());
    day_10::star_1();
    println!("Day 22: {}", day_22::star_1());
}
