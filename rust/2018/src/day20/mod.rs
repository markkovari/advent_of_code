use anyhow::Result;
use aoc_common::Solution;
use std::collections::HashMap;

pub struct Day20;

impl Solution for Day20 {
    fn year(&self) -> u32 {
        2018
    }
    fn day(&self) -> u32 {
        20
    }

    fn part1(&self, input: &str) -> Result<String> {
        let distances = get_distances(input);
        Ok((*distances.values().max().unwrap_or(&0)).to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let distances = get_distances(input);
        Ok((distances.values().filter(|&&d| d >= 1000).count()).to_string())
    }
}

fn get_distances(input: &str) -> HashMap<(i32, i32), usize> {
    let mut distances = HashMap::new();
    let mut stack = Vec::new();
    let mut current_pos = (0, 0);
    distances.insert(current_pos, 0);

    for char in input.chars() {
        match char {
            '(' => stack.push(current_pos),
            '|' => current_pos = *stack.last().unwrap(),
            ')' => current_pos = stack.pop().unwrap(),
            'N' | 'S' | 'E' | 'W' => {
                let (dx, dy) = match char {
                    'N' => (0, -1),
                    'S' => (0, 1),
                    'E' => (1, 0),
                    'W' => (-1, 0),
                    _ => unreachable!(),
                };
                let dist = distances[&current_pos] + 1;
                current_pos = (current_pos.0 + dx, current_pos.1 + dy);
                distances.entry(current_pos).or_insert(dist);
            }
            _ => {}
        }
    }
    distances
}

aoc_common::aoc_test!(Day20);
