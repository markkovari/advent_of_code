use anyhow::Result;
use aoc_rust_common::Solution;

pub struct Day11;

fn increment(character: char) -> char {
    match character {
        'z' => 'a',
        _ => (character as u8 + 1) as char,
    }
}

fn has_two_non_overlapping_pairs(password: &str) -> bool {
    let mut pairs = 0;
    let mut index = 0;
    let chars: Vec<char> = password.chars().collect();
    loop {
        if index >= chars.len() - 1 {
            break;
        }
        let character = chars[index];
        let next_character = chars[index + 1];
        if character == next_character {
            pairs += 1;
            index += 2;
        } else {
            index += 1;
        }
    }
    pairs >= 2
}

fn is_valid_password(password: &str) -> bool {
    let chars: Vec<char> = password.chars().collect();
    let mut triples = 0;
    for window in chars.windows(3) {
        if window.iter().any(|&c| c == 'i' || c == 'o' || c == 'l') {
            return false;
        }
        if window[0] as u8 + 1 == window[1] as u8 && window[1] as u8 + 1 == window[2] as u8 {
            triples += 1;
        }
    }
    if !has_two_non_overlapping_pairs(password) {
        return false;
    }

    triples >= 1
}

fn create_new_password(password: String) -> String {
    let mut password = password;
    loop {
        password = increment_password(password);
        if is_valid_password(&password) {
            break;
        }
    }
    password
}

fn increment_password(password: String) -> String {
    let mut chars: Vec<char> = password.chars().collect();
    let mut index = chars.len() - 1;
    loop {
        let character = chars[index];
        let new_character = increment(character);
        chars[index] = new_character;
        if new_character == 'a' {
            if index == 0 {
                break;
            }
            index -= 1;
        } else {
            break;
        }
    }
    chars.iter().collect()
}

impl Solution for Day11 {
    fn year(&self) -> u32 {
        2015
    }
    fn day(&self) -> u32 {
        11
    }

    fn part1(&self, input: &str) -> Result<String> {
        Ok((create_new_password(input.trim().to_owned())).to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let p1 = create_new_password(input.trim().to_owned());
        Ok((create_new_password(p1)).to_string())
    }
}

aoc_rust_common::aoc_test!(Day11);
