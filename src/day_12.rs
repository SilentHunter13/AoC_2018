// const INITIAL_STATE: &str = "#..#.#..##......###...###";
//
// const NOTES: &str = "...## => #
// ..#.. => #
// .#... => #
// .#.#. => #
// .#.## => #
// .##.. => #
// .#### => #
// #.#.# => #
// #.### => #
// ##.#. => #
// ##.## => #
// ###.. => #
// ###.# => #
// ####. => #";

const INITIAL_STATE: &str = "#.####...##..#....#####.##.......##.#..###.#####.###.##.###.###.#...#...##.#.##.#...#..#.##..##.#.##";

const NOTES: &str = ".##.. => .
..##. => #
.#..# => #
.#.#. => .
..#.. => #
###.. => #
##..# => .
##... => #
#.### => #
.##.# => #
#.... => .
###.# => .
..... => .
.#... => #
....# => .
#.#.. => .
...#. => #
#...# => .
##.#. => .
.#.## => #
..#.# => #
#.#.# => .
.#### => .
##### => .
..### => .
...## => .
#..## => .
#.##. => .
#..#. => #
.###. => #
##.## => #
####. => .";

const ARRAY_SIZE: usize = 200;
const NOTE_SIZE: usize = 5;
const START_OFFSET: usize = 30;

pub fn star_1() -> i32 {
    let mut state: [bool; ARRAY_SIZE] = [false; ARRAY_SIZE];
    let mut notes: Vec<[bool; NOTE_SIZE]> = Vec::new();

    //Initialzustand auslesen
    for (n, i) in INITIAL_STATE.chars().enumerate() {
        if i == '#' {
            state[n + START_OFFSET] = true;
        }
    }

    //Notes auslesen
    for line in NOTES.lines() {
        let line: Vec<&str> = line.split(' ').collect();
        //nur die Notes speichern die zu einer Pflanze führen
        if line.get(2).expect("kein drittes Element").contains('#') {
            let mut note = [false; NOTE_SIZE];
            for (n, char) in line[0].chars().enumerate() {
                if char == '#' {
                    note[n] = true;
                }
            }
            notes.push(note);
        }
    }

    //20 Schritte simulieren
    for _ in 0..20 {
        let mut new_state = [false; ARRAY_SIZE];
        for (n, _) in state.iter().enumerate() {
            let start: i32 = n as i32 - 2;
            let end = n + 2;
            if start >= 0 && end < ARRAY_SIZE {
                let mut neighborhood = [false; NOTE_SIZE];
                neighborhood.clone_from_slice(&state[(n - 2)..=(n + 2)]);
                if notes.contains(&neighborhood) {
                    new_state[n] = true;
                }
            }
        }
        state = new_state;
    }

    let get_index = |(n, x): (usize, &bool)| -> i32 {
        if *x {
            (n as i32) - (START_OFFSET as i32)
        } else {
            0
        }
    };

    //Töpfe mit Pflanzen aufsummieren
    state.iter().enumerate().map(get_index).sum()
}
