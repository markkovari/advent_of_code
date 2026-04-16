use aoc_rust_common::Solution;
use std::fmt::Display;
use regex::Regex;

pub struct Day09;

fn play_game(num_players: usize, last_marble: u32) -> u32 {
    let mut circle = Vec::with_capacity(last_marble as usize);
    circle.push(0);
    let mut scores = vec![0; num_players];
    let mut current_pos = 0;

    for marble in 1..=last_marble {
        if marble % 23 == 0 {
            let player = (marble - 1) as usize % num_players;
            let remove_pos = (current_pos + circle.len() - 7) % circle.len();
            scores[player] += marble + circle.remove(remove_pos);
            current_pos = remove_pos;
        } else {
            let insert_pos = (current_pos + 2) % circle.len();
            circle.insert(insert_pos, marble);
            current_pos = insert_pos;
        }
    }
    *scores.iter().max().unwrap()
}

impl Solution for Day09 {
    fn year(&self) -> u32 { 2018 }
    fn day(&self) -> u32 { 9 }

    fn part1(&self, input: &str) -> Box<dyn Display> {
        let re = Regex::new(r"(\d+)\s*players;\s*last\s*marble\s*is\s*worth\s*(\d+)\s*points?").unwrap();
        let caps = re.captures(input).unwrap();
        let players: usize = caps[1].parse().unwrap();
        let marbles: u32 = caps[2].parse().unwrap();
        Box::new(play_game(players, marbles))
    }

    fn part2(&self, input: &str) -> Box<dyn Display> {
        let re = Regex::new(r"(\d+)\s*players;\s*last\s*marble\s*is\s*worth\s*(\d+)\s*points?").unwrap();
        let caps = re.captures(input).unwrap();
        let players: usize = caps[1].parse().unwrap();
        let marbles: u32 = caps[2].parse().unwrap();
        Box::new(play_game(players, marbles * 100))
    }
}
