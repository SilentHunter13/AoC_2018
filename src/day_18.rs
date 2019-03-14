use std::collections::HashMap;
use std::fmt;
use std::fs;

const AREA_SIZE: i32 = 50;
//const AREA_SIZE: i32 = 10;

#[derive(Debug, Clone, Copy)]
enum AreaTypes {
    Open,
    Trees,
    Lumberyard,
}

impl fmt::Display for AreaTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AreaTypes::Open => write!(f, "."),
            AreaTypes::Trees => write!(f, "|"),
            AreaTypes::Lumberyard => write!(f, "#"),
        }
    }
}

pub fn star_1() -> u32 {
    simulate_acres(10)
}

pub fn star_2() -> u32 {
    simulate_acres(1000000000)
}

fn simulate_acres(minutes: i32) -> u32 {
    let contents =
        fs::read_to_string("./input/day_18.txt").expect("Something went wrong reading the file");

    let mut lumber_collection_area: HashMap<(i32, i32), AreaTypes> = HashMap::new();

    for (y, line) in contents.lines().enumerate() {
        for (x, acre) in line.chars().enumerate() {
            match acre {
                '.' => {
                    lumber_collection_area.insert((x as i32, y as i32), AreaTypes::Open);
                }
                '|' => {
                    lumber_collection_area.insert((x as i32, y as i32), AreaTypes::Trees);
                }
                '#' => {
                    lumber_collection_area.insert((x as i32, y as i32), AreaTypes::Lumberyard);
                }
                _ => panic!("Unkown area type"),
            }
        }
    }

    for _ in 0..minutes {
        let mut new_area: HashMap<(i32, i32), AreaTypes> = HashMap::new();
        for x in 0..AREA_SIZE {
            for y in 0..AREA_SIZE {
                let neighbours = neighbours_get(x, y, &lumber_collection_area);
                let mut new_type: AreaTypes = lumber_collection_area
                    .get(&(x, y))
                    .expect("Unkown location")
                    .clone();
                match new_type {
                    AreaTypes::Open => {
                        if neighbours.1 >= 3 {
                            new_type = AreaTypes::Trees;
                        }
                    }
                    AreaTypes::Trees => {
                        if neighbours.2 >= 3 {
                            new_type = AreaTypes::Lumberyard;
                        }
                    }
                    AreaTypes::Lumberyard => {
                        if neighbours.2 < 1 || neighbours.1 < 1 {
                            new_type = AreaTypes::Open;
                        }
                    }
                }
                new_area.insert((x, y), new_type);
            }
        }
        lumber_collection_area = new_area;
    }

    let mut trees = 0;
    let mut lumberyard = 0;
    for x in 0..AREA_SIZE {
        for y in 0..AREA_SIZE {
            if let Some(area_type) = lumber_collection_area.get(&(x, y)) {
                match area_type {
                    AreaTypes::Trees => {
                        trees += 1;
                    }
                    AreaTypes::Lumberyard => {
                        lumberyard += 1;
                    }
                    _ => {}
                }
            }
        }
    }
    trees * lumberyard
}

fn neighbours_get(x: i32, y: i32, area: &HashMap<(i32, i32), AreaTypes>) -> (u32, u32, u32) {
    let mut open = 0;
    let mut trees = 0;
    let mut lumberyard = 0;
    for x_diff in -1..=1 {
        for y_diff in -1..=1 {
            if (x_diff != 0) || (y_diff != 0) {
                if let Some(area_type) = area.get(&(x + x_diff, y + y_diff)) {
                    match area_type {
                        AreaTypes::Open => {
                            open += 1;
                        }
                        AreaTypes::Trees => {
                            trees += 1;
                        }
                        AreaTypes::Lumberyard => {
                            lumberyard += 1;
                        }
                    }
                }
            }
        }
    }

    (open, trees, lumberyard)
}

fn plot_acres(acres: &HashMap<(i32, i32), AreaTypes>) {
    for y in 0..AREA_SIZE {
        for x in 0..AREA_SIZE {
            print!("{}", acres.get(&(x, y)).expect("The acre must be there"));
        }
        println!("");
    }
    println!("");
}
