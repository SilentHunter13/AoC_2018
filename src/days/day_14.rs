const PUZZLE_INPUT: &str = "760221";
// const PUZZLE_INPUT: &str = "51589";
// const PUZZLE_INPUT: &str = "59414";

pub fn star_1() -> String {
    let mut board: Vec<usize> = vec![3, 7];
    let mut elf_1 = 0;
    let mut elf_2 = 1;
    let max_iterations = PUZZLE_INPUT.parse::<usize>().expect("no number");

    let recipes: Vec<usize> = loop {
        let sum = board[elf_1] + board[elf_2];

        let new_recipes = desintegrate(sum);
        board.extend(new_recipes);

        if board.len() >= max_iterations + 10 {
            board.truncate(max_iterations + 10);
            break board;
        }

        elf_1 = (elf_1 + board[elf_1] + 1) % board.len();
        elf_2 = (elf_2 + board[elf_2] + 1) % board.len();
    };

    recipes[max_iterations..]
        .iter()
        .map(|x| x.to_string())
        .collect::<String>()
}

pub fn star_2() -> usize {
    let mut board: Vec<usize> = vec![3, 7];
    let mut elf_1 = 0;
    let mut elf_2 = 1;

    let sequence: Vec<usize> = PUZZLE_INPUT
        .chars()
        .map(|x| x.to_digit(10).expect("NaN") as usize)
        .collect();

    let mut next_index = 0;
    let mut steps_taken = 2;

    loop {
        let sum = board[elf_1] + board[elf_2];

        let new_recipes = desintegrate(sum);

        // Dieser algorithms funktioniert nicht für alle Eingaben
        for recipe in new_recipes.iter() {
            if let Some(sequence_value) = sequence.get(next_index) {
                if *recipe == *sequence_value {
                    next_index += 1;
                } else {
                    next_index = 0; // Hier müsste auch in new_recipes zurückgesprungen werden
                }
                steps_taken += 1;
            }
        }

        if next_index >= sequence.len() {
            break steps_taken - next_index;
        }

        board.extend(new_recipes);

        elf_1 = (elf_1 + board[elf_1] + 1) % board.len();
        elf_2 = (elf_2 + board[elf_2] + 1) % board.len();
    }
}

fn desintegrate(value: usize) -> Vec<usize> {
    let value_1 = value / 10;
    let value_2 = value % 10;

    if value_1 == 1 {
        vec![value_1, value_2]
    } else {
        vec![value_2]
    }
}
