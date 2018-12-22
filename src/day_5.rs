use std::fs;

pub fn star_1() -> usize {
    let contents =
        fs::read_to_string("./input/day_5.txt").expect("Something went wrong reading the file");

    let mut polymers = Vec::from(contents.trim().as_bytes());

    loop {
        let mut triggered_units: Vec<usize> = Vec::new();
        let mut skip = false;
        for i in 1..polymers.len() {
            if !skip {
                let diff: i16 = (polymers[i - 1] as i16 - polymers[i] as i16).abs();
                if diff == 0x20 {
                    triggered_units.push(i - 1);
                    skip = true;
                }
            } else {
                skip = false;
            }
        }
        if triggered_units.len() == 0 {
            break;
        }

        for index in triggered_units.iter().rev() {
            polymers.remove(*index);
            polymers.remove(*index);
        }
    }

    polymers.len()
}

pub fn star_2() -> usize {
    let contents =
        fs::read_to_string("./input/day_5.txt").expect("Something went wrong reading the file");

    let original_polymers = Vec::from(contents.trim().as_bytes());

    let mut min_length = usize::max_value();
    for char in 0x41..0xB5 {
        let mut polymers = original_polymers.clone();
        polymers.retain(|x| (*x != char) && (*x != char + 0x20));
        loop {
            let mut triggered_units: Vec<usize> = Vec::new();
            let mut skip = false;
            for i in 1..polymers.len() {
                if !skip {
                    let diff: i16 = (polymers[i - 1] as i16 - polymers[i] as i16).abs();
                    if diff == 0x20 {
                        triggered_units.push(i - 1);
                        skip = true;
                    }
                } else {
                    skip = false;
                }
            }
            if triggered_units.len() == 0 {
                break;
            }

            for index in triggered_units.iter().rev() {
                polymers.remove(*index);
                polymers.remove(*index);
            }
        }
        if polymers.len() < min_length {
            min_length = polymers.len();
        }
    }

    min_length
}
