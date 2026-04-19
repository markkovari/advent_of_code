use anyhow::Result;
use aoc_rust_common::Solution;

pub struct Day20;

fn divisors_sum(n: usize) -> usize {
    let mut sum = 0;
    let sqrt = (n as f64).sqrt() as usize;
    for i in 1..=sqrt {
        if n.is_multiple_of(i) {
            sum += i;
            if i * i != n {
                sum += n / i;
            }
        }
    }
    sum
}

fn divisors_sum_part2(n: usize) -> usize {
    let mut sum = 0;
    let sqrt = (n as f64).sqrt() as usize;
    for i in 1..=sqrt {
        if n.is_multiple_of(i) {
            if n / i <= 50 {
                sum += i;
            }
            if i * i != n && n / (n / i) <= 50 {
                sum += n / i;
            }
        }
    }
    sum
}

impl Solution for Day20 {
    fn year(&self) -> u32 {
        2015
    }
    fn day(&self) -> u32 {
        20
    }

    fn part1(&self, input: &str) -> Result<String> {
        let target: usize = input.trim().parse().unwrap();
        let target_presents = target / 10;
        for i in 1.. {
            if divisors_sum(i) >= target_presents {
                return Ok((i as i64).to_string());
            }
        }
        Ok((0).to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let target: usize = input.trim().parse().unwrap();
        for i in 1.. {
            if divisors_sum_part2(i) * 11 >= target {
                return Ok((i as i64).to_string());
            }
        }
        Ok((0).to_string())
    }
}

aoc_rust_common::aoc_test!(Day20);
