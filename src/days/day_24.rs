use regex::Regex;
use std::fs;

pub fn star_1() -> u32 {
    //1 units; 2 hitpoints; 3 strength,weaknesses; 4 damage; 5 damage type; 6 initiative
    let unit_re = Regex::new(
        "([0-9]+)[^0-9]+([0-9]+)[^0-9(]+(?:\\((.+)\\))?[^0-9]+([0-9]+) ([a-z]+) [^0-9]+([0-9]+)",
    )
    .unwrap();

    //1 immunities
    let imunities_re = Regex::new("immune to ([^;\n]+)").unwrap();

    //1 weaknesses
    let weaknesses_re = Regex::new("weak to ([^;\n]+)").unwrap();

    let contents =
        fs::read_to_string("./input/day_3.txt").expect("Something went wrong reading the file");
    42
}
