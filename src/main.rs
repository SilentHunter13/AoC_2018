#[cfg(feature = "run_all")]
mod algorithms;
mod days;

fn main() {
    #[cfg(feature = "run_all")]
    run_all();

    #[cfg(not(feature = "run_all"))]
    run_one_day();
}

#[cfg(not(feature = "run_all"))]
fn run_one_day() {
    print!("Day xx: {:?}", days::day_24::star_1());
    println!(", {}", days::day_24::star_2());
}

#[cfg(feature = "run_all")]
fn run_all() {
    print!("Day 1: {}", days::day_1::star_1());
    println!(", {}", days::day_1::star_2());
    print!("Day 2: {}", days::day_2::star_1());
    println!(", {}", days::day_2::star_2());
    print!("Day 3: {}", days::day_3::star_1());
    println!(", {}", days::day_3::star_2());
    print!("Day 4: {}", days::day_4::star_1());
    println!(", {}", days::day_4::star_2());
    print!("Day 5: {}", days::day_5::star_1());
    println!(", {}", days::day_5::star_2());
    print!("Day 6: {}", days::day_6::star_1());
    println!(", {}", days::day_6::star_2());
    print!("Day 7: {}", days::day_7::star_1());
    println!(", {}", days::day_7::star_2());
    print!("Day 8: {}", days::day_8::star_1());
    println!(", {}", days::day_8::star_2());
    print!("Day 9: {}", days::day_9::star_1());
    println!(", {}", days::day_9::star_2());
    days::day_10::star_1_2();
    print!("Day 11: {:?}", days::day_11::star_1());
    println!(", {:?}", days::day_11::star_2());
    print!("Day 12: {}", days::day_12::star_1());
    println!(", {}", days::day_12::star_2());
    print!("Day 13: {:?}", days::day_13::star_1());
    println!(", {:?}", days::day_13::star_2());
    print!("Day 14: {}", days::day_14::star_1());
    println!(", {}", days::day_14::star_2());
    print!("Day 16: {}", days::day_16::star_1());
    println!(", {}", days::day_16::star_2());
    print!("Day 18: {}", days::day_18::star_1());
    println!(", {}", days::day_18::star_2());
    print!("Day 19: {}", days::day_19::star_1());
    println!(", {}", days::day_19::star_2());
    print!("Day 21: {}", days::day_21::star_1());
    println!(); //", {}", days::day_21::star_2());
    print!("Day 22: {}", days::day_22::star_1());
    println!(", {}", days::day_22::star_2());
    print!("Day 23: {:?}", days::day_23::star_1());
    println!();
    print!("Day 24: {}", days::day_24::star_1());
    println!(", {}", days::day_24::star_2());
}
