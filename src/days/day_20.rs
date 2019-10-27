use std::collections::HashMap;
use std::fs;
use std::iter::FromIterator;

#[derive(Debug)]
struct State {
    position: (i32, i32),
    alternatives: Vec<usize>,
    brace_depth: u32,
}

pub fn star_1() {
    let contents =
        fs::read_to_string("./input/day_20.txt").expect("Something went wrong reading the file");

    let directions = Vec::from_iter(contents.chars());

    let mut stack: Vec<State> = Vec::new();
    let mut index = 0;
    let mut position = (0, 0);
    let mut map: HashMap<(i32, i32), [bool; 4]> = HashMap::new();
    let mut brace_depth = 0;
    let mut wait_for_closing_brace = false;
    let mut ignore_until = 0;

    loop {
        let direction = directions[index];

        match direction {
            '(' => {
                brace_depth += 1;
            }
            ')' => {
                if brace_depth == ignore_until {
                    wait_for_closing_brace = false;
                }
                brace_depth -= 1;
            }
            _ => (),
        }

        if !wait_for_closing_brace {
            match direction {
                '$' => {
                    let mut found = false;
                    let mut ready = false;

                    while !ready {
                        if let Some(mut state) = stack.pop() {
                            if let Some(alternative) = state.alternatives.pop() {
                                index = alternative;
                                position = state.position;
                                brace_depth = state.brace_depth;
                                stack.push(state);
                                found = true;
                                ready = true;
                            }
                        } else {
                            ready = true;
                        }
                    }
                    if !found {
                        break;
                    }
                }
                '(' => {
                    stack.push(State {
                        position,
                        alternatives: Vec::new(),
                        brace_depth,
                    });
                }
                '|' => {
                    let state_index = stack
                        .iter()
                        .position(|x| x.brace_depth == brace_depth)
                        .expect("Es muss eine Klammer offen sein!");
                    let last_state = &mut stack[state_index];
                    if !last_state.alternatives.contains(&index) {
                        last_state.alternatives.push(index);
                    }
                    wait_for_closing_brace = true;
                    ignore_until = brace_depth;
                }
                'N' => {
                    let old_room = map.entry(position).or_default();
                    old_room[0] = true;
                    position.1 += 1;
                    let new_room = map.entry(position).or_default();
                    new_room[2] = true;
                }
                'E' => {
                    let old_room = map.entry(position).or_default();
                    old_room[1] = true;
                    position.0 += 1;
                    let new_room = map.entry(position).or_default();
                    new_room[3] = true;
                }
                'S' => {
                    let old_room = map.entry(position).or_default();
                    old_room[2] = true;
                    position.1 -= 1;
                    let new_room = map.entry(position).or_default();
                    new_room[0] = true;
                }
                'W' => {
                    let old_room = map.entry(position).or_default();
                    old_room[3] = true;
                    position.0 -= 1;
                    let new_room = map.entry(position).or_default();
                    new_room[1] = true;
                }
                _ => {} //alles andere ignorieren
            }
        }
        index += 1;
    }

    show_map(&map);
}

//fn load_map()

fn show_map(map: &HashMap<(i32, i32), [bool; 4]>) {
    let min_x = (map.iter().min_by_key(|x| (x.0).0).expect("gibts").0).0;
    let min_y = (map.iter().min_by_key(|x| (x.0).1).expect("gibts").0).1;
    let max_x = (map.iter().max_by_key(|x| (x.0).0).expect("gibts").0).0;
    let max_y = (map.iter().max_by_key(|x| (x.0).1).expect("gibts").0).1;

    let mut line1 = String::new();
    let mut line2 = String::new();

    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            if let Some(room) = map.get(&(x, y)) {
                if x == 0 && y == 0 {
                    line1.push('X');
                } else {
                    line1.push('.');
                }
                if room[1] {
                    line1.push('|');
                } else {
                    line1.push('#');
                }

                if room[2] {
                    line2.push('-');
                } else {
                    line2.push('#');
                }
                line2.push('#');
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
