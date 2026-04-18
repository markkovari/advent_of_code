use anyhow::Result;
use aoc_rust_common::Solution;
use regex::Regex;
use std::collections::VecDeque;

pub struct Day09;

fn play_game(num_players: usize, last_marble: u32) -> u32 {
    let mut circle = VecDeque::with_capacity(last_marble as usize + 1);
    circle.push_back(0);
    let mut scores = vec![0; num_players];

    for marble in 1..=last_marble {
        if marble % 23 == 0 {
            let player_index = (marble - 1) as usize % num_players;
            circle.rotate_right(7);
            scores[player_index] += marble + circle.pop_back().unwrap();
            circle.rotate_left(1);
        } else {
            circle.rotate_left(1);
            circle.push_back(marble);
        }
    }
    *scores.iter().max().unwrap()
}

impl Solution for Day09 {
    fn year(&self) -> u32 {
        2018
    }
    fn day(&self) -> u32 {
        9
    }

    fn part1(&self, input: &str) -> Result<String> {
        let re =
            Regex::new(r"(\d+)\s*players;\s*last\s*marble\s*is\s*worth\s*(\d+)\s*points?").unwrap();
        let caps = re.captures(input).unwrap();
        let players: usize = caps[1].parse().unwrap();
        let marbles: u32 = caps[2].parse().unwrap();
        Ok((play_game(players, marbles)).to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let re =
            Regex::new(r"(\d+)\s*players;\s*last\s*marble\s*is\s*worth\s*(\d+)\s*points?").unwrap();
        let caps = re.captures(input).unwrap();
        let players: usize = caps[1].parse().unwrap();
        let marbles: u32 = caps[2].parse().unwrap();
        Ok((play_game(players, marbles * 100)).to_string())
    }
}
