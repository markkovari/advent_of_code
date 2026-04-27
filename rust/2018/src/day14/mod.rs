use anyhow::Result;
use aoc_common::Solution;

pub struct Day14;

impl Solution for Day14 {
    fn year(&self) -> u32 {
        2018
    }
    fn day(&self) -> u32 {
        14
    }

    fn part1(&self, input: &str) -> Result<String> {
        let recipe_count: usize = input.trim().parse().unwrap();
        let mut scores = vec![3, 7];
        let mut elves = vec![0, 1];

        while scores.len() < recipe_count + 10 {
            let new_recipe: u32 = elves.iter().map(|&e| scores[e]).sum();
            for &digit in new_recipe.to_string().as_bytes() {
                scores.push((digit - b'0') as u32);
            }
            for e in &mut elves {
                *e = (*e + scores[*e] as usize + 1) % scores.len();
            }
        }

        let result: String = scores[recipe_count..recipe_count + 10]
            .iter()
            .map(|s| s.to_string())
            .collect();
        Ok((result).to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let digits: Vec<u32> = input
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        let mut scores = vec![3, 7];
        let mut elves = vec![0, 1];

        loop {
            let new_recipe: u32 = elves.iter().map(|&e| scores[e]).sum();
            for &digit in new_recipe.to_string().as_bytes() {
                scores.push((digit - b'0') as u32);
                if scores.len() > digits.len() {
                    if scores[scores.len() - digits.len()..] == digits[..] {
                        return Ok((scores.len() - digits.len()).to_string());
                    }
                }
            }

            for e in &mut elves {
                *e = (*e + scores[*e] as usize + 1) % scores.len();
            }
        }
    }
}

aoc_common::aoc_test!(Day14);
