use core::cmp::Reverse;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

const MAP_SIZE: usize = 32;
const ATTACK_POWER: u32 = 3;

#[derive(Debug, Clone, Copy, std::cmp::PartialEq, std::cmp::PartialOrd)]
enum Party {
    Goblin,
    Elf,
}

#[derive(Debug, Clone, Copy, std::cmp::PartialEq, std::cmp::PartialOrd)]
struct Fighter {
    hit_points: u32,
    party: Party,
}

#[derive(Clone, Copy, std::cmp::PartialEq, std::cmp::PartialOrd)]
enum AreaType {
    Wall,
    Cavern,
}

#[derive(Clone, Copy, std::cmp::PartialEq, std::cmp::PartialOrd)]
enum Entity {
    Area(AreaType),
    Fighter(Fighter),
}

pub fn star_1() -> u32 {
    let mut map = read_map();
    show_map(&map);
    let mut full_round = false;
    let mut rounds = 0;
    loop {
        let running_order = get_running_order(&map);
        let mut died = HashSet::new();

        for mut starter in running_order {
            full_round = targets_available(&map);
            //println!("F {:?}", full_round);
            if !died.contains(&starter) {
                //Bewegen
                if let Some(target) = find_nearest_target(&map, starter) {
                    //Schritt machen
                    if map[target.1][target.0] == Entity::Area(AreaType::Cavern) {
                        map[target.1][target.0] = map[starter.1][starter.0];
                        map[starter.1][starter.0] = Entity::Area(AreaType::Cavern);
                        starter = target;
                    }
                }

                //Kämpfen
                if let Some(defender_loc) = get_weakest_opponent_in_range(&map, &starter) {
                    if let Entity::Fighter(mut defender) = &mut map[defender_loc.1][defender_loc.0]
                    {
                        //println!("A{:?} D{:?}", starter, defender_loc);
                        if defender.hit_points <= ATTACK_POWER {
                            map[defender_loc.1][defender_loc.0] = Entity::Area(AreaType::Cavern);
                            died.insert(defender_loc);
                        } else {
                            defender.hit_points -= ATTACK_POWER;
                            map[defender_loc.1][defender_loc.0] = Entity::Fighter(defender);
                        }
                    }
                }
            }
        }
        show_map(&map);
        if full_round {
            rounds += 1;
            println!("{:?}", rounds);
        }
        if !targets_available(&map) {
            break;
        }
    }
    calculate_outcome(&map, rounds)
}

fn read_map() -> Vec<Vec<Entity>> {
    let contents =
        fs::read_to_string("./input/day_15.txt").expect("Something went wrong reading the file");

    let mut map = vec![vec![Entity::Area(AreaType::Wall); MAP_SIZE]; MAP_SIZE];

    for (row, line) in contents.lines().enumerate() {
        for (column, spot) in line.chars().enumerate() {
            match spot {
                '#' => map[row][column] = Entity::Area(AreaType::Wall),
                '.' => map[row][column] = Entity::Area(AreaType::Cavern),
                'G' => {
                    map[row][column] = Entity::Fighter(Fighter {
                        hit_points: 200,
                        party: Party::Goblin,
                    })
                }
                'E' => {
                    map[row][column] = Entity::Fighter(Fighter {
                        hit_points: 200,
                        party: Party::Elf,
                    })
                }
                _ => panic!("Unbekannte Zelle!"),
            }
        }
    }
    map
}

fn calculate_outcome(map: &[Vec<Entity>], rounds: u32) -> u32 {
    let mut hit_points = 0;
    for row in map {
        for spot in row {
            if let Entity::Fighter(unit) = spot {
                hit_points += unit.hit_points;
            }
        }
    }
    println!("{:?} {:?}", hit_points, rounds);

    hit_points * rounds
}

fn targets_available(map: &[Vec<Entity>]) -> bool {
    let mut goblins_available = false;
    let mut elves_available = false;
    for row in map {
        for spot in row {
            match spot {
                Entity::Fighter(unit) => match unit.party {
                    Party::Goblin => goblins_available = true,
                    Party::Elf => elves_available = true,
                },
                _ => (),
            }
        }
    }
    goblins_available && elves_available
}

fn get_weakest_opponent_in_range(
    map: &[Vec<Entity>],
    starter: &(usize, usize),
) -> Option<(usize, usize)> {
    let mut ret_val: Option<(usize, usize)> = None;
    if let Entity::Fighter(subject) = map[starter.1][starter.0] {
        for spot in get_neighbours(*starter) {
            if let Entity::Fighter(object) = map[spot.1][spot.0] {
                if is_opponent(&subject, &object) {
                    if let Some(opponent) = ret_val {
                        if let Entity::Fighter(opponent) = map[opponent.1][opponent.0] {
                            if is_weaker(&opponent, &object) {
                                ret_val = Some(spot);
                            }
                        }
                    } else {
                        ret_val = Some(spot);
                    }
                }
            }
        }
    }
    ret_val
}

fn is_weaker(subject: &Fighter, object: &Fighter) -> bool {
    object.hit_points < subject.hit_points
}

fn get_running_order(map: &[Vec<Entity>]) -> Vec<(usize, usize)> {
    let mut running_order = Vec::new();

    for (y, row) in map.iter().enumerate() {
        for (x, spot) in row.iter().enumerate() {
            match spot {
                Entity::Fighter(_) => running_order.push((x, y)),
                _ => (),
            }
        }
    }
    running_order
}

fn find_nearest_target(map: &[Vec<Entity>], start: (usize, usize)) -> Option<(usize, usize)> {
    match map[start.1][start.0] {
        Entity::Fighter(starter) => {
            let mut tentative: VecDeque<(usize, usize)> = VecDeque::new();
            let mut visited: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
            let mut ret_val = None;
            let mut possibilities = Vec::new();

            visited.insert(start, (0, 0));
            tentative.push_back(start);

            'outer: while let Some(this) = tentative.pop_front() {
                for neighbour in get_neighbours(this) {
                    if !visited.contains_key(&neighbour) {
                        visited.insert(neighbour, this);
                        if let Entity::Fighter(n) = map[neighbour.1][neighbour.0] {
                            if is_opponent(&starter, &n) {
                                //Backtracking
                                let mut predecessor = neighbour;
                                let mut steps = 0;
                                loop {
                                    let pre_predecessor =
                                        *visited.get(&predecessor).expect("kein Vorgänger!");
                                    if pre_predecessor == start {
                                        possibilities.push((neighbour, predecessor, steps));
                                        break;
                                    }
                                    steps += 1;
                                    predecessor = pre_predecessor;
                                }
                            }
                        } else if is_cavern(&map, neighbour) {
                            tentative.push_back(neighbour);
                        }
                    }
                }
            }

            possibilities.sort_by_key(|x| Reverse((x.0).0));
            possibilities.sort_by_key(|x| Reverse((x.0).1));
            possibilities.sort_by_key(|x| Reverse(x.2));

            if let Some(r) = possibilities.pop() {
                ret_val = Some(r.1);
            }
            ret_val
        }
        _ => None,
    }
}

// Sollte diese Funktion die Karte kennen?
fn get_neighbours(start: (usize, usize)) -> NeighbourGenerator {
    NeighbourGenerator { start, state: 0 }
}

struct NeighbourGenerator {
    start: (usize, usize),
    state: u8,
}

impl Iterator for NeighbourGenerator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<(usize, usize)> {
        let mut ret_val = None;
        loop {
            match self.state {
                0 => {
                    self.state += 1;
                    if self.start.1 > 0 {
                        ret_val = Some((self.start.0, self.start.1 - 1));
                        break;
                    }
                }
                1 => {
                    self.state += 1;
                    if self.start.0 > 0 {
                        ret_val = Some((self.start.0 - 1, self.start.1));
                        break;
                    }
                }
                2 => {
                    self.state += 1;
                    if self.start.0 + 1 < MAP_SIZE {
                        ret_val = Some((self.start.0 + 1, self.start.1));
                        break;
                    }
                }
                3 => {
                    self.state += 1;
                    if self.start.1 + 1 < MAP_SIZE {
                        ret_val = Some((self.start.0, self.start.1 + 1));
                        break;
                    }
                }
                _ => {
                    break;
                }
            }
        }

        ret_val
    }
}

fn is_cavern(map: &[Vec<Entity>], position: (usize, usize)) -> bool {
    Entity::Area(AreaType::Cavern) == map[position.1][position.0]
}

//Parameter in Fighter struct ändern
fn is_opponent(subject: &Fighter, object: &Fighter) -> bool {
    subject.party != object.party
}

fn show_map(map: &[Vec<Entity>]) {
    for row in map {
        let mut area_line = String::new();
        let mut hp_line = String::new();
        for spot in row {
            match spot {
                Entity::Area(AreaType::Wall) => area_line.push('#'),
                Entity::Area(AreaType::Cavern) => area_line.push('.'),
                Entity::Fighter(fighter) => match fighter.party {
                    Party::Goblin => {
                        area_line.push('G');
                        hp_line.push_str(&format!(" G({:?})", fighter.hit_points));
                    }
                    Party::Elf => {
                        area_line.push('E');
                        hp_line.push_str(&format!(" E({:?})", fighter.hit_points));
                    }
                },
            };
        }
        area_line.push_str(&hp_line);
        println!("{}", area_line);
        area_line.clear();
        hp_line.clear();
    }
}
