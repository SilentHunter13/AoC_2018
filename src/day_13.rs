use std::fs;

const MAP_SIZE: usize = 150;

#[derive(Clone, Copy, Debug)]
enum Rail {
    Straight,
    Intersection,
    Bend1,
    Bend2,
    Offrail,
}

#[derive(Debug)]
struct Train {
    x: usize,
    y: usize,
    direction: (i8, i8),
    next_turn: i8,
}

pub fn star_1() -> (usize, usize) {
    let contents =
        fs::read_to_string("./input/day_13.txt").expect("Something went wrong reading the file");

    let mut rails = [[Rail::Offrail; MAP_SIZE]; MAP_SIZE];
    let mut trains: Vec<Train> = Vec::new();

    //Gleise und Züge parsen
    for (y, line) in contents.lines().enumerate() {
        for (x, spot) in line.chars().enumerate() {
            match spot {
                //Gleise
                '-' => rails[x][y] = Rail::Straight,
                '|' => rails[x][y] = Rail::Straight,
                '+' => rails[x][y] = Rail::Intersection,
                '/' => rails[x][y] = Rail::Bend1,
                '\\' => rails[x][y] = Rail::Bend2,
                //Züge
                '<' => {
                    rails[x][y] = Rail::Straight;
                    let train = Train {
                        x,
                        y,
                        direction: (-1, 0),
                        next_turn: -1,
                    };
                    trains.push(train);
                }
                '>' => {
                    rails[x][y] = Rail::Straight;
                    let train = Train {
                        x,
                        y,
                        direction: (1, 0),
                        next_turn: -1,
                    };
                    trains.push(train);
                }
                '^' => {
                    rails[x][y] = Rail::Straight;
                    let train = Train {
                        x,
                        y,
                        direction: (0, -1),
                        next_turn: -1,
                    };
                    trains.push(train);
                }
                'v' => {
                    rails[x][y] = Rail::Straight;
                    let train = Train {
                        x,
                        y,
                        direction: (0, 1),
                        next_turn: -1,
                    };
                    trains.push(train);
                }
                _ => {}
            }
        }
    }

    //Züge simulieren
    loop {
        let mut new_trains: Vec<Train> = Vec::new();
        let mut collision: Option<(usize, usize)> = None;
        for (n, train) in trains.iter().enumerate() {
            let new_train = train.step(&rails);
            if trains[n..].contains(&new_train) || new_trains.contains(&new_train) {
                collision = Some((new_train.x, new_train.y));
                break;
            }
            new_trains.push(new_train);
        }
        if let Some(c) = collision {
            break c;
        }
        new_trains.sort_unstable_by_key(|x| x.x);
        new_trains.sort_by_key(|x| x.y);
        trains = new_trains;
    }
}

impl Train {
    fn step(&self, rails: &[[Rail; MAP_SIZE]; MAP_SIZE]) -> Train {
        let mut new_train = Train {
            x: self.x,
            y: self.y,
            direction: self.direction,
            next_turn: self.next_turn,
        };
        let rail = rails[new_train.x][new_train.y];
        match rail {
            Rail::Straight => {}
            Rail::Intersection => {
                let turn_matrix;
                match new_train.next_turn {
                    -1 => {
                        turn_matrix = (0, 1, -1, 0);
                        new_train.next_turn = 0;
                    }
                    0 => {
                        turn_matrix = (1, 0, 0, 1);
                        new_train.next_turn = 1;
                    }
                    1 => {
                        turn_matrix = (0, -1, 1, 0);
                        new_train.next_turn = -1;
                    }
                    _ => panic!("invalid next turn"),
                }
                new_train.direction = (
                    (new_train.direction.0 * turn_matrix.0)
                        + (new_train.direction.1 * turn_matrix.1),
                    (new_train.direction.0 * turn_matrix.2)
                        + (new_train.direction.1 * turn_matrix.3),
                )
            }
            Rail::Bend1 => match new_train.direction {
                (0, -1) => new_train.direction = (1, 0),
                (-1, 0) => new_train.direction = (0, 1),
                (0, 1) => new_train.direction = (-1, 0),
                (1, 0) => new_train.direction = (0, -1),
                _ => panic!("impossible direction on Bend1"),
            },
            Rail::Bend2 => match new_train.direction {
                (0, -1) => new_train.direction = (-1, 0),
                (1, 0) => new_train.direction = (0, 1),
                (0, 1) => new_train.direction = (1, 0),
                (-1, 0) => new_train.direction = (0, -1),
                _ => panic!("impossible direction on Bend2"),
            },
            Rail::Offrail => panic!("offrail"),
        }
        //Bewegen
        new_train.x = (new_train.x as i32 + i32::from(new_train.direction.0)) as usize;
        new_train.y = (new_train.y as i32 + i32::from(new_train.direction.1)) as usize;
        new_train
    }
}

impl std::cmp::PartialEq for Train {
    fn eq(&self, other: &Train) -> bool {
        (self.x == other.x) && (self.y == other.y)
    }
}
