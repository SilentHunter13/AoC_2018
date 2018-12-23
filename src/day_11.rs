const GRID_SERIAL_NUMBER: i32 = 8199;

pub fn star_1() -> (i32, i32) {
    let mut max_power = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    for x in 1..=298 {
        for y in 1..=298 {
            let power = get_power_level(x, y)
                + get_power_level(x, y + 1)
                + get_power_level(x, y + 2)
                + get_power_level(x + 1, y)
                + get_power_level(x + 1, y + 1)
                + get_power_level(x + 1, y + 2)
                + get_power_level(x + 2, y)
                + get_power_level(x + 2, y + 1)
                + get_power_level(x + 2, y + 2);

            if power > max_power {
                max_power = power;
                max_x = x;
                max_y = y;
            }
        }
    }

    (max_x, max_y)
}

fn get_power_level(x: i32, y: i32) -> i32 {
    let rack_id = x + 10;
    ((((rack_id * y) + GRID_SERIAL_NUMBER) * (rack_id)) / 100) % 10 - 5
}
