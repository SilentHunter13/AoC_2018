const GRID_SERIAL_NUMBER: i32 = 8199;

const GRID_SIZE: i32 = 300;

pub fn star_1() -> (i32, i32) {
    let mut max_power = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    for x in 0..=297 {
        for y in 0..=297 {
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

    (max_x + 1, max_y + 1) //x und y sind in Aufgabe 1 basiert
}

pub fn star_2() -> (i32, i32, i32) {
    let grid = get_grid();
    let mut max_power = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_size = 0;
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            let max_possible_size = GRID_SIZE - x.max(y);
            let mut power = 0;
            for size in 1..=max_possible_size {
                power += get_total_power_inc(&grid, x, y, size);

                if power > max_power {
                    max_power = power;
                    max_x = x;
                    max_y = y;
                    max_size = size;
                }
            }
        }
    }

    (max_x + 1, max_y + 1, max_size) //x und y sind in Aufgabe 1 basiert
}

//Es könnte sein, dass das Vorberechnen keinen Geschwindigkeitsvorteil mehr bringt?
fn get_grid() -> [[i32; 300]; 300] {
    let mut grid: [[i32; 300]; 300] = [[0; 300]; 300];

    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            grid[x as usize][y as usize] = get_power_level(x, y);
        }
    }
    grid
}

fn get_power_level(x: i32, y: i32) -> i32 {
    let rack_id = (x + 1) + 10;
    ((((rack_id * (y + 1)) + GRID_SERIAL_NUMBER) * (rack_id)) / 100) % 10 - 5
}

fn get_total_power_inc(grid: &[[i32; 300]; 300], x: i32, y: i32, size: i32) -> i32 {
    let mut total_power_inc = 0;

    for i in x..(x + size) {
        total_power_inc += grid[i as usize][(y + size - 1) as usize];
    }

    for k in y..(y - 1 + size) {
        total_power_inc += grid[(x + size - 1) as usize][k as usize];
    }

    total_power_inc
}
