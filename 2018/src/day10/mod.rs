use aoc_rust_common::Solution;
use lazy_static::lazy_static;
use regex::Regex;
use std::fmt::Display;

pub struct Day10;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl Node {
    fn tick(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
    }
}

impl TryFrom<&str> for Node {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>")
                    .unwrap();
        }
        let caps = RE.captures(value).unwrap();
        Ok(Node {
            x: caps[1].parse().unwrap(),
            y: caps[2].parse().unwrap(),
            vx: caps[3].parse().unwrap(),
            vy: caps[4].parse().unwrap(),
        })
    }
}

fn find_message(nodes: &mut Vec<Node>) -> (String, i32) {
    for counter in 1.. {
        nodes.iter_mut().for_each(|node| node.tick());
        let min_y = nodes.iter().map(|n| n.y).min().unwrap();
        let max_y = nodes.iter().map(|n| n.y).max().unwrap();

        if max_y - min_y <= 10 {
            let min_x = nodes.iter().map(|n| n.x).min().unwrap();
            let max_x = nodes.iter().map(|n| n.x).max().unwrap();

            let mut grid =
                vec![vec!['.'; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];
            for node in nodes {
                grid[(node.y - min_y) as usize][(node.x - min_x) as usize] = '#';
            }
            let message = grid
                .iter()
                .map(|row| row.iter().collect::<String>())
                .collect::<Vec<_>>()
                .join(
                    "
",
                );
            return (message, counter);
        }
    }
    ("".to_string(), 0)
}

impl Solution for Day10 {
    fn year(&self) -> u32 {
        2018
    }
    fn day(&self) -> u32 {
        10
    }

    fn part1(&self, input: &str) -> Box<dyn Display> {
        let mut nodes: Vec<Node> = input.lines().map(|s| s.try_into().unwrap()).collect();
        let (message, _) = find_message(&mut nodes);
        Box::new(format!(
            "
{}",
            message
        ))
    }

    fn part2(&self, input: &str) -> Box<dyn Display> {
        let mut nodes: Vec<Node> = input.lines().map(|s| s.try_into().unwrap()).collect();
        let (_, seconds) = find_message(&mut nodes);
        Box::new(seconds)
    }
}
