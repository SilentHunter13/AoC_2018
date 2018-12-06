extern crate regex;

mod day_1;
mod day_2;
mod day_3;

fn main() {
    println!("Day 1: {}, {}", day_1::star_1(), day_1::star_2());
    println!("Day 2: {}, {}", day_2::star_1(), day_2::star_2());
    println!("Day 3: {}", day_3::star_1());
}
