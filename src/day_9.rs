const MARBLE_MAX: usize = 71052;
const PLAYER_COUNT: usize = 419;
//const MARBLE_MAX: usize = 25;
//const PLAYER_COUNT: usize = 9;

pub fn star_1() -> usize {
    play_marble_game(PLAYER_COUNT, MARBLE_MAX)
}

pub fn star_2() -> usize {
    play_marble_game(PLAYER_COUNT, MARBLE_MAX * 100)
}

enum Direction {
    Forward,
    Reverse,
}

#[derive(Debug)]
struct Link {
    counter_clockwise: usize,
    clockwise: usize,
}

fn play_marble_game(players: usize, marbles: usize) -> usize {
    let mut board = Vec::with_capacity(marbles);
    let mut players: Vec<usize> = vec![0; players];
    let mut current_marble = 0;

    board.push(Link {
        counter_clockwise: 0,
        clockwise: 0,
    });

    for i in 1..=marbles {
        //aktuelle Position berechnen
        if (i % 23) == 0 {
            players[i % PLAYER_COUNT] += i;

            current_marble = new_position_get(&board, current_marble, 7, Direction::Reverse);
            //Murmel entfernen
            players[i % PLAYER_COUNT] += current_marble;
            current_marble = remove_marble(&mut board, current_marble);
        } else {
            current_marble = new_position_get(&board, current_marble, 1, Direction::Forward);

            //Murmel einfügen
            current_marble = insert_marble(&mut board, current_marble);
        }
    }

    *players.iter().max().expect("No Maximum")
}

fn new_position_get(board: &[Link], current: usize, steps: usize, direction: Direction) -> usize {
    let mut new_position = current;

    for _ in 0..steps {
        if let Some(marble) = board.get(new_position) {
            new_position = match direction {
                Direction::Forward => marble.clockwise,
                Direction::Reverse => marble.counter_clockwise,
            }
        }
    }
    new_position
}

fn remove_marble(board: &mut Vec<Link>, index: usize) -> usize {
    let clockwise;
    let counter_clockwise;
    if let Some(current) = board.get(index) {
        clockwise = current.clockwise;
        counter_clockwise = current.counter_clockwise;
    } else {
        panic!("Index nicht vorhanden")
    }
    if let Some(before) = board.get_mut(counter_clockwise) {
        before.clockwise = clockwise;
    }

    if let Some(after) = board.get_mut(clockwise) {
        after.counter_clockwise = counter_clockwise;
    }

    //leere Murmel einfügen da der Index im Array die Nummer der Murmel ist
    board.push(Link {
        counter_clockwise: 0,
        clockwise: 0,
    });
    clockwise
}

//Fügt eine Murmel nach der Murmel mit dem Index ein.
fn insert_marble(board: &mut Vec<Link>, index: usize) -> usize {
    let new_index = board.len();

    let mut new_marble = Link {
        clockwise: 0,
        counter_clockwise: index,
    };
    if let Some(before) = board.get_mut(index) {
        new_marble.clockwise = before.clockwise;
        before.clockwise = new_index;
    } else {
        panic!("Index nicht vorhanden")
    }

    if let Some(after) = board.get_mut(new_marble.clockwise) {
        after.counter_clockwise = new_index;
    } else {
        panic!("Index nicht vorhanden")
    }

    board.push(new_marble);
    new_index
}
