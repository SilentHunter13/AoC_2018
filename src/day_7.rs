use regex::Regex;
use std::collections::HashMap;
use std::fs;

const WORKER_MAX: usize = 5;

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

pub fn star_2() -> u32 {
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

    let mut seconds = 0;
    let mut workers: HashMap<char, u8> = HashMap::new();
    while steps.len() > 0 || workers.len() > 0 {
        //Zeit ablaufen lassen
        seconds += 1;
        for work in &mut workers {
            *work.1 -= 1;

            if *work.1 == 0 {
                //gefundenen Schritt als Vorbedingung löschen
                for step in steps.iter_mut() {
                    step.1.retain(|x| x != work.0);
                }
            }
        }

        //freie Arbeiter löschen
        workers.retain(|_, x| *x != 0);

        //bereite Schritte finden
        let mut founds: Vec<char> = Vec::new();
        for (letter, pre_steps) in steps.iter() {
            if pre_steps.is_empty() {
                founds.push(*letter);
            }
        }
        founds.sort_unstable();

        let mut founds = founds.iter();

        //bereite Schritte auf die Arbeiter verteilen
        while workers.len() < WORKER_MAX {
            let next = founds.next();

            if let Some(i) = next {
                workers.insert(*i, get_duration(*i));

                //gefundenen Schritt löschen
                steps.remove(i);
            } else {
                break;
            }
        }
    }
    seconds - 1
}

fn get_duration(letter: char) -> u8 {
    60 + letter as u8 - 0x40
}
