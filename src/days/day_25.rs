use std::convert::TryInto;
use std::fs;

pub fn star_1() -> u32 {
    let contents =
        fs::read_to_string("./input/day_25.txt").expect("Something went wrong reading the file");

    let mut coordinates: Vec<[i32; 4]> = Vec::new();

    for line in contents.lines() {
        let point = line
            .split(',')
            .map(|x| x.trim().parse::<i32>().expect("Can not parse!"))
            .collect::<Vec<i32>>()[0..4]
            .try_into()
            .expect("Genau vier Punkte benötigt!");

        coordinates.push(point);
    }

    merge_constellations(coordinates)
}

fn merge_constellations(mut coordinates: Vec<[i32; 4]>) -> u32 {
    let mut constellation_count = 0;
    loop {
        if coordinates.is_empty() {
            break;
        }

        let mut constellation = Vec::new();

        constellation.push(coordinates.swap_remove(0));
        loop {
            let mut to_move: Option<usize> = None;

            for (index, coordinate) in coordinates.iter().enumerate() {
                if in_constellation(&constellation, &coordinate) {
                    to_move = Some(index);
                }
            }

            if let Some(index) = to_move {
                constellation.push(coordinates.swap_remove(index));
            } else {
                break;
            }
        }
        constellation_count += 1;
    }
    constellation_count
}

fn in_constellation(constellation: &[[i32; 4]], point: &[i32; 4]) -> bool {
    constellation.iter().any(|x| in_range(x, point))
}

fn in_range(point1: &[i32; 4], point2: &[i32; 4]) -> bool {
    point1
        .iter()
        .zip(point2.iter())
        .map(|x| (*x.1 - *x.0).abs())
        .sum::<i32>()
        <= 3
}
