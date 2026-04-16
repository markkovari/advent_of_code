use aoc_rust_common::Solution;
use std::fmt::Display;
use std::str::FromStr;

pub struct Day10;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Instruction { Noop, AddX(i32) }

impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        match parts[0] {
            "noop" => Ok(Instruction::Noop),
            "addx" => Ok(Instruction::AddX(parts[1].parse().unwrap())),
            _ => Err(()),
        }
    }
}

impl Solution for Day10 {
    fn year(&self) -> u32 { 2022 }
    fn day(&self) -> u32 { 10 }

    fn part1(&self, input: &str) -> Box<dyn Display> {
        let instructions: Vec<Instruction> = input.lines().map(|l| l.parse().unwrap()).collect();
        let mut x = 1;
        let mut cycle = 0;
        let mut signal_strength = 0;
        
        let mut run_cycle = |x, cycle| {
            if (cycle - 20) % 40 == 0 { signal_strength += cycle * x; }
        };

        for inst in instructions {
            match inst {
                Instruction::Noop => { cycle += 1; run_cycle(x, cycle); }
                Instruction::AddX(v) => {
                    for _ in 0..2 { cycle += 1; run_cycle(x, cycle); }
                    x += v;
                }
            }
        }
        Box::new(signal_strength)
    }

    fn part2(&self, input: &str) -> Box<dyn Display> {
        let instructions: Vec<Instruction> = input.lines().map(|l| l.parse().unwrap()).collect();
        let mut x = 1;
        let mut cycle = 0;
        let mut crt = String::new();

        let mut draw_pixel = |x: i32, cycle: i32| {
            let pos = (cycle - 1) % 40;
            if (x - 1..=x + 1).contains(&pos) { crt.push('#'); } else { crt.push('.'); }
            if cycle % 40 == 0 { crt.push('\n'); }
        };

        for inst in instructions {
            match inst {
                Instruction::Noop => { cycle += 1; draw_pixel(x, cycle); }
                Instruction::AddX(v) => {
                    for _ in 0..2 { cycle += 1; draw_pixel(x, cycle); }
                    x += v;
                }
            }
        }
        Box::new(format!("\n{}", crt))
    }
}
