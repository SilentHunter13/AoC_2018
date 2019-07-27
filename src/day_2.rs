use std::fs;
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

        if result_tuple.0 {
            counter[0] += 1;
        }
        if result_tuple.1 {
            counter[1] += 1;
        }
    }
    counter[0] * counter[1]
}

pub fn star_2() -> String {
    let contents =
        fs::read_to_string("./input/day_2.txt").expect("Something went wrong reading the file");

    let mut common_chars = String::new();

    for line1 in contents.lines() {
        for line2 in contents.lines() {
            if line1 != line2 {
                // String Position für String Position vergleichen
                let mut different_positions = 0;
                for (character1, character2) in line1.chars().zip(line2.chars()) {
                    if character1 != character2 {
                        different_positions += 1;
                    } else {
                        common_chars.push(character1);
                    }
                    if different_positions > 1 {
                        break;
                    }
                }
                if different_positions == 1 {
                    return common_chars;
                }
                common_chars.clear();
            }
        }
    }
    common_chars
}
