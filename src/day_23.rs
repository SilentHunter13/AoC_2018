use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Bot {
    x: i32,
    y: i32,
    z: i32,
    radius: i32,
}

impl Bot {
    fn in_range(&self, other: &Bot) -> bool {
        let distance =
            (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs();

        self.radius >= distance
    }
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
                x: bot_match[1].parse::<i32>().unwrap(),
                y: bot_match[2].parse::<i32>().unwrap(),
                z: bot_match[3].parse::<i32>().unwrap(),
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
