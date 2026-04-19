use anyhow::Result;
use aoc_rust_common::Solution;

pub struct Day11;

const GRID_SIZE: i32 = 300;

fn get_power(x: i32, y: i32, serial_number: i32) -> i32 {
    let rack_id = x + 10;
    let mut power = rack_id * y;
    power += serial_number;
    power *= rack_id;
    (power / 100) % 10 - 5
}

fn build_summed_area_table(serial_number: i32) -> Vec<Vec<i32>> {
    let mut sat = vec![vec![0; (GRID_SIZE + 1) as usize]; (GRID_SIZE + 1) as usize];
    for y in 1..=GRID_SIZE {
        for x in 1..=GRID_SIZE {
            let power = get_power(x, y, serial_number);
            sat[y as usize][x as usize] =
                power + sat[y as usize - 1][x as usize] + sat[y as usize][x as usize - 1]
                    - sat[y as usize - 1][x as usize - 1];
        }
    }
    sat
}

fn get_square_power(sat: &[Vec<i32>], x: i32, y: i32, size: i32) -> i32 {
    let (x1, y1, x2, y2) = (x - 1, y - 1, x + size - 1, y + size - 1);
    sat[y2 as usize][x2 as usize] - sat[y1 as usize][x2 as usize] - sat[y2 as usize][x1 as usize]
        + sat[y1 as usize][x1 as usize]
}

impl Solution for Day11 {
    fn year(&self) -> u32 {
        2018
    }
    fn day(&self) -> u32 {
        11
    }

    fn part1(&self, input: &str) -> Result<String> {
        let serial_number: i32 = input.trim().parse().unwrap();
        let sat = build_summed_area_table(serial_number);
        let mut max_power = i32::MIN;
        let mut max_coord = (0, 0);
        for y in 1..=GRID_SIZE - 2 {
            for x in 1..=GRID_SIZE - 2 {
                let power = get_square_power(&sat, x, y, 3);
                if power > max_power {
                    max_power = power;
                    max_coord = (x, y);
                }
            }
        }
        Ok((format!("{},{}", max_coord.0, max_coord.1)).to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let serial_number: i32 = input.trim().parse().unwrap();
        let sat = build_summed_area_table(serial_number);
        let mut max_power = i32::MIN;
        let mut max_details = (0, 0, 0);
        for size in 1..=GRID_SIZE {
            for y in 1..=GRID_SIZE - size + 1 {
                for x in 1..=GRID_SIZE - size + 1 {
                    let power = get_square_power(&sat, x, y, size);
                    if power > max_power {
                        max_power = power;
                        max_details = (x, y, size);
                    }
                }
            }
        }
        Ok(format!(
            "{},{},{}",
            max_details.0, max_details.1, max_details.2
        ))
    }
}

aoc_rust_common::aoc_test!(Day11);
