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

    let count = go(&instructions, 0, (0, 0), &mut map);

    println!("{:?}", count);
    show_map(&map, false);

    find_largest_distance(&map)
}

struct State {
    alternatives_start: Vec<usize>,
    alternatives_end: Vec<usize>,
    predecessor: usize,
}

fn parse_instructions() -> Vec<Instruction> {
    let contents = fs::read_to_string("./input/day_20_test.txt")
        .expect("Something went wrong reading the file");

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
            ')' => {
                state.alternatives_end.push(current_index);
                current_index = instructions.len();

                instructions.push(Instruction::new());
                for alt in state.alternatives_start {
                    instructions[state.predecessor].successors.push(alt);
                }
                for alt in state.alternatives_end {
                    instructions[alt].successors.push(current_index);
                }

                state = state_stack.pop().expect("Es ist keine Klammer offen!");
            }
            'N' | 'E' | 'S' | 'W' => {
                instructions[current_index].steps.push(char);
            }
            _ => {}
        }
    }

    loop {
        let mut empty_inst: Option<(usize, usize)> = None;
        //leere Instruktionen entfernen
        for (index, inst) in instructions.iter().enumerate() {
            if inst.steps.is_empty() && inst.successors.len() == 1 {
                empty_inst = Some((index, inst.successors[0]));
                break;
            }
        }

        if let Some((index, successor)) = empty_inst {
            instructions[index].steps.push('X');
            for inst in &mut instructions {
                if inst.successors.contains(&index) {
                    inst.successors.retain(|&x| x != index);
                    inst.successors.push(successor);
                }
            }
        } else {
            break;
        }
    }
    let last = instructions.len() - 1;
    for inst in &mut instructions {
        inst.successors.retain(|&x| x != last);
    }
    //println!("{:?}", instructions);
    instructions
}

fn go(
    instructions: &[Instruction],
    start_index: usize,
    start_position: (i32, i32),
    map: &mut HashMap<(i32, i32), Room>,
) -> u32 {
    println!("{:?} {:?}", start_index, start_position);
    let position = go_steps(start_position, &instructions[start_index].steps, map);
    let mut count = 1;

    for successor in &instructions[start_index].successors {
        count += go(&instructions, *successor, position, map);
    }
    count
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
                new_room.distance = old_distance + 1;
            }
            'E' => {
                old_room.doors[1] = true;
                position.0 += 1;
                let mut new_room = map.entry(position).or_default();
                new_room.doors[3] = true;
                new_room.distance = old_distance + 1;
            }
            'S' => {
                old_room.doors[2] = true;
                position.1 -= 1;
                let mut new_room = map.entry(position).or_default();
                new_room.doors[0] = true;
                new_room.distance = old_distance + 1;
            }
            'W' => {
                old_room.doors[3] = true;
                position.0 -= 1;
                let mut new_room = map.entry(position).or_default();
                new_room.doors[1] = true;
                new_room.distance = old_distance + 1;
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
                                .expect("hat nur eine Stelle"),
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

#[test]
fn go_performance() {
    let instructions = vec![
        Instruction {
            steps: "ENWWW".to_string(),
            successors: vec![1, 2],
        },
        Instruction {
            steps: "NEEE".to_string(),
            successors: vec![],
        },
        Instruction {
            steps: "SSE".to_string(),
            successors: vec![3, 4],
        },
        Instruction {
            steps: "EE".to_string(),
            successors: vec![],
        },
        Instruction {
            steps: "N".to_string(),
            successors: vec![],
        },
    ];

    let mut map = HashMap::new();

    for _ in 0..10_000 {
        go(&instructions, 0, (0, 0), &mut map);
    }
}
