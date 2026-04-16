use aoc_rust_common::Solution;
use std::fmt::Display;
use regex::Regex;

pub struct Day12;

impl Solution for Day12 {
    fn year(&self) -> u32 { 2018 }
    fn day(&self) -> u32 { 12 }

    fn part1(&self, input: &str) -> Box<dyn Display> {
        let (initial_state, rules) = parse_input(input);
        let result = run_simulation(initial_state, rules, 20);
        Box::new(result)
    }

    fn part2(&self, input: &str) -> Box<dyn Display> {
        let (initial_state, rules) = parse_input(input);
        // After ~100 generations, the pattern stabilizes and shifts right by 1 pot each generation.
        // The sum increases by a fixed amount each time.
        let val_100 = run_simulation(initial_state.clone(), rules.clone(), 100);
        let val_101 = run_simulation(initial_state, rules, 101);
        let diff = val_101 - val_100;
        let result = val_100 + (50_000_000_000 - 100) * diff;
        Box::new(result)
    }
}

fn parse_input(input: &str) -> (Vec<bool>, [bool; 32]) {
    let mut lines = input.lines();
    let initial_state_str = lines.next().unwrap().replace("initial state: ", "");
    let initial_state: Vec<bool> = initial_state_str.chars().map(|c| c == '#').collect();
    
    lines.next(); // Skip empty line
    let mut rules = [false; 32];
    let re = Regex::new(r"([.#]{5}) => ([.#])").unwrap();
    for line in lines {
        let caps = re.captures(line).unwrap();
        let pattern: Vec<bool> = caps[1].chars().map(|c| c == '#').collect();
        let result = &caps[2] == "#";
        let index = pattern.iter().fold(0, |acc, &bit| (acc << 1) | bit as usize);
        rules[index] = result;
    }
    (initial_state, rules)
}

fn run_simulation(initial_state: Vec<bool>, rules: [bool; 32], generations: i64) -> i64 {
    let mut state = initial_state;
    let mut zero_offset = 0i64;

    for _ in 0..generations {
        let last_plant = state.iter().rposition(|&p| p).unwrap_or(0);

        let mut next_state = Vec::new();
        let new_len = last_plant + 5;
        let old_len = state.len();
        if new_len > old_len {
            state.resize(new_len, false);
        }
        
        let start_padding = 4;
        let end_padding = 4;
        
        state.splice(0..0, vec![false; start_padding]);
        state.extend(vec![false; end_padding]);
        zero_offset -= start_padding as i64;
        
        for i in 2..state.len() - 2 {
            let pattern = &state[i-2..=i+2];
            let index = pattern.iter().fold(0, |acc, &bit| (acc << 1) | bit as usize);
            next_state.push(rules[index]);
        }
        state = next_state;
    }
    
    state.iter().enumerate().filter(|(_, &p)| p).map(|(i, _)| i as i64 + zero_offset).sum()
}
