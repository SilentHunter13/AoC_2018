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

fn play_marble_game(players: usize, marbles: usize) -> usize {
    let mut board: Vec<usize> = vec![0];
    let mut players: Vec<usize> = vec![0; players];
    let mut current_marble = 0;

    for i in 1..=marbles {
        //aktuelle Position berechnen
        if (i % 23) == 0 {
            players[i % PLAYER_COUNT] += i;

            current_marble = (current_marble + board.len() - 7) % board.len();

            //Murmel entfernen
            players[i % PLAYER_COUNT] += board.remove(current_marble);
        } else {
            current_marble = (current_marble + 2) % board.len();

            //Murmel einfügen
            board.insert(current_marble, i);
        }
    }

    *players.iter().max().expect("No Maximum")
}
