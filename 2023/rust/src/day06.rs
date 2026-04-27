use anyhow::Result;
use aoc_rust_common::Solution;

pub struct Day06;

impl Solution for Day06 {
    fn year(&self) -> u32 {
        2023
    }
    fn day(&self) -> u32 {
        6
    }

    fn part1(&self, input: &str) -> Result<String> {
        let lines: Vec<&str> = input.lines().collect();
        let times: Vec<i64> = lines[0]
            .split_whitespace()
            .skip(1)
            .map(|s| s.parse().unwrap())
            .collect();
        let distances: Vec<i64> = lines[1]
            .split_whitespace()
            .skip(1)
            .map(|s| s.parse().unwrap())
            .collect();

        let mut product = 1;
        for (i, &time) in times.iter().enumerate() {
            let record = distances[i];
            let mut count = 0;
            for hold in 0..time {
                let dist = hold * (time - hold);
                if dist > record {
                    count += 1;
                }
            }
            product *= count;
        }

        Ok(product.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let lines: Vec<&str> = input.lines().collect();
        let time: i64 = lines[0]
            .split(':')
            .nth(1)
            .unwrap()
            .replace(' ', "")
            .parse()
            .unwrap();
        let record: i64 = lines[1]
            .split(':')
            .nth(1)
            .unwrap()
            .replace(' ', "")
            .parse()
            .unwrap();

        let mut count = 0;
        for hold in 0..time {
            let dist = hold * (time - hold);
            if dist > record {
                count += 1;
            }
        }

        Ok(count.to_string())
    }
}

aoc_rust_common::aoc_test!(Day06, "4403592", "38017587");
