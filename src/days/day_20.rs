use std::collections::HashMap;
use std::fs;

#[derive(Default)]
struct Room {
    doors: [bool; 4],
    distance: usize,
}

#[derive(Debug)]
struct Instruction {
    steps: String,
    successors: Vec<usize>,
}

impl Instruction {
    pub fn new() -> Instruction {
        Instruction {
            steps: String::new(),
            successors: Vec::new(),
        }
    }
}

pub fn star_1() -> usize {
    let instructions = parse_instructions();

    let mut map = HashMap::new();

    go(&instructions, 0, (0, 0), &mut map);

    find_largest_distance(&map)
}

pub fn star_2() -> usize {
    let instructions = parse_instructions();

    let mut map = HashMap::new();

    go(&instructions, 0, (0, 0), &mut map);

    find_greater_1000(&map)
}

struct State {
    alternatives_start: Vec<usize>,
    alternatives_end: Vec<usize>,
    predecessor: usize,
}

fn parse_instructions() -> Vec<Instruction> {
    let contents =
        fs::read_to_string("./input/day_20.txt").expect("Something went wrong reading the file");

    let mut instructions: Vec<Instruction> = vec![Instruction::new()];

    let mut state_stack = Vec::new();

    let mut state = State {
        alternatives_start: vec![],
        alternatives_end: vec![],
        predecessor: 0,
    };

    let mut current_index = 0;

    for char in contents.chars() {
        match char {
            '(' => {
                state_stack.push(state);
                state = State {
                    predecessor: current_index,
                    alternatives_start: vec![instructions.len()],
                    alternatives_end: Vec::new(),
                };
                current_index = instructions.len();

                instructions.push(Instruction::new());
            }
            '|' => {
                state.alternatives_end.push(current_index);
                current_index = instructions.len();
                state.alternatives_start.push(current_index);
                instructions.push(Instruction::new());
            }
            //Regardless of which option is taken,
            //the route continues from the position it is left at
            //after taking those steps.
            ')' => {
                for alt in state.alternatives_start {
                    instructions[state.predecessor].successors.push(alt);
                }

                state = state_stack.pop().expect("Es ist keine Klammer offen!");
            }
            'N' | 'E' | 'S' | 'W' => {
                instructions[current_index].steps.push(char);
            }
            _ => (),
        }
    }

    instructions
}

fn go(
    instructions: &[Instruction],
    start_index: usize,
    start_position: (i32, i32),
    map: &mut HashMap<(i32, i32), Room>,
) {
    let position = go_steps(start_position, &instructions[start_index].steps, map);

    for successor in &instructions[start_index].successors {
        go(&instructions, *successor, position, map);
    }
}

fn go_steps(
    start: (i32, i32),
    instructions: &str,
    map: &mut HashMap<(i32, i32), Room>,
) -> (i32, i32) {
    let mut position = start;
    for inst in instructions.chars() {
        let mut old_room = map.entry(position).or_default();
        let old_distance = old_room.distance;
        match inst {
            'N' => {
                old_room.doors[0] = true;
                position.1 += 1;
                let mut new_room = map.entry(position).or_default();
                new_room.doors[2] = true;
                if new_room.distance == 0 {
                    new_room.distance = old_distance + 1;
                }
            }
            'E' => {
                old_room.doors[1] = true;
                position.0 += 1;
                let mut new_room = map.entry(position).or_default();
                new_room.doors[3] = true;
                if new_room.distance == 0 {
                    new_room.distance = old_distance + 1;
                }
            }
            'S' => {
                old_room.doors[2] = true;
                position.1 -= 1;
                let mut new_room = map.entry(position).or_default();
                new_room.doors[0] = true;
                if new_room.distance == 0 {
                    new_room.distance = old_distance + 1;
                }
            }
            'W' => {
                old_room.doors[3] = true;
                position.0 -= 1;
                let mut new_room = map.entry(position).or_default();
                new_room.doors[1] = true;
                if new_room.distance == 0 {
                    new_room.distance = old_distance + 1;
                }
            }
            _ => {} //alles andere ignorieren
        }
    }
    position
}

fn find_largest_distance(map: &HashMap<(i32, i32), Room>) -> usize {
    map.values()
        .max_by_key(|x| x.distance)
        .expect("Es gibt kein Maximum!")
        .distance
}

fn find_greater_1000(map: &HashMap<(i32, i32), Room>) -> usize {
    map.values().filter(|x| x.distance >= 1000).count()
}

fn show_map(map: &HashMap<(i32, i32), Room>, show_distance: bool) {
    let min_x = (map.iter().min_by_key(|x| (x.0).0).expect("gibts").0).0;
    let min_y = (map.iter().min_by_key(|x| (x.0).1).expect("gibts").0).1;
    let max_x = (map.iter().max_by_key(|x| (x.0).0).expect("gibts").0).0;
    let max_y = (map.iter().max_by_key(|x| (x.0).1).expect("gibts").0).1;

    let mut line1 = String::new();
    let mut line2 = String::new();

    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            if let Some(room) = map.get(&(x, y)) {
                if room.doors.iter().all(|&x| !x) {
                    line1.push('#');
                    line1.push('#');
                    line2.push('#');
                    line2.push('#');
                } else {
                    if x == 0 && y == 0 {
                        line1.push('X');
                    } else if show_distance {
                        line1.push(
                            std::char::from_digit((room.distance % 10) as u32, 10)
                                .expect("mehr als eine Stelle"),
                        );
                    } else {
                        line1.push('.');
                    }
                    if room.doors[1] {
                        line1.push('|');
                    } else {
                        line1.push('#');
                    }

                    if room.doors[2] {
                        line2.push('-');
                    } else {
                        line2.push('#');
                    }
                    line2.push('#');
                }
            } else {
                line1.push('#');
                line1.push('#');
                line2.push('#');
                line2.push('#');
            }
        }
        println!("{}", line1);
        println!("{}", line2);
        line1.clear();
        line2.clear();
    }
}
