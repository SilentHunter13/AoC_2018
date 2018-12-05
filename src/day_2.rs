use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn star_1() -> u32 {
    let file = File::open("./input/day_2.txt").expect("Cannot open file");
    let file = BufReader::new(&file);

    let mut counter: Vec<u32> = vec![0; 2];
    for line in file.lines() {
        let line = line.expect("Cannot read line!");

        let mut character_occurences: Vec<u32> = vec![0; 26];

        // Auftreten der Buchstaben zählen
        for character in line.chars() {
            let index = character as usize - 0x61; //0x61 = 'a'
            character_occurences[index] += 1;
        }

        // Prüfen, ob Buchstaben zwei oder dreimal vorkamen
        let mut result_tuple = (false, false);
        for occurence in character_occurences {
            if occurence == 2 {
                result_tuple.0 = true;
            } else if occurence == 3 {
                result_tuple.1 = true;
            }
        }

        if result_tuple.0 == true {
            counter[0] += 1;
        }
        if result_tuple.1 == true {
            counter[1] += 1;
        }
    }
    counter[0] * counter[1]
}

pub fn star_2() -> i32 {
    0
}
