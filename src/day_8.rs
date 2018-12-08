use std::collections::VecDeque;
use std::fs;

pub fn star_1() -> u32 {
    let contents =
        fs::read_to_string("./input/day_8.txt").expect("Something went wrong reading the file");

    let mut numbers: VecDeque<u32> = contents
        .split(" ")
        .map(|x| x.trim().parse::<u32>().expect("Can not parse!"))
        .collect();

    parse_element(&mut numbers)
}

fn parse_element(element: &mut VecDeque<u32>) -> u32 {
    let child_count = if let Some(i) = element.pop_front() {
        i
    } else {
        0
    };

    let metadata_count = if let Some(i) = element.pop_front() {
        i
    } else {
        0
    };

    let mut meta_sum = 0;
    for _ in 0..child_count {
        meta_sum += parse_element(element);
    }

    for _ in 0..metadata_count {
        meta_sum += if let Some(i) = element.pop_front() {
            i
        } else {
            0
        };
    }

    meta_sum
}
