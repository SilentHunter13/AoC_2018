use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Dot {
    x: i32,
    y: i32,
    x_dt: i32,
    y_dt: i32,
}

pub fn star_1() {
    //1 X Pos; 2 Y Pos; 3 x Speed; 4 Y Speed
    let dot_re =
        Regex::new("position=< *(-?[0-9]+), *(-?[0-9]+)> velocity=< *(-?[0-9]+), *(-?[0-9]+)>")
            .unwrap();

    let contents =
        fs::read_to_string("./input/day_10.txt").expect("Something went wrong reading the file");

    let mut dots: Vec<Dot> = Vec::new();

    for dot in contents.lines() {
        let dot_match = dot_re.captures(dot).unwrap();

        let x = dot_match[1].parse::<i32>().unwrap();
        let y = dot_match[2].parse::<i32>().unwrap();
        let x_dt = dot_match[3].parse::<i32>().unwrap();
        let y_dt = dot_match[4].parse::<i32>().unwrap();

        let dot = Dot { x, y, x_dt, y_dt };
        dots.push(dot);
    }

    //Punkte um einen Schritt weiter simulieren
    let mut counter = 0;
    let mut last_height = get_height(&dots);
    let seconds = loop {
        for dot in dots.iter_mut() {
            dot.x = dot.x + dot.x_dt;
            dot.y = dot.y + dot.y_dt;
        }
        counter += 1;
        let height = get_height(&dots);

        //Die Nachricht erscheint bei der kleinsten Ausdehnung in der Höhe
        if height > last_height {
            break counter - 1;
        }
        last_height = height;
    };

    //Wir waren einen Schritt zu weit deshalb wieder zurück simulieren
    for dot in dots.iter_mut() {
        dot.x = dot.x - dot.x_dt;
        dot.y = dot.y - dot.y_dt;
    }

    println!("Day 10:");
    plot_dots(&mut dots);
    println!(
        "The elves would have needed to wait for {} seconds!",
        seconds
    );
}

fn plot_dots(dots: &mut Vec<Dot>) {
    //Punkte sortieren
    dots.sort_by_key(|t| t.x);
    dots.sort_by_key(|t| t.y);

    let x_min = dots.iter().min_by_key(|t| t.x).unwrap().x;
    let x_max = dots.iter().max_by_key(|t| t.x).unwrap().x;
    let mut last_y = dots[0].y;
    let line_length = (x_max - x_min + 1) as usize;

    let mut line = vec!['.'; line_length];
    for dot in dots.iter() {
        if dot.y != last_y {
            println!("{:?}", line.iter().collect::<String>());

            for x in &mut line {
                *x = '.';
            }
        }

        let index = (dot.x - x_min) as usize;
        line[index] = '#';
        last_y = dot.y;
    }
    println!("{:?}", line.iter().collect::<String>());
}

fn get_height(dots: &Vec<Dot>) -> i32 {
    let y_min = dots.iter().min_by_key(|t| t.y).unwrap().y;
    let y_max = dots.iter().max_by_key(|t| t.y).unwrap().y;

    y_max - y_min
}
