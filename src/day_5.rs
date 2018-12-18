use std::fs;

pub fn star_1() -> usize {
    let contents =
        fs::read_to_string("./input/day_5.txt").expect("Something went wrong reading the file");

    let mut polymers = Vec::from(contents.trim().as_bytes());

    loop {
        //println!("polymers {:?}", polymers);
        let mut triggered_units: Vec<usize> = Vec::new();
        let mut skip = false;
        for i in 1..polymers.len() {
            if !skip {
                let diff: i16 = (polymers[i - 1] as i16 - polymers[i] as i16).abs();
                if diff == 0x20 {
                    triggered_units.push(i - 1);
                    triggered_units.push(i - 1);
                    skip = true;
                }
            } else {
                skip = false;
            }
        }
        //println!("triggered {:?}", triggered_units);
        if triggered_units.len() == 0 {
            break;
        }

        for index in triggered_units.iter().rev() {
            //println!("{:?}", index);
            polymers.remove(*index);
        }
    }

    polymers.len()
}
