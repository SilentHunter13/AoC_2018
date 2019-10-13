use regex::Regex;
use std::fs;

#[derive(Debug, Clone)]
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

#[derive(Debug)]
struct Bot {
    position: Point,
    radius: i32,
}

impl Bot {
    fn in_range(&self, other: &Bot) -> bool {
        let distance = self.position.distance(&other.position);

        self.radius >= distance
    }
    fn point_in_range(&self, point: &Point) -> bool {
        let distance = self.position.distance(&point);

        self.radius >= distance
    }
    fn iter_edge(&self) -> BotEdge {
        let none = self.position.distance(&Point { x: 0, y: 0, z: 0 }) < self.radius;
        let sign_x = if self.position.x < 0 { 1 } else { -1 };
        let sign_y = if self.position.y < 0 { 1 } else { -1 };
        let sign_z = if self.position.z < 0 { 1 } else { -1 };

        BotEdge {
            position: self.position.clone(),
            radius: self.radius,
            akt_x: self.radius,
            akt_y: 0,
            akt_z: 0,
            sign_x,
            sign_y,
            sign_z,
            start: true,
            none,
        }
    }
}

struct BotEdge {
    position: Point,
    radius: i32,
    akt_x: i32,
    akt_y: i32,
    akt_z: i32,
    sign_x: i32,
    sign_y: i32,
    sign_z: i32,
    start: bool,
    none: bool,
}

impl Iterator for BotEdge {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        if self.none {
            return None;
        }
        //Der Rand des Bereiches ist zusammengesetzt aus acht Dreiecken
        if self.start {
            self.start = false;
            return Some(Point {
                x: self.sign_x * self.akt_x + self.position.x,
                y: self.sign_y * self.akt_y + self.position.y,
                z: self.sign_z * self.akt_z + self.position.z,
            });
        }
        //nur die Eckpunkte eines Dreiecks zurückgeben
        if self.akt_x == self.radius {
            self.akt_x = 0;
            self.akt_y = self.radius;
        } else if self.akt_y == self.radius {
            self.akt_y = 0;
            self.akt_z = self.radius;
        } else {
            return None;
        }
        //es fehlen die Punkte in der nähe von Schnitten mit anderen Bots

        Some(Point {
            x: self.sign_x * self.akt_x + self.position.x,
            y: self.sign_y * self.akt_y + self.position.y,
            z: self.sign_z * self.akt_z + self.position.z,
        })
    }
}

fn bots_in_range(point: &Point, bots: &[Bot]) -> u32 {
    bots.iter()
        .filter(|bot| bot.point_in_range(point))
        .fold(0, |sum, _| sum + 1)
}

pub fn star_1() -> u32 {
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

    let strongest_bot = bots
        .iter()
        .max_by_key(|x| x.radius)
        .expect("there should be a largest");

    bots.iter()
        .filter(|x| strongest_bot.in_range(x))
        .fold(0, |sum, _| sum + 1)
}

pub fn star_2() -> i32 {
    //1 x; 2 y; 3 z; 4 Radius
    let bot_re = Regex::new("pos=<([-0-9]+),([-0-9]+),([-0-9]+)>, r=([0-9]+)").unwrap();

    let contents = fs::read_to_string("./input/day_23_test3.txt")
        .expect("Something went wrong reading the file");

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

    let mut closeset_point: (Point, u32) = (Point { x: 0, y: 0, z: 0 }, 0);

    for bot in &bots {
        //println!("{:?}", bot);
        //iterieren über die Punkte des Randes
        for point in bot.iter_edge() {
            //println!("{:?}", point);
            //Anzahl Bots in Reichweite bestimmen
            let bots_in_range = bots_in_range(&point, &bots);
            //println!("{:?}", bots_in_range);
            //wenn mehr als bisheriger Punkt
            if bots_in_range > closeset_point.1 {
                closeset_point = (point, bots_in_range);
            } else if bots_in_range == closeset_point.1 {
                //wenn der aktuelle Punkt näher am Ursprung liegt
                if point.distance(&Point { x: 0, y: 0, z: 0 })
                    < closeset_point.0.distance(&Point { x: 0, y: 0, z: 0 })
                {
                    closeset_point = (point, bots_in_range);
                }
            }
        }
        //println!("{:?}", closeset_point);
    }
    closeset_point.0.distance(&Point { x: 0, y: 0, z: 0 })
}
