use std::io::Result;

const SERIAL_NUMBER: i32 = 6392;
const GRID_SIZE: i32 = 300;

struct Grid {
    power: Vec<Vec<i32>>,
}

impl Grid {
    fn new(size: i32) -> Grid {
        Grid {
            power: vec![vec![0; size as usize]; size as usize],
        }
    }

    fn set(&mut self, x: i32, y: i32, power: i32) {
        self.power[x as usize - 1][y as usize - 1] = power;
    }

    fn get(&self, x: i32, y: i32) -> Option<i32> {
        let (x, y) = (x - 1, y - 1);
        if (0..GRID_SIZE).contains(&x) && (0..GRID_SIZE).contains(&y) {
            Some(self.power[x as usize][y as usize])
        } else {
            None
        }
    }

    fn square_power(&self, size: i32, top_left_x: i32, top_left_y: i32) -> i32 {
        let mut power = 0;
        for x in top_left_x..top_left_x + size {
            for y in top_left_y..top_left_y + size {
                power += self.get(x, y).unwrap_or(0);
            }
        }
        power
    }
}

fn fuel_cell_power(x: i32, y: i32) -> i32 {
    let rack_id = x + 10;
    let mut power = rack_id * y;
    power += SERIAL_NUMBER;
    power *= rack_id;
    power = (power / 100) % 10;
    power -= 5;
    power
}

fn part1(grid: &Grid) -> Result<(i32, i32)> {
    let (mut max_x, mut max_y, mut max_power) = (1, 1, grid.square_power(3, 1, 1));
    for x in 1..=GRID_SIZE {
        for y in 1..=GRID_SIZE {
            let power = grid.square_power(3, x, y);
            if power > max_power {
                max_x = x;
                max_y = y;
                max_power = power;
            }
        }
    }

    Ok((max_x, max_y))
}

fn part2(grid: &Grid) -> Result<(i32, i32, i32)> {
    let (mut max_size, mut max_x, mut max_y, mut max_power) = (1, 1, 1, grid.square_power(1, 1, 1));

    for size in 1..=GRID_SIZE {
        for x in 1..=GRID_SIZE {
            for y in 1..=GRID_SIZE {
                println!("size: {}, x: {}, y: {}", size, x, y);
                let power = grid.square_power(size, x, y);
                if power > max_power {
                    max_size = size;
                    max_x = x;
                    max_y = y;
                    max_power = power;
                }
                if y + size > GRID_SIZE {
                    break;
                }
            }
            if x + size > GRID_SIZE {
                break;
            }
        }
    }
    Ok((max_x, max_y, max_size))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_first() {
        let mut grid = Grid::new(GRID_SIZE);
        for x in 1..=GRID_SIZE {
            for y in 1..=GRID_SIZE {
                grid.set(x, y, fuel_cell_power(x, y));
            }
        }

        let (x, y) = part1(&grid).unwrap();
        println!("most powerful 3x3 square: {},{}", x, y);
        let (x, y, size) = part2(&grid).unwrap();
        println!("most powerful 3x3 square with size: {},{},{}", x, y, size);
    }
}
