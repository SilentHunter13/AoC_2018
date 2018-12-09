const MARBLE_MAX: usize = 71052;
const PLAYER_COUNT: usize = 419;
//const MARBLE_MAX: usize = 25;
//const PLAYER_COUNT: usize = 9;

pub fn star_1() -> usize {
    let mut board: Vec<usize> = vec![0];
    let mut players: Vec<usize> = vec![0; PLAYER_COUNT];
    let mut current_marble = 0;

    for i in 1..=MARBLE_MAX {
        //aktuelle Position berechnen
        if (i % 23) == 0 {
            players[i % PLAYER_COUNT] += i;

            let remove_index = (current_marble + board.len() - 7) % board.len();
            players[i % PLAYER_COUNT] += board.remove(remove_index);

            current_marble = remove_index;
        } else {
            current_marble = (current_marble + 1) % (board.len()) + 1;

            //Murmel einfügen
            board.insert(current_marble, i);
        }
    }

    *players.iter().max().expect("No Maximum")
}
