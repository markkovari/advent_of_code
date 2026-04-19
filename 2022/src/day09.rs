use anyhow::Result;
use aoc_rust_common::Solution;
use std::collections::HashSet;

pub struct Day09;

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn move_in(&mut self, direction: &str) {
        match direction {
            "U" => self.y += 1,
            "D" => self.y -= 1,
            "L" => self.x -= 1,
            "R" => self.x += 1,
            _ => unreachable!(),
        }
    }

    fn follow(&mut self, head: Pos) {
        let dx = head.x - self.x;
        let dy = head.y - self.y;
        if dx.abs() > 1 || dy.abs() > 1 {
            self.x += dx.signum();
            self.y += dy.signum();
        }
    }
}

fn simulate(input: &str, rope_len: usize) -> usize {
    let mut rope = vec![Pos::default(); rope_len];
    let mut visited = HashSet::new();
    visited.insert(*rope.last().unwrap());

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let direction = parts.next().unwrap();
        let steps: usize = parts.next().unwrap().parse().unwrap();
        for _ in 0..steps {
            rope[0].move_in(direction);
            for i in 1..rope_len {
                let head = rope[i - 1];
                rope[i].follow(head);
            }
            visited.insert(*rope.last().unwrap());
        }
    }
    visited.len()
}

impl Solution for Day09 {
    fn year(&self) -> u32 {
        2022
    }
    fn day(&self) -> u32 {
        9
    }

    fn part1(&self, input: &str) -> Result<String> {
        Ok((simulate(input, 2)).to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        Ok((simulate(input, 10)).to_string())
    }
}

aoc_rust_common::aoc_test!(Day09);
