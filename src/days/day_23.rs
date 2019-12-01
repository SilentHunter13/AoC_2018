use regex::Regex;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

#[derive(Debug, Eq)]
struct Octant {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    min_z: i32,
    max_z: i32,
    bots_inside: usize,
}

fn min_coord(pos: i32, min: i32, max: i32) -> i32 {
    let mut x_rel = min - pos;
    let k_max = max - min;
    if x_rel < 0 {
        x_rel += k_max;
        if x_rel > 0 {
            x_rel = 0;
        }
    }

    x_rel.abs()
}

impl Octant {
    fn get_octants(&self) -> OctantIterator {
        let step;
        if self.is_leaf() {
            step = 8;
        } else {
            step = 0;
        }
        OctantIterator { base: &self, step }
    }
    fn intersects_with(&self, bot: &Bot) -> bool {
        let mut r = min_coord(bot.position.x, self.min_x, self.max_x);
        r += min_coord(bot.position.y, self.min_y, self.max_y);
        r += min_coord(bot.position.z, self.min_z, self.max_z);
        bot.radius >= r
    }
    fn count_bots_inside(&mut self, bots: &[Bot]) {
        self.bots_inside = bots.iter().filter(|b| self.intersects_with(b)).count()
    }
    fn is_leaf(&self) -> bool {
        self.min_x == self.max_x && self.min_y == self.max_y && self.min_z == self.max_z
    }
}

impl std::cmp::PartialEq for Octant {
    fn eq(&self, other: &Octant) -> bool {
        self.bots_inside == other.bots_inside
    }
}

impl std::cmp::PartialOrd for Octant {
    fn partial_cmp(&self, other: &Octant) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Octant {
    fn cmp(&self, other: &Octant) -> Ordering {
        match self.bots_inside.cmp(&other.bots_inside) {
            Ordering::Equal => {
                let self_size =
                    self.max_x - self.min_x + self.max_y - self.min_y + self.max_z - self.min_z;
                let other_size = other.max_x - other.min_x + other.max_y - other.min_y
                    + other.max_z
                    - other.min_z;
                Reverse(self_size).cmp(&Reverse(other_size))
            }
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
        }
    }
}

struct OctantIterator<'a> {
    base: &'a Octant,
    step: u8,
}

fn divide_points(min: i32, max: i32) -> (i32, i32) {
    let diff = (max - min) / 2;

    (min + diff, max - diff)
}

impl Iterator for OctantIterator<'_> {
    type Item = Octant;

    fn next(&mut self) -> Option<Octant> {
        match self.step {
            0 => {
                self.step += 1;
                Some(Octant {
                    min_x: divide_points(self.base.min_x, self.base.max_x).1,
                    max_x: self.base.max_x,
                    min_y: divide_points(self.base.min_y, self.base.max_y).1,
                    max_y: self.base.max_y,
                    min_z: divide_points(self.base.min_z, self.base.max_z).1,
                    max_z: self.base.max_z,
                    bots_inside: 0,
                })
            }
            1 => {
                self.step += 1;
                Some(Octant {
                    min_x: self.base.min_x,
                    max_x: divide_points(self.base.min_x, self.base.max_x).0,
                    min_y: divide_points(self.base.min_y, self.base.max_y).1,
                    max_y: self.base.max_y,
                    min_z: divide_points(self.base.min_z, self.base.max_z).1,
                    max_z: self.base.max_z,
                    bots_inside: 0,
                })
            }
            2 => {
                self.step += 1;
                Some(Octant {
                    min_x: self.base.min_x,
                    max_x: divide_points(self.base.min_x, self.base.max_x).0,
                    min_y: self.base.min_y,
                    max_y: divide_points(self.base.min_y, self.base.max_y).0,
                    min_z: divide_points(self.base.min_z, self.base.max_z).1,
                    max_z: self.base.max_z,
                    bots_inside: 0,
                })
            }
            3 => {
                self.step += 1;
                Some(Octant {
                    min_x: divide_points(self.base.min_x, self.base.max_x).1,
                    max_x: self.base.max_x,
                    min_y: self.base.min_y,
                    max_y: divide_points(self.base.min_y, self.base.max_y).0,
                    min_z: divide_points(self.base.min_z, self.base.max_z).1,
                    max_z: self.base.max_z,
                    bots_inside: 0,
                })
            }
            4 => {
                self.step += 1;
                Some(Octant {
                    min_x: divide_points(self.base.min_x, self.base.max_x).1,
                    max_x: self.base.max_x,
                    min_y: divide_points(self.base.min_y, self.base.max_y).1,
                    max_y: self.base.max_y,
                    min_z: self.base.min_z,
                    max_z: divide_points(self.base.min_z, self.base.max_z).0,
                    bots_inside: 0,
                })
            }
            5 => {
                self.step += 1;
                Some(Octant {
                    min_x: self.base.min_x,
                    max_x: divide_points(self.base.min_x, self.base.max_x).0,
                    min_y: divide_points(self.base.min_y, self.base.max_y).1,
                    max_y: self.base.max_y,
                    min_z: self.base.min_z,
                    max_z: divide_points(self.base.min_z, self.base.max_z).0,
                    bots_inside: 0,
                })
            }
            6 => {
                self.step += 1;
                Some(Octant {
                    min_x: self.base.min_x,
                    max_x: divide_points(self.base.min_x, self.base.max_x).0,
                    min_y: self.base.min_y,
                    max_y: divide_points(self.base.min_y, self.base.max_y).0,
                    min_z: self.base.min_z,
                    max_z: divide_points(self.base.min_z, self.base.max_z).0,
                    bots_inside: 0,
                })
            }
            7 => {
                self.step += 1;
                Some(Octant {
                    min_x: divide_points(self.base.min_x, self.base.max_x).1,
                    max_x: self.base.max_x,
                    min_y: self.base.min_y,
                    max_y: divide_points(self.base.min_y, self.base.max_y).0,
                    min_z: self.base.min_z,
                    max_z: divide_points(self.base.min_z, self.base.max_z).0,
                    bots_inside: 0,
                })
            }
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Bot {
    position: Point,
    radius: i32,
}

impl Bot {
    fn in_range(&self, other: &Bot) -> bool {
        let distance = self.position.distance(&other.position);

        self.radius >= distance
    }
}

pub fn star_1() -> usize {
    let bots = read_input();
    let strongest_bot = bots
        .iter()
        .max_by_key(|x| x.radius)
        .expect("there should be a largest");

    bots.iter().filter(|x| strongest_bot.in_range(x)).count()
}

pub fn star_2() -> i32 {
    let bots = read_input();

    let left_bot = bots
        .iter()
        .min_by_key(|b| b.position.x - b.radius)
        .expect("there should be a largest");
    let bottom_bot = bots
        .iter()
        .min_by_key(|b| b.position.y - b.radius)
        .expect("there should be a largest");
    let rear_bot = bots
        .iter()
        .min_by_key(|b| b.position.z - b.radius)
        .expect("there should be a largest");
    let right_bot = bots
        .iter()
        .max_by_key(|b| b.position.x + b.radius)
        .expect("there should be a largest");
    let top_bot = bots
        .iter()
        .max_by_key(|b| b.position.y + b.radius)
        .expect("there should be a largest");
    let front_bot = bots
        .iter()
        .max_by_key(|b| b.position.z + b.radius)
        .expect("there should be a largest");

    let mut base = Octant {
        min_x: left_bot.position.x - left_bot.radius,
        max_x: right_bot.position.x + right_bot.radius,
        min_y: bottom_bot.position.y - bottom_bot.radius,
        max_y: top_bot.position.y + top_bot.radius,
        min_z: rear_bot.position.z - rear_bot.radius,
        max_z: front_bot.position.z + front_bot.radius,
        bots_inside: 0,
    };

    base.count_bots_inside(&bots);

    let mut stack = BinaryHeap::new();
    stack.push(base);

    let mut best: Option<Octant> = None;
    while let Some(oct) = stack.pop() {
        //println!("{:?}", best);
        if oct.is_leaf() {
            if let Some(best_oct) = &best {
                match best_oct.cmp(&oct) {
                    Ordering::Less => best = Some(oct),
                    _ => (), //hier fehlt noch gleich
                }
            } else {
                best = Some(oct);
            }
        } else {
            for mut o in oct.get_octants() {
                o.count_bots_inside(&bots);
                //println!("{:?}", o);
                if let Some(best_oct) = &best {
                    if o.bots_inside >= best_oct.bots_inside {
                        stack.push(o);
                    }
                } else {
                    stack.push(o);
                }
            }
        }
    }

    let ret_val;
    if let Some(best) = best {
        let p = Point {
            x: best.min_x,
            y: best.min_y,
            z: best.min_z,
        };
        ret_val = p.distance(&Point { x: 0, y: 0, z: 0 });
    } else {
        ret_val = 0;
    }
    ret_val
}

fn read_input() -> Vec<Bot> {
    //1 x; 2 y; 3 z; 4 Radius
    let bot_re = Regex::new("pos=<([-0-9]+),([-0-9]+),([-0-9]+)>, r=([0-9]+)").unwrap();

    let contents =
        fs::read_to_string("./input/day_23.txt").expect("Something went wrong reading the file");

    let mut bots = Vec::new();

    for line in contents.lines() {
        if let Some(bot_match) = bot_re.captures(line) {
            bots.push(Bot {
                position: Point {
                    x: bot_match[1].parse::<i32>().unwrap(),
                    y: bot_match[2].parse::<i32>().unwrap(),
                    z: bot_match[3].parse::<i32>().unwrap(),
                },
                radius: bot_match[4].parse::<i32>().unwrap(),
            });
        }
    }
    bots
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_inside() {
        let bot = Bot {
            position: Point { x: 0, y: 0, z: 0 },
            radius: 10,
        };

        let quad = Octant {
            min_x: -5,
            max_x: 5,
            min_y: -5,
            max_y: 5,
            min_z: -5,
            max_z: 5,
            bots_inside: 0,
        };

        let result = quad.intersects_with(&bot);

        assert_eq!(result, true);
    }

    #[test]
    fn test_is_not_inside() {
        let bot = Bot {
            position: Point { x: 16, y: 0, z: 0 },
            radius: 10,
        };

        let quad = Octant {
            min_x: -5,
            max_x: 5,
            min_y: -5,
            max_y: 5,
            min_z: -5,
            max_z: 5,
            bots_inside: 0,
        };

        let result = quad.intersects_with(&bot);

        assert_eq!(result, false);
    }

    #[test]
    fn test_get_octants() {
        let root = Octant {
            min_x: 8,
            max_x: 10,
            min_y: 17,
            max_y: 19,
            min_z: -23,
            max_z: -21,
            bots_inside: 0,
        };

        let mut childs = root.get_octants();

        assert_eq!(
            childs.next(),
            Some(Octant {
                min_x: 9,
                max_x: 10,
                min_y: 18,
                max_y: 19,
                min_z: -22,
                max_z: -21,
                bots_inside: 0
            })
        );
        assert_eq!(
            childs.next(),
            Some(Octant {
                min_x: 8,
                max_x: 9,
                min_y: 18,
                max_y: 19,
                min_z: -22,
                max_z: -21,
                bots_inside: 0
            })
        );
        assert_eq!(
            childs.next(),
            Some(Octant {
                min_x: 8,
                max_x: 9,
                min_y: 17,
                max_y: 18,
                min_z: -22,
                max_z: -21,
                bots_inside: 0
            })
        );
        assert_eq!(
            childs.next(),
            Some(Octant {
                min_x: 9,
                max_x: 10,
                min_y: 17,
                max_y: 18,
                min_z: -22,
                max_z: -21,
                bots_inside: 0
            })
        );
        assert_eq!(
            childs.next(),
            Some(Octant {
                min_x: 9,
                max_x: 10,
                min_y: 18,
                max_y: 19,
                min_z: -23,
                max_z: -22,
                bots_inside: 0
            })
        );
        assert_eq!(
            childs.next(),
            Some(Octant {
                min_x: 8,
                max_x: 9,
                min_y: 18,
                max_y: 19,
                min_z: -23,
                max_z: -22,
                bots_inside: 0
            })
        );
        assert_eq!(
            childs.next(),
            Some(Octant {
                min_x: 8,
                max_x: 9,
                min_y: 17,
                max_y: 18,
                min_z: -23,
                max_z: -22,
                bots_inside: 0
            })
        );
        assert_eq!(
            childs.next(),
            Some(Octant {
                min_x: 9,
                max_x: 10,
                min_y: 17,
                max_y: 18,
                min_z: -23,
                max_z: -22,
                bots_inside: 0
            })
        );
        assert_eq!(childs.next(), None);
    }

    #[test]
    fn test_bin_heap() {
        let mut bin_heap = BinaryHeap::new();

        bin_heap.push(Octant {
            min_x: 9,
            max_x: 10,
            min_y: 17,
            max_y: 18,
            min_z: -23,
            max_z: -22,
            bots_inside: 2,
        });

        bin_heap.push(Octant {
            min_x: 9,
            max_x: 10,
            min_y: 17,
            max_y: 18,
            min_z: -23,
            max_z: -22,
            bots_inside: 3,
        });
        bin_heap.push(Octant {
            min_x: 9,
            max_x: 10,
            min_y: 17,
            max_y: 18,
            min_z: -23,
            max_z: -22,
            bots_inside: 1,
        });

        assert_eq!(
            bin_heap.pop(),
            Some(Octant {
                min_x: 9,
                max_x: 10,
                min_y: 17,
                max_y: 18,
                min_z: -23,
                max_z: -22,
                bots_inside: 3,
            })
        )
    }
}
