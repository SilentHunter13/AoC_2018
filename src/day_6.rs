use std::fs;

const THRESHOLD: u32 = 10000;

pub fn star_1() -> u32 {
    let contents =
        fs::read_to_string("./input/day_6.txt").expect("Something went wrong reading the file");

    let mut places: Vec<(u32, u32)> = Vec::new();
    let mut min_x = u32::max_value();
    let mut min_y = u32::max_value();
    let mut max_x = 0;
    let mut max_y = 0;

    //input parsen
    for line in contents.lines() {
        let mut coordinates = line
            .split(',')
            .map(|x| x.trim().parse::<u32>().expect("Parse Error!"));

        let x = coordinates.next().expect("no x");
        let y = coordinates.next().expect("no y");

        min_x = x.min(min_x);
        min_y = x.min(min_y);
        max_x = x.max(max_x);
        max_y = x.max(max_y);

        places.push((x, y));
    }

    #[derive(Clone, Debug)]
    struct Place {
        size: u32,
        infinite: bool,
    };

    let mut results = vec![
        Place {
            size: 0,
            infinite: false
        };
        places.len()
    ];

    //für jeden Punkt den Besitzer bestimmen
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let owner = get_owner(&places, x, y);
            if let Ok(index) = owner {
                results[index].size += 1;
            }

            //Wenn der Punkt auf dem Rand liegt ist die Fläche unbegrenzt
            if x == min_x || x == max_x || y == min_y || y == max_y {
                if let Ok(index) = owner {
                    results[index].infinite = true;
                }
            }
        }
    }

    //unbegrenzte Flächen entfernen
    results.retain(|x| !x.infinite);
    //größte Fläche finden
    let result = results.iter().max_by_key(|x| x.size);

    if let Some(element) = result {
        element.size
    } else {
        0
    }
}

pub fn star_2() -> u32 {
    let contents =
        fs::read_to_string("./input/day_6.txt").expect("Something went wrong reading the file");

    let mut places: Vec<(u32, u32)> = Vec::new();
    let mut min_x = u32::max_value();
    let mut min_y = u32::max_value();
    let mut max_x = 0;
    let mut max_y = 0;

    //input parsen
    for line in contents.lines() {
        let mut coordinates = line
            .split(',')
            .map(|x| x.trim().parse::<u32>().expect("Parse Error!"));

        let x = coordinates.next().expect("no x");
        let y = coordinates.next().expect("no y");

        min_x = x.min(min_x);
        min_y = x.min(min_y);
        max_x = x.max(max_x);
        max_y = x.max(max_y);

        places.push((x, y));
    }

    let mut size = 0;

    //für jeden Punkt prüfen ob er in der gesuchten Region liegt
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if is_in_region(&places, x, y) {
                size += 1;
            }
        }
    }

    size
}

fn get_owner(places: &[(u32, u32)], x: u32, y: u32) -> Result<usize, ()> {
    let mut min_distance = u32::max_value();
    let mut ret_val = Err(());
    for (i, place) in places.iter().enumerate() {
        let distance = x.max(place.0) - x.min(place.0) + y.max(place.1) - y.min(place.1);

        if distance < min_distance {
            min_distance = distance;
            ret_val = Ok(i);
        } else if distance == min_distance {
            ret_val = Err(());
        }
    }
    ret_val
}

fn is_in_region(places: &[(u32, u32)], x: u32, y: u32) -> bool {
    let mut distance_sum = 0;
    for place in places {
        distance_sum += x.max(place.0) - x.min(place.0) + y.max(place.1) - y.min(place.1);
    }
    distance_sum < THRESHOLD
}
