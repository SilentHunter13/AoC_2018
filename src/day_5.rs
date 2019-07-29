
use std::collections::VecDeque;
use std::fs;
pub fn star_1() -> usize {
    let contents =
        fs::read_to_string("./input/day_5.txt").expect("Something went wrong reading the file");

    let mut polymer_before = VecDeque::from(Vec::from(contents.trim().as_bytes()));

    let mut polymer_after: Vec<u8> = Vec::new();

    while let Some(right) = polymer_before.pop_front() {
        if let Some(left) = polymer_after.pop() {
            let diff: i16 = (i16::from(left) - i16::from(right)).abs();

            if diff != 0x20 {
                polymer_after.push(left);
                polymer_after.push(right);
            }

        } else {
            //rechts nach links
            polymer_after.push(right);
        }
    }

    polymer_after.len()
}

pub fn star_2() -> usize {
    let contents =
        fs::read_to_string("./input/day_5.txt").expect("Something went wrong reading the file");

    let original_polymer = Vec::from(contents.trim().as_bytes());

    let mut min_length = usize::max_value();
    for char in 0x41..0xB5 {
        let mut polymer_before = VecDeque::from(original_polymer.clone());
        let mut polymer_after: Vec<u8> = Vec::new();
        polymer_before.retain(|x| (*x != char) && (*x != char + 0x20));
        while let Some(right) = polymer_before.pop_front() {
            if let Some(left) = polymer_after.pop() {
                let diff: i16 = (i16::from(left) - i16::from(right)).abs();

                if diff != 0x20 {
                    polymer_after.push(left);
                    polymer_after.push(right);
                }

            } else {
                //rechts nach links
                polymer_after.push(right);
            }
        }
        if polymer_after.len() < min_length {
            min_length = polymer_after.len();
        }
    }

    min_length
}
