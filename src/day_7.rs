use regex::Regex;
use std::collections::HashMap;
use std::fs;

pub fn star_1() -> String {
    //1 Prestep; 2 Step
    let claim_re =
        Regex::new("Step ([A-Z]+) must be finished before step ([A-Z]+) can begin.").unwrap();

    let contents =
        fs::read_to_string("./input/day_7.txt").expect("Something went wrong reading the file");

    let mut steps: HashMap<char, Vec<char>> = HashMap::new();

    for line in contents.lines() {
        let result = claim_re.captures(line).expect("Regex passt nicht!");

        let letter = result[2].chars().nth(0).unwrap();
        let pre_step = result[1].chars().nth(0).unwrap(); //hier muss noch ein besserer Typ gefunden werden

        steps.entry(letter).or_insert(Vec::new()).push(pre_step);
        steps.entry(pre_step).or_insert(Vec::new());
    }

    let mut step_sequence = String::new();

    while steps.len() > 0 {
        //Schritt finden
        let mut founds: Vec<char> = Vec::new();
        for (letter, pre_steps) in steps.iter() {
            if pre_steps.is_empty() {
                founds.push(*letter);
            }
        }

        founds.sort_unstable();
        let found = founds.first().unwrap();

        //gefundenen Schritt löschen
        steps.remove(found);
        step_sequence.push(*found);

        //gefundenen Schritt als Vorbedingung löschen
        for step in steps.iter_mut() {
            step.1.retain(|x| *x != *found);
        }
    }

    step_sequence
}
