use regex::Regex;
use std::collections::HashMap;
use std::fs;

pub fn star_1() -> u32 {
    //1 ID; 2 X; 3 Y; 4 Breite; 5 Höhe
    let claim_re = Regex::new("#([0-9]+) @ ([0-9]+),([0-9]+): ([0-9]+)x([0-9]+)").unwrap();

    let contents =
        fs::read_to_string("./input/day_3.txt").expect("Something went wrong reading the file");

    let mut fabric: HashMap<(u32, u32), u32> = HashMap::new();

    for line in contents.lines() {
        let claim_match = claim_re.captures(line).unwrap();

        let x = claim_match[2].parse::<u32>().unwrap();
        let y = claim_match[3].parse::<u32>().unwrap();
        let width = claim_match[4].parse::<u32>().unwrap();
        let height = claim_match[5].parse::<u32>().unwrap();

        for current_x in x..x + width {
            for current_y in y..y + height {
                *fabric.entry((current_x, current_y)).or_insert(0) += 1;
            }
        }
    }

    let mut double_claims = 0;
    for value in fabric.values() {
        if *value > 1 {
            double_claims += 1;
        }
    }
    double_claims
}
