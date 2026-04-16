use aoc_rust_common::Solution;
use std::fmt::Display;

pub struct Day08;

fn sum_metadata(data: &mut &[i32]) -> i32 {
    let child_count = data[0];
    let meta_count = data[1];
    *data = &data[2..];
    
    let mut sum = 0;
    for _ in 0..child_count {
        sum += sum_metadata(data);
    }

    for _ in 0..meta_count {
        sum += data[0];
        *data = &data[1..];
    }
    sum
}

fn node_value(data: &mut &[i32]) -> i32 {
    let child_count = data[0] as usize;
    let meta_count = data[1] as usize;
    *data = &data[2..];

    if child_count == 0 {
        let mut sum = 0;
        for _ in 0..meta_count {
            sum += data[0];
            *data = &data[1..];
        }
        return sum;
    }
    
    let mut child_values = Vec::new();
    for _ in 0..child_count {
        child_values.push(node_value(data));
    }
    
    let mut value = 0;
    for _ in 0..meta_count {
        let meta_val = data[0] as usize;
        if meta_val > 0 && meta_val <= child_count {
            value += child_values[meta_val - 1];
        }
        *data = &data[1..];
    }
    value
}

impl Solution for Day08 {
    fn year(&self) -> u32 { 2018 }
    fn day(&self) -> u32 { 8 }

    fn part1(&self, input: &str) -> Box<dyn Display> {
        let numbers: Vec<i32> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();
        Box::new(sum_metadata(&mut &numbers[..]))
    }

    fn part2(&self, input: &str) -> Box<dyn Display> {
        let numbers: Vec<i32> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();
        Box::new(node_value(&mut &numbers[..]))
    }
}
