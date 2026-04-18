use anyhow::Result;
use aoc_rust_common::Solution;
use std::collections::HashMap;

type Position = (i32, i32);

struct Santa {
    position: Position,
    visited: HashMap<Position, usize>,
}

impl Santa {
    pub fn new() -> Self {
        let mut visited = HashMap::new();
        visited.insert((0, 0), 1);
        Self {
            position: (0, 0),
            visited,
        }
    }
    pub fn move_to(&mut self, direction: char) {
        match direction {
            '>' => {
                self.position.0 += 1;
            }
            '<' => {
                self.position.0 -= 1;
            }
            '^' => {
                self.position.1 -= 1;
            }
            'v' => {
                self.position.1 += 1;
            }
            _ => {}
        }
        let counter = self.visited.entry(self.position).or_insert(0);
        *counter += 1;
    }

    pub fn get_visited_multiple(&self) -> usize {
        let visiteds = self
            .visited
            .values()
            .filter(|&x| *x >= 1)
            .collect::<Vec<_>>();
        visiteds.len()
    }
}

pub struct Day03;

fn get_visited_multiple_times(first: Santa, second: Santa) -> usize {
    let mut visited = HashMap::new();
    for (key, value) in first.visited.iter() {
        visited.insert(*key, *value);
    }
    for (key, value) in second.visited.iter() {
        visited.insert(*key, *value);
    }
    visited.values().filter(|&x| *x >= 1).count()
}

impl Solution for Day03 {
    fn year(&self) -> u32 {
        2015
    }
    fn day(&self) -> u32 {
        3
    }

    fn part1(&self, input: &str) -> Result<String> {
        let mut santa = Santa::new();
        for direction in input.chars() {
            santa.move_to(direction);
        }
        Ok((santa.get_visited_multiple() as i64).to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let mut santa = Santa::new();
        let mut robo_santa = Santa::new();
        for direction in input.chars().enumerate() {
            if direction.0 % 2 == 0 {
                santa.move_to(direction.1);
            } else {
                robo_santa.move_to(direction.1);
            }
        }
        Ok((get_visited_multiple_times(santa, robo_santa) as i64).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day03() {
        let day = Day03;
        const PROD: &str = include_str!("inputs/3_prod.txt");

        assert_eq!(day.part1(">").unwrap(), "2");
        assert_eq!(day.part1("^>v<").unwrap(), "4");
        assert_eq!(day.part1("^v^v^v^v^v").unwrap(), "2");
        assert_eq!(day.part1(PROD).unwrap(), "2572");

        assert_eq!(day.part2("^v").unwrap(), "3");
        assert_eq!(day.part2("^>v<").unwrap(), "3");
        assert_eq!(day.part2("^v^v^v^v^v").unwrap(), "11");
        assert_eq!(day.part2(PROD).unwrap(), "2631");
    }
}
