use anyhow::Result;
use aoc_rust_common::Solution;
use regex::Regex;
use std::collections::HashMap;

pub struct Day03;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn within(&self, start: Coordinate, end: Coordinate) -> bool {
        self.y >= start.y && self.y <= end.y && self.x >= start.x && self.x <= end.x
    }
}

#[derive(Debug)]
struct PartNumber {
    value: i32,
    start: Coordinate,
    end: Coordinate,
}

#[derive(Debug, Clone)]
struct Symbol {
    sym: String,
    pos: Coordinate,
}

impl Symbol {
    fn neighbor_of(&self, number: &PartNumber) -> bool {
        self.pos.within(
            Coordinate {
                x: number.start.x - 1,
                y: number.start.y - 1,
            },
            Coordinate {
                x: number.end.x + 1,
                y: number.end.y + 1,
            },
        )
    }
}

enum Item {
    PartNumber(PartNumber),
    Symbol(Symbol),
}

fn find_parts_and_symbols(input: &str) -> Vec<Item> {
    let re = Regex::new(r"(\d+)|[^.\n]").unwrap();
    let mut items = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for mat in re.find_iter(line) {
            let s = mat.as_str();
            let x_start = mat.start() as i32;
            let x_end = mat.end() as i32;
            if s.chars().next().unwrap().is_ascii_digit() {
                items.push(Item::PartNumber(PartNumber {
                    value: s.parse().unwrap(),
                    start: Coordinate {
                        x: x_start,
                        y: y as i32,
                    },
                    end: Coordinate {
                        x: x_end - 1,
                        y: y as i32,
                    },
                }));
            } else {
                items.push(Item::Symbol(Symbol {
                    sym: s.to_string(),
                    pos: Coordinate {
                        x: x_start,
                        y: y as i32,
                    },
                }));
            }
        }
    }
    items
}

impl Solution for Day03 {
    fn year(&self) -> u32 {
        2023
    }
    fn day(&self) -> u32 {
        3
    }

    fn part1(&self, input: &str) -> Result<String> {
        let all = find_parts_and_symbols(input);
        let symbols: Vec<Symbol> = all
            .iter()
            .filter_map(|item| {
                if let Item::Symbol(s) = item {
                    Some(s.clone())
                } else {
                    None
                }
            })
            .collect();

        let mut sum = 0;
        for item in &all {
            if let Item::PartNumber(n) = item {
                if symbols.iter().any(|s| s.neighbor_of(n)) {
                    sum += n.value;
                }
            }
        }
        Ok(sum.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let all = find_parts_and_symbols(input);
        let symbols: Vec<Symbol> = all
            .iter()
            .filter_map(|item| {
                if let Item::Symbol(s) = item {
                    Some(s.clone())
                } else {
                    None
                }
            })
            .collect();

        let mut gear_parts: HashMap<Coordinate, Vec<i32>> = HashMap::new();

        for item in &all {
            if let Item::PartNumber(n) = item {
                for s in &symbols {
                    if s.sym == "*" && s.neighbor_of(n) {
                        gear_parts.entry(s.pos).or_default().push(n.value);
                    }
                }
            }
        }

        let result: i32 = gear_parts
            .values()
            .filter(|parts| parts.len() == 2)
            .map(|parts| parts[0] * parts[1])
            .sum();

        Ok(result.to_string())
    }
}

aoc_rust_common::aoc_test!(Day03, "540212", "87605697");
