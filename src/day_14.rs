const PUZZLE_INPUT: &str = "760221";

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

fn desintegrate(value: usize) -> Vec<usize> {
    let value_1 = value / 10;
    let value_2 = value % 10;

    if value_1 == 1 {
        vec![value_1, value_2]
    } else {
        vec![value_2]
    }
}
