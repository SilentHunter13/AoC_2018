use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::fs;
use std::hash::Hash;
use std::hash::Hasher;

const AREA_SIZE: usize = 50;

#[derive(Debug, Clone, Copy, Hash)]
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

struct Area {
    area: [[AreaTypes; AREA_SIZE]; AREA_SIZE],
}

impl Area {
    fn new() -> Area {
        Area {
            area: [[AreaTypes::Open; AREA_SIZE]; AREA_SIZE],
        }
    }
    fn get(&self, x: usize, y: usize) -> Option<AreaTypes> {
        let column = self.area.get(x)?;
        let a = column.get(y)?;
        Some(*a)
    }
    fn count_neighbours(&self, x: usize, y: usize) -> (u32, u32, u32) {
        let mut open = 0;
        let mut trees = 0;
        let mut lumberyard = 0;
        for x_diff in -1..=1 {
            for y_diff in -1..=1 {
                if (x_diff != 0) || (y_diff != 0) {
                    if let Some(area_type) =
                        self.get((x as i32 + x_diff) as usize, (y as i32 + y_diff) as usize)
                    {
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
    // fn plot_acres(&self) {
    //     for y in 0..AREA_SIZE {
    //         for x in 0..AREA_SIZE {
    //             print!("{}", self.area[x][y]);
    //         }
    //         println!("");
    //     }
    //     println!("");
    // }
}

pub fn star_1() -> u32 {
    simulate_acres(10) //10 minutes
}

pub fn star_2() -> u32 {
    simulate_acres(1_000_000_000) //1000000000 minutes
}

fn simulate_acres(minutes: usize) -> u32 {
    let contents =
        fs::read_to_string("./input/day_18.txt").expect("Something went wrong reading the file");

    let mut lumber_collection_area = Area::new();

    for (y, line) in contents.lines().enumerate() {
        for (x, acre) in line.chars().enumerate() {
            match acre {
                '.' => {
                    lumber_collection_area.area[x][y] = AreaTypes::Open;
                }
                '|' => {
                    lumber_collection_area.area[x][y] = AreaTypes::Trees;
                }
                '#' => {
                    lumber_collection_area.area[x][y] = AreaTypes::Lumberyard;
                }
                _ => panic!("Unkown area type"),
            }
        }
    }

    //lumber_collection_area.plot_acres();

    let mut resource_value = 0;
    let mut hashes = Vec::new();
    let mut resource_values = Vec::new();

    for minute in 0..minutes {
        let mut new_area = Area::new();
        let mut hasher = DefaultHasher::new();
        let mut trees = 0;
        let mut lumberyard = 0;
        for x in 0..AREA_SIZE {
            for y in 0..AREA_SIZE {
                let neighbours = lumber_collection_area.count_neighbours(x, y);
                let mut new_type: AreaTypes = lumber_collection_area.area[x][y];
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
                match new_type {
                    AreaTypes::Trees => {
                        trees += 1;
                    }
                    AreaTypes::Lumberyard => {
                        lumberyard += 1;
                    }
                    _ => {}
                }
                new_type.hash(&mut hasher);
                new_area.area[x][y] = new_type;
            }
        }
        lumber_collection_area = new_area;

        let hash = hasher.finish();

        resource_value = trees * lumberyard;

        //Wenn der Hash bereits in der Liste ist, liegt mit großer Wahrscheinlichkeit ein Zyklus vor
        if hashes.contains(&hash) {
            if let Some(first) = hashes.iter().position(|x| x == &hash) {
                //minutes -1, weil Index benötigt wird
                let index = (minutes - 1 - first) % (minute - first) + first;
                //index ist sicher vorhanden
                resource_value = *resource_values.get(index).unwrap();
            }
            break;
        }

        hashes.push(hash);
        resource_values.push(resource_value);
    }

    resource_value
}
