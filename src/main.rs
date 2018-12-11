extern crate regex;

mod day_1;
mod day_10;
mod day_2;
mod day_3;
mod day_8;

fn main() {
    println!("Day 1: {}, {}", day_1::star_1(), day_1::star_2());
    println!("Day 2: {}, {}", day_2::star_1(), day_2::star_2());
    println!("Day 3: {}, {}", day_3::star_1(), day_3::star_2());
    println!("Day 8: {}, {}", day_8::star_1(), day_8::star_2());
    day_10::star_1();
}
