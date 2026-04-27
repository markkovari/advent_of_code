use anyhow::Result;
use aoc_rust_common::Point;
use aoc_rust_common::Solution;
use std::collections::{HashMap, HashSet};

pub struct Day03;

impl Solution for Day03 {
    fn year(&self) -> u32 {
        2019
    }
    fn day(&self) -> u32 {
        3
    }

    fn part1(&self, input: &str) -> Result<String> {
        let wires: Vec<Vec<&str>> = input
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.split(',').collect())
            .collect();
        let path1 = get_path(&wires[0]);
        let path2 = get_path(&wires[1]);

        let set1: HashSet<Point> = path1.keys().cloned().collect();
        let set2: HashSet<Point> = path2.keys().cloned().collect();

        let min_dist = set1
            .intersection(&set2)
            .map(|p| p.x.abs() + p.y.abs())
            .min()
            .unwrap();

        Ok(min_dist.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let wires: Vec<Vec<&str>> = input
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.split(',').collect())
            .collect();
        let path1 = get_path(&wires[0]);
        let path2 = get_path(&wires[1]);

        let set1: HashSet<Point> = path1.keys().cloned().collect();
        let set2: HashSet<Point> = path2.keys().cloned().collect();

        let min_steps = set1
            .intersection(&set2)
            .map(|p| path1[p] + path2[p])
            .min()
            .unwrap();

        Ok(min_steps.to_string())
    }
}

fn get_path(wire: &[&str]) -> HashMap<Point, i32> {
    let mut path = HashMap::new();
    let mut pos = Point::new(0, 0);
    let mut steps = 0;

    for move_str in wire {
        let dir = &move_str[0..1];
        let dist: i32 = move_str[1..].parse().unwrap();
        for _ in 0..dist {
            match dir {
                "R" => pos.x += 1,
                "L" => pos.x -= 1,
                "U" => pos.y -= 1, // standard grid: y-increases downwards
                "D" => pos.y += 1,
                _ => panic!("Unknown direction"),
            }
            steps += 1;
            path.entry(pos).or_insert(steps);
        }
    }
    path
}

aoc_rust_common::aoc_test!(Day03);
