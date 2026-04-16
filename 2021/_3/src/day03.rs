use aoc_rust_common::Solution;
use std::fmt::Display;

pub struct Day03;

impl Solution for Day03 {
    fn year(&self) -> u32 { 2021 }
    fn day(&self) -> u32 { 3 }

    fn part1(&self, input: &str) -> Box<dyn Display> {
        let lines: Vec<&str> = input.lines().collect();
        let element_count = lines[0].len();
        let line_count = lines.len();
        let counts: Vec<i32> = lines.iter().fold(vec![0; element_count], |mut acc, line| {
            for (index, c) in line.chars().enumerate() {
                if c == '1' { acc[index] += 1; }
            }
            acc
        });

        let gamma: i32 = counts.iter().enumerate().fold(0, |acc, (i, &c)| {
            if c > (line_count / 2) as i32 { acc + (1 << (element_count - 1 - i)) } else { acc }
        });
        let epsilon: i32 = counts.iter().enumerate().fold(0, |acc, (i, &c)| {
            if c <= (line_count / 2) as i32 { acc + (1 << (element_count - 1 - i)) } else { acc }
        });
        Box::new(gamma * epsilon)
    }

    fn part2(&self, input: &str) -> Box<dyn Display> {
        let lines: Vec<&str> = input.lines().collect();
        let oxygen = filter_lines(lines.clone(), |c, pos, lines| {
            let ones = lines.iter().filter(|l| l.chars().nth(pos).unwrap() == '1').count();
            let target = if ones * 2 >= lines.len() { '1' } else { '0' };
            c == target
        });
        let co2 = filter_lines(lines, |c, pos, lines| {
            let ones = lines.iter().filter(|l| l.chars().nth(pos).unwrap() == '1').count();
            let target = if ones * 2 >= lines.len() { '0' } else { '1' };
            c == target
        });
        Box::new(oxygen * co2)
    }
}

fn filter_lines<F>(lines: Vec<&str>, predicate: F) -> i32 
where F: Fn(char, usize, &Vec<&str>) -> bool {
    let mut current_lines = lines;
    for pos in 0..current_lines[0].len() {
        if current_lines.len() == 1 { break; }
        let next_lines: Vec<&str> = current_lines.iter().filter(|l| predicate(l.chars().nth(pos).unwrap(), pos, &current_lines)).cloned().collect();
        current_lines = next_lines;
    }
    i32::from_str_radix(current_lines[0], 2).unwrap()
}
