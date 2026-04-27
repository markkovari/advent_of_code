use anyhow::Result;
use aoc_rust_common::Solution;
use std::collections::HashMap;

pub struct Day02;

struct Game {
    id: u32,
    hands: Vec<Vec<Hand>>,
}

struct Hand {
    color: String,
    count: u32,
}

fn parse(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            let id = parts[0].split(' ').last().unwrap().parse().unwrap();
            let hands = parts[1]
                .split("; ")
                .map(|hand| {
                    hand.split(", ")
                        .map(|item| {
                            let item_parts: Vec<&str> = item.split(' ').collect();
                            Hand {
                                count: item_parts[0].parse().unwrap(),
                                color: item_parts[1].to_string(),
                            }
                        })
                        .collect()
                })
                .collect();
            Game { id, hands }
        })
        .collect()
}

impl Solution for Day02 {
    fn year(&self) -> u32 {
        2023
    }
    fn day(&self) -> u32 {
        2
    }

    fn part1(&self, input: &str) -> Result<String> {
        let games = parse(input);
        let limits = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

        let result: u32 = games
            .iter()
            .filter(|game| {
                game.hands.iter().all(|hand| {
                    hand.iter().all(|item| {
                        item.count <= *limits.get(item.color.as_str()).unwrap_or(&0)
                    })
                })
            })
            .map(|game| game.id)
            .sum();

        Ok(result.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let games = parse(input);

        let result: u32 = games
            .iter()
            .map(|game| {
                let mut max_counts = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
                for hand in &game.hands {
                    for item in hand {
                        let entry = max_counts.entry(item.color.as_str()).or_insert(0);
                        if item.count > *entry {
                            *entry = item.count;
                        }
                    }
                }
                max_counts.values().product::<u32>()
            })
            .sum();

        Ok(result.to_string())
    }
}

aoc_rust_common::aoc_test!(Day02, "2617", "59795");
