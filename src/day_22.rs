use crate::algorithms::a_star;
use crate::algorithms::a_star::AStar;
use std::collections::HashMap;
use std::fmt;

// //Puzzle Input
const DEPTH: u32 = 11739;
const TARGET_X: u32 = 11;
const TARGET_Y: u32 = 718;

//Game Constants
const X_FACTOR: u32 = 16807;
const Y_FACTOR: u32 = 48271;
const DIVIDENT: u32 = 20183;

pub fn star_1() -> u32 {
    let mut erosion_levels: HashMap<(u32, u32), u32> = HashMap::new();
    let mut risk_level = 0;
    for x in 0..=TARGET_X {
        for y in 0..=TARGET_Y {
            let geologic_index;
            if ((x == 0) && (y == 0)) || ((x == TARGET_X) && (y == TARGET_Y)) {
                geologic_index = 0;
            } else if x == 0 {
                geologic_index = y * Y_FACTOR;
            } else if y == 0 {
                geologic_index = x * X_FACTOR;
            } else {
                geologic_index = erosion_levels.get(&(x - 1, y)).unwrap()
                    * erosion_levels.get(&(x, y - 1)).unwrap();
            }

            let erosion_level = (geologic_index + DEPTH) % DIVIDENT;
            erosion_levels.insert((x, y), erosion_level);

            risk_level += erosion_level % 3;
        }
    }
    risk_level
}

pub fn star_2() -> u32 {
    let region = Region::new(200, 1000, TARGET_X, TARGET_Y);

    let mut a_star = AStar::new(region);

    a_star
        .calc_path(
            Position {
                coordinates: (0, 0),
                gear: Gear::Torch,
            },
            Position {
                coordinates: (TARGET_X, TARGET_Y),
                gear: Gear::Torch,
            },
        )
        .expect("No path found")
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Gear {
    Neither,
    Torch,
    Climbing,
}

//Position
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Position {
    coordinates: (u32, u32),
    gear: Gear,
}

impl a_star::Cost<Position> for Position {
    fn cost(&self, position: &Position) -> u32 {
        let mut cost = 0;

        if position.gear != self.gear {
            cost += 7;
        }

        cost += (position.coordinates.0 as i32 - self.coordinates.0 as i32).abs() as u32;
        cost += (position.coordinates.1 as i32 - self.coordinates.1 as i32).abs() as u32;
        cost
    }
}

#[derive(Debug)]
enum Type {
    Rocky,
    Wet,
    Narrow,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Rocky => write!(f, "."),
            Type::Wet => write!(f, "="),
            Type::Narrow => write!(f, "|"),
        }
    }
}

struct Region {
    width: u32,
    height: u32,
    erosion_map: HashMap<(u32, u32), u32>,
}

impl Region {
    fn new(width: u32, height: u32, x_target: u32, y_target: u32) -> Region {
        let mut region = Region {
            width,
            height,
            erosion_map: HashMap::new(),
        };
        for x in 0..=width {
            for y in 0..=height {
                let geologic_index;
                if ((x == 0) && (y == 0)) || ((x == x_target) && (y == y_target)) {
                    geologic_index = 0;
                } else if x == 0 {
                    geologic_index = y * Y_FACTOR;
                } else if y == 0 {
                    geologic_index = x * X_FACTOR;
                } else {
                    geologic_index = region.erosion_map.get(&(x - 1, y)).unwrap()
                        * region.erosion_map.get(&(x, y - 1)).unwrap();
                }

                let erosion_level = (geologic_index + DEPTH) % DIVIDENT;
                region.erosion_map.insert((x, y), erosion_level);
            }
        }
        region
    }
    fn region_type_get(&self, x: u32, y: u32) -> Type {
        match self.erosion_map.get(&(x, y)).expect("Unknown coordinates") % 3 {
            0 => Type::Rocky,
            1 => Type::Wet,
            2 => Type::Narrow,
            _ => panic!("modulo 3 above 2"),
        }
    }
}

impl a_star::Neighbour<Position> for Region {
    fn neighbours_iter(&self, pos: Position) -> Vec<Position> {
        let mut list = Vec::new();

        let type_here = self.region_type_get(pos.coordinates.0, pos.coordinates.1);

        //links möglich?
        if pos.coordinates.0 > 0 {
            let neighbour_type = self.region_type_get(pos.coordinates.0 - 1, pos.coordinates.1);
            if gear_compatible(neighbour_type, pos.gear) {
                list.push(Position {
                    coordinates: (pos.coordinates.0 - 1, pos.coordinates.1),
                    gear: pos.gear,
                })
            }
        }

        //oben möglich?
        if pos.coordinates.1 > 0 {
            let neighbour_type = self.region_type_get(pos.coordinates.0, pos.coordinates.1 - 1);
            if gear_compatible(neighbour_type, pos.gear) {
                list.push(Position {
                    coordinates: (pos.coordinates.0, pos.coordinates.1 - 1),
                    gear: pos.gear,
                })
            }
        }

        //rechts
        let neighbour_type = self.region_type_get(pos.coordinates.0 + 1, pos.coordinates.1);
        if gear_compatible(neighbour_type, pos.gear) {
            list.push(Position {
                coordinates: (pos.coordinates.0 + 1, pos.coordinates.1),
                gear: pos.gear,
            })
        }

        //unten
        let neighbour_type = self.region_type_get(pos.coordinates.0, pos.coordinates.1 + 1);
        if gear_compatible(neighbour_type, pos.gear) {
            list.push(Position {
                coordinates: (pos.coordinates.0, pos.coordinates.1 + 1),
                gear: pos.gear,
            })
        }

        //Ausrüstungswechsel
        list.push(Position {
            coordinates: pos.coordinates,
            gear: other_gear(pos.gear, type_here),
        });
        list
    }
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for h in 0..=self.height {
            for w in 0..=self.width {
                write!(f, "{}", self.region_type_get(w, h))?;
            }
            writeln!(f, "")?;
        }
        writeln!(f, "")
    }
}

fn other_gear(gear: Gear, region_type: Type) -> Gear {
    match (gear, region_type) {
        (Gear::Torch, Type::Rocky) => Gear::Climbing,
        (Gear::Climbing, Type::Rocky) => Gear::Torch,
        (Gear::Neither, Type::Wet) => Gear::Climbing,
        (Gear::Climbing, Type::Wet) => Gear::Neither,
        (Gear::Neither, Type::Narrow) => Gear::Torch,
        (Gear::Torch, Type::Narrow) => Gear::Neither,
        _ => panic!("Impossible Gear"),
    }
}

fn gear_compatible(region_type: Type, gear: Gear) -> bool {
    match (gear, region_type) {
        (Gear::Torch, Type::Rocky) => true,
        (Gear::Torch, Type::Wet) => false,
        (Gear::Torch, Type::Narrow) => true,
        (Gear::Climbing, Type::Rocky) => true,
        (Gear::Climbing, Type::Wet) => true,
        (Gear::Climbing, Type::Narrow) => false,
        (Gear::Neither, Type::Rocky) => false,
        (Gear::Neither, Type::Wet) => true,
        (Gear::Neither, Type::Narrow) => true,
    }
}
