use regex::Regex;
use std::fs;

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;

struct Claim {
    id: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

pub fn star_1() -> u32 {
    //1 ID; 2 X; 3 Y; 4 Breite; 5 Höhe
    let claim_re = Regex::new("#([0-9]+) @ ([0-9]+),([0-9]+): ([0-9]+)x([0-9]+)").unwrap();

    let contents =
        fs::read_to_string("./input/day_3.txt").expect("Something went wrong reading the file");

    let mut fabric: [u32; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];

    for line in contents.lines() {
        let claim_match = claim_re.captures(line).unwrap();

        let claim_x = claim_match[2].parse::<u32>().unwrap();
        let claim_y = claim_match[3].parse::<u32>().unwrap();
        let width = claim_match[4].parse::<u32>().unwrap();
        let height = claim_match[5].parse::<u32>().unwrap();

        for current_y in claim_y..claim_y + height {
            for current_x in claim_x..claim_x + width {
                fabric[current_x as usize + (current_y as usize * WIDTH)] += 1;
            }
        }
    }

    let mut double_claims = 0;
    for value in fabric.iter() {
        if *value > 1 {
            double_claims += 1;
        }
    }
    double_claims
}

pub fn star_2() -> u32 {
    //1 ID; 2 X; 3 Y; 4 Breite; 5 Höhe
    let claim_re = Regex::new("#([0-9]+) @ ([0-9]+),([0-9]+): ([0-9]+)x([0-9]+)").unwrap();

    let contents =
        fs::read_to_string("./input/day_3.txt").expect("Something went wrong reading the file");

    let mut fabric: [u32; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];
    let mut claim_list: Vec<Claim> = Vec::new();

    for line in contents.lines() {
        let claim_match = claim_re.captures(line).unwrap();

        let claim = Claim {
            id: claim_match[1].parse::<u32>().unwrap(),
            x: claim_match[2].parse::<u32>().unwrap(),
            y: claim_match[3].parse::<u32>().unwrap(),
            width: claim_match[4].parse::<u32>().unwrap(),
            height: claim_match[5].parse::<u32>().unwrap(),
        };

        for current_y in claim.y..claim.y + claim.height {
            for current_x in claim.x..claim.x + claim.width {
                fabric[current_x as usize + (current_y as usize * WIDTH)] += 1;
            }
        }
        claim_list.push(claim);
    }

    let mut found = true;
    for claim in claim_list.iter() {
        for current_y in claim.y..claim.y + claim.height {
            for current_x in claim.x..claim.x + claim.width {
                if fabric[current_x as usize + (current_y as usize * WIDTH)] != 1 {
                    found = false;
                }
            }
        }

        if found {
            return claim.id;
        } else {
            found = true;
        }
    }
    0
}
