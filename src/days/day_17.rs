use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[derive(PartialEq)]
enum GroundTypes {
    Sand,
    Clay,
    Water,
    Flow,
}

#[derive(Debug)]
enum Direction {
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Drop {
    x: usize,
    y: usize,
    starts: Vec<(usize, usize)>,
    direction: Direction,
    left_bound: bool,
}

impl Drop {
    fn new(x: usize, y: usize) -> Drop {
        Drop {
            x,
            y,
            starts: vec![(x, y)],
            direction: Direction::Down,
            left_bound: false,
        }
    }
    fn copy(&self, new_start: (usize, usize)) -> Drop {
        let mut new_drop = Drop {
            x: self.x,
            y: self.y,
            starts: self.starts.to_vec(),
            direction: Direction::Down,
            left_bound: self.left_bound,
        };
        new_drop.starts.push(new_start);
        new_drop
    }
    fn overflown(&self, ground: &HashMap<(usize, usize), GroundTypes>) -> bool {
        let down = ground
            .get(&(self.x, self.y + 1))
            .unwrap_or(&GroundTypes::Sand);
        *down == GroundTypes::Flow || *down == GroundTypes::Water
    }
    fn step(
        &mut self,
        ground: &mut HashMap<(usize, usize), GroundTypes>,
        tentative: &mut Vec<Drop>,
    ) -> bool {
        let mut ret_val = true;
        let mut down_possible = false;

        let down = ground
            .entry((self.x, self.y + 1))
            .or_insert(GroundTypes::Sand);
        if down != &GroundTypes::Clay && down != &GroundTypes::Water {
            down_possible = true;
        }
        match self.direction {
            Direction::Down => {
                if down_possible {
                    *down = GroundTypes::Flow;
                    self.y += 1;
                } else {
                    self.direction = Direction::Left;
                }
            }
            Direction::Left => {
                if !down_possible {
                    let left = ground
                        .entry((self.x - 1, self.y))
                        .or_insert(GroundTypes::Sand);
                    if left != &GroundTypes::Clay {
                        *left = GroundTypes::Flow;
                        self.x -= 1;
                    } else {
                        self.direction = Direction::Right;
                        self.left_bound = true;
                    }
                } else {
                    self.direction = Direction::Right;
                    tentative.push(self.copy((self.x, self.y)));
                    self.x = self.starts.last().expect("Es gibt keinen Start!").0;
                }
            }
            Direction::Right => {
                if !down_possible {
                    let right = ground
                        .entry((self.x + 1, self.y))
                        .or_insert(GroundTypes::Sand);
                    if right != &GroundTypes::Clay {
                        *right = GroundTypes::Flow;
                        self.x += 1
                    } else if self.left_bound {
                        settle_water(self.x, self.y, ground);
                        let last_start = self.starts.pop().expect("Kein Start vorhanden!");
                        self.x = last_start.0;
                        self.y -= 1;
                        if self.y > last_start.1 {
                            self.starts.push(last_start);
                        }
                        self.direction = Direction::Left;
                        self.left_bound = false;
                    } else {
                        ret_val = false;
                    }
                } else {
                    self.starts.push((self.x, self.y));
                    self.direction = Direction::Down;
                }
            }
        }
        ret_val
    }
}

//Direction Down und Spread

pub fn star_1() -> usize {
    let mut ground = read_scan();

    let min_y = ground.keys().min_by_key(|x| x.1).expect("gibts").1;
    let max_y = ground.keys().max_by_key(|x| x.1).expect("gibts").1;
    let mut tentative: Vec<Drop> = Vec::new();
    tentative.push(Drop::new(500, 0));
    while let Some(mut drop) = tentative.pop() {
        if !drop.overflown(&ground) {
            loop {
                let go_on = drop.step(&mut ground, &mut tentative);

                if !go_on || drop.y >= max_y {
                    break;
                }
            }
        }
    }

    //show_map(&ground);
    ground
        .iter()
        .filter(|(key, value)| {
            (key.1 <= max_y && key.1 >= min_y)
                && (**value == GroundTypes::Flow || **value == GroundTypes::Water)
        })
        .count()
}

pub fn star_2() -> usize {
    let mut ground = read_scan();

    let min_y = ground.keys().min_by_key(|x| x.1).expect("gibts").1;
    let max_y = ground.keys().max_by_key(|x| x.1).expect("gibts").1;
    let mut tentative: Vec<Drop> = Vec::new();
    tentative.push(Drop::new(500, 0));
    while let Some(mut drop) = tentative.pop() {
        //println!("{:?}", drop);
        if !drop.overflown(&ground) {
            loop {
                let go_on = drop.step(&mut ground, &mut tentative);

                if !go_on || drop.y >= max_y {
                    break;
                }
            }
        }
    }

    //show_map(&ground);
    ground
        .iter()
        .filter(|(key, value)| (key.1 <= max_y && key.1 >= min_y) && **value == GroundTypes::Water)
        .count()
}

fn settle_water(x: usize, y: usize, ground: &mut HashMap<(usize, usize), GroundTypes>) {
    let mut start_x = x;
    let mut settle = false;

    loop {
        start_x -= 1;
        let left = ground.get(&(start_x, y)).unwrap_or(&GroundTypes::Sand);
        let down = ground.get(&(start_x, y + 1)).unwrap_or(&GroundTypes::Sand);

        if down == &GroundTypes::Flow || down == &GroundTypes::Sand {
            break;
        } else if left == &GroundTypes::Clay {
            settle = true;
            start_x += 1;
            break;
        }
    }

    if settle {
        for akt_x in start_x..=x {
            let soil = ground.entry((akt_x, y)).or_insert(GroundTypes::Sand);
            *soil = GroundTypes::Water;
        }
    }
}

fn read_scan() -> HashMap<(usize, usize), GroundTypes> {
    //1:feste Achse, 2: Koordinate feste Achse, 3: Startwert 4:Endwert
    let vein_re = Regex::new("([xy])=([0-9]+), [xy]=([0-9]+)..([0-9]+)").unwrap();

    let contents =
        fs::read_to_string("./input/day_17.txt").expect("Something went wrong reading the file");

    let mut ground: HashMap<(usize, usize), GroundTypes> = HashMap::new();

    for line in contents.lines() {
        if let Some(captures) = vein_re.captures(line) {
            match &captures[1] {
                "x" => {
                    let x: usize = captures[2].parse().expect("Ist keine Zahl!");
                    let start: usize = captures[3].parse().expect("Ist keine Zahl!");
                    let end: usize = captures[4].parse().expect("Ist keine Zahl!");
                    for y in start..=end {
                        ground.insert((x, y), GroundTypes::Clay);
                    }
                }
                "y" => {
                    let y: usize = captures[2].parse().expect("Ist keine Zahl!");
                    let start: usize = captures[3].parse().expect("Ist keine Zahl!");
                    let end: usize = captures[4].parse().expect("Ist keine Zahl!");
                    for x in start..=end {
                        ground.insert((x, y), GroundTypes::Clay);
                    }
                }
                _ => panic!("x oder y erforderlich!"),
            }
        }
    }
    ground
}

fn show_map(map: &HashMap<(usize, usize), GroundTypes>) {
    let min_x = map.keys().min_by_key(|x| x.0).expect("gibts").0;
    let min_y = map.keys().min_by_key(|x| x.1).expect("gibts").1;
    let max_x = map.keys().max_by_key(|x| x.0).expect("gibts").0;
    let max_y = map.keys().max_by_key(|x| x.1).expect("gibts").1;

    let mut line = String::new();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if let Some(ground) = map.get(&(x, y)) {
                match ground {
                    GroundTypes::Clay => line.push('#'),
                    GroundTypes::Flow => line.push('|'),
                    GroundTypes::Sand => line.push('.'),
                    GroundTypes::Water => line.push('~'),
                }
            } else {
                line.push('.');
            }
        }
        println!("{}", line);
        line.clear();
    }
}
