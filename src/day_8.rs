use std::collections::VecDeque;
use std::fs;

pub fn star_1() -> u32 {
    let contents =
        fs::read_to_string("./input/day_8.txt").expect("Something went wrong reading the file");

    let mut numbers: VecDeque<u32> = contents
        .split(' ')
        .map(|x| x.trim().parse::<u32>().expect("Can not parse!"))
        .collect();

    parse_element(&mut numbers, &metadata_sum)
}

pub fn star_2() -> u32 {
    let contents =
        fs::read_to_string("./input/day_8.txt").expect("Something went wrong reading the file");

    let mut numbers: VecDeque<u32> = contents
        .split(' ')
        .map(|x| x.trim().parse::<u32>().expect("Can not parse!"))
        .collect();

    parse_element(&mut numbers, &metadata_index)
}

fn parse_element(
    element: &mut VecDeque<u32>,
    f: &dyn Fn(&mut VecDeque<u32>, u32, u32) -> u32,
) -> u32 {
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

    f(element, child_count, metadata_count)
}

fn metadata_sum(element: &mut VecDeque<u32>, child_count: u32, metadata_count: u32) -> u32 {
    let mut meta_sum = 0;
    for _ in 0..child_count {
        meta_sum += parse_element(element, &metadata_sum);
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

fn metadata_index(element: &mut VecDeque<u32>, child_count: u32, metadata_count: u32) -> u32 {
    let mut node_values = Vec::new();

    let mut meta_sum = 0;
    if child_count > 0 {
        for _ in 0..child_count {
            node_values.push(parse_element(element, &metadata_index));
        }

        for _ in 0..metadata_count {
            let index = if let Some(i) = element.pop_front() {
                if i > 0 {
                    (i - 1) as usize
                } else {
                    usize::max_value()
                }
            } else {
                usize::max_value()
            };

            meta_sum += if let Some(&node_value) = node_values.get(index) {
                node_value
            } else {
                0
            }
        }
    } else {
        for _ in 0..metadata_count {
            meta_sum += if let Some(i) = element.pop_front() {
                i
            } else {
                0
            };
        }
    }

    meta_sum
}
