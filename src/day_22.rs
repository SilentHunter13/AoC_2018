use std::collections::HashMap;

//Puzzle Input
const DEPTH: u32 = 11739;
const TARGET_X: u32 = 11;
const TARGET_Y: u32 = 718;

//Game Constants
const X_FACTOR: u32 = 16807;
const Y_FACTOR: u32 = 48271;
const DIVIDENT: u32 = 20183;

pub fn star_1() -> u32 {
    let mut erosion_levels: HashMap<(u32, u32), u32> = HashMap::new();
    let mut risk_level = 0;
    for x in 0..=TARGET_X {
        for y in 0..=TARGET_Y {
            let geologic_index;
            if ((x == 0) && (y == 0)) || ((x == TARGET_X) && (y == TARGET_Y)) {
                geologic_index = 0;
            } else if x == 0 {
                geologic_index = y * Y_FACTOR;
            } else if y == 0 {
                geologic_index = x * X_FACTOR;
            } else {
                geologic_index = erosion_levels.get(&(x - 1, y)).unwrap()
                    * erosion_levels.get(&(x, y - 1)).unwrap();
            }

            let erosion_level = (geologic_index + DEPTH) % DIVIDENT;
            erosion_levels.insert((x, y), erosion_level);

            risk_level += erosion_level % 3;
        }
    }
    risk_level
}
