use anyhow::Result;
use aoc_rust_common::Solution;
use std::collections::HashMap;

pub struct Day18;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Acre {
    Open,
    Trees,
    Lumberyard,
}

fn parse(input: &str) -> Vec<Vec<Acre>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Acre::Open,
                    '|' => Acre::Trees,
                    '#' => Acre::Lumberyard,
                    _ => panic!("Invalid acre type"),
                })
                .collect()
        })
        .collect()
}

fn step(area: &Vec<Vec<Acre>>) -> Vec<Vec<Acre>> {
    let mut new_area = area.clone();
    for y in 0..area.len() {
        for x in 0..area[y].len() {
            let mut tree_count = 0;
            let mut lumber_count = 0;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let ny = y as i32 + dy;
                    let nx = x as i32 + dx;
                    if ny >= 0 && ny < area.len() as i32 && nx >= 0 && nx < area[y].len() as i32 {
                        match area[ny as usize][nx as usize] {
                            Acre::Trees => tree_count += 1,
                            Acre::Lumberyard => lumber_count += 1,
                            _ => {}
                        }
                    }
                }
            }

            new_area[y][x] = match area[y][x] {
                Acre::Open if tree_count >= 3 => Acre::Trees,
                Acre::Trees if lumber_count >= 3 => Acre::Lumberyard,
                Acre::Lumberyard if lumber_count == 0 || tree_count == 0 => Acre::Open,
                current => current,
            };
        }
    }
    new_area
}

fn resource_value(area: &Vec<Vec<Acre>>) -> usize {
    let wooded = area.iter().flatten().filter(|&&a| a == Acre::Trees).count();
    let lumberyards = area
        .iter()
        .flatten()
        .filter(|&&a| a == Acre::Lumberyard)
        .count();
    wooded * lumberyards
}

impl Solution for Day18 {
    fn year(&self) -> u32 {
        2018
    }
    fn day(&self) -> u32 {
        18
    }

    fn part1(&self, input: &str) -> Result<String> {
        let mut area = parse(input);
        for _ in 0..10 {
            area = step(&area);
        }
        Ok((resource_value(&area)).to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let mut area = parse(input);
        let mut history = HashMap::new();
        let target_min = 1_000_000_000;

        for min in 1..=target_min {
            area = step(&area);
            if let Some(prev_min) = history.insert(area.clone(), min) {
                let cycle_len = min - prev_min;
                let remaining_mins = target_min - min;
                if remaining_mins % cycle_len == 0 {
                    return Ok((resource_value(&area)).to_string());
                }
            }
        }
        Ok((resource_value(&area)).to_string())
    }
}
