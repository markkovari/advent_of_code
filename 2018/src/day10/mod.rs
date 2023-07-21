use lazy_static::lazy_static;
use regex::{Error, Regex};

use std::io::BufRead;

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
            static ref RE: Regex = Regex::new(
                r"position=<\s*([-]?\d+),\s*([-]?\d+)> velocity=<\s*([-]?\d+),\s*([-]?\d+)>"
            )
            .unwrap();
        }

        let caps = RE.captures(value).unwrap();
        let x = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let y = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let vx = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
        let vy = caps.get(4).unwrap().as_str().parse::<i32>().unwrap();
        Ok(Node { x, y, vx, vy })
    }
}

fn solution(nodes: &mut Vec<Node>) {
    for counter in 1.. {
        nodes.iter_mut().for_each(|node| node.tick());
        if nodes[..nodes.len() - 1]
            .iter()
            .zip(nodes[1..].iter())
            .all(|pair| (pair.0.y - pair.1.y).abs() <= 10)
        {
            println!("result of q02 is {}", counter);
            break;
        }
    }
}

fn visualize(nodes: &Vec<Node>) {
    let min_x = nodes.iter().map(|n| n.x).min().unwrap();
    let min_y = nodes.iter().map(|n| n.y).min().unwrap();

    let mut visualized: [[char; 100]; 10] = [['.'; 100]; 10];
    nodes
        .iter()
        .for_each(|n| visualized[(n.y - min_y) as usize][(n.x - min_x) as usize] = '#');
    visualized.iter().for_each(|line| {
        line.iter().for_each(|c| print!("{}", c));
        println!();
    });
}

fn part1(content: String) -> Result<(), Error> {
    let _path = format!("./input/{}", "day10.txt");

    let mut nodes: Vec<Node> = content
        .lines()
        .map(|s| s.try_into().unwrap())
        .collect::<Vec<_>>();

    solution(&mut nodes);
    visualize(&nodes);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_first() {
        let example = include_str!("./example.data");
        let _ = part1(example.to_owned());
        let prod = include_str!("./prod.data");
        let _ = part1(prod.to_owned());
    }
}
