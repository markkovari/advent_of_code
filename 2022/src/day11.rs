use aoc_rust_common::Solution;
use std::collections::VecDeque;
use std::fmt::Display;

pub struct Day11;

#[derive(Clone, Copy)]
enum Op {
    Add(usize),
    Mul(usize),
    AddOld,
    MulOld,
}

impl Op {
    fn apply(&self, old: usize) -> usize {
        match self {
            Op::Add(c) => old + c,
            Op::Mul(c) => old * c,
            Op::AddOld => old + old,
            Op::MulOld => old * old,
        }
    }
}

#[derive(Clone)]
struct Monkey {
    items: VecDeque<usize>,
    op: Op,
    div: usize,
    if_true: usize,
    if_false: usize,
    inspections: usize,
}

fn parse(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|block| {
            let lines: Vec<&str> = block.lines().collect();
            let items = lines[1][18..]
                .split(", ")
                .map(|s| s.parse().unwrap())
                .collect();
            let op_parts: Vec<&str> = lines[2].split_whitespace().collect();
            let op_val = op_parts[5];
            let op = match (op_parts[4], op_val) {
                ("+", "old") => Op::AddOld,
                ("*", "old") => Op::MulOld,
                ("+", _) => Op::Add(op_val.parse().unwrap()),
                ("*", _) => Op::Mul(op_val.parse().unwrap()),
                _ => unreachable!(),
            };
            let div = lines[3].split_whitespace().last().unwrap().parse().unwrap();
            let if_true = lines[4].split_whitespace().last().unwrap().parse().unwrap();
            let if_false = lines[5].split_whitespace().last().unwrap().parse().unwrap();
            Monkey {
                items,
                op,
                div,
                if_true,
                if_false,
                inspections: 0,
            }
        })
        .collect()
}

fn solve(input: &str, rounds: usize, part1: bool) -> usize {
    let mut monkeys = parse(input);
    let common_divisor: usize = monkeys.iter().map(|m| m.div).product();
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop_front() {
                monkeys[i].inspections += 1;
                let mut worry = monkeys[i].op.apply(item);
                if part1 {
                    worry /= 3;
                } else {
                    worry %= common_divisor;
                }

                let target = if worry % monkeys[i].div == 0 {
                    monkeys[i].if_true
                } else {
                    monkeys[i].if_false
                };
                monkeys[target].items.push_back(worry);
            }
        }
    }
    let mut ins: Vec<usize> = monkeys.iter().map(|m| m.inspections).collect();
    ins.sort_by(|a, b| b.cmp(a));
    ins[0] * ins[1]
}

impl Solution for Day11 {
    fn year(&self) -> u32 {
        2022
    }
    fn day(&self) -> u32 {
        11
    }

    fn part1(&self, input: &str) -> Box<dyn Display> {
        Box::new(solve(input, 20, true))
    }

    fn part2(&self, input: &str) -> Box<dyn Display> {
        Box::new(solve(input, 10000, false))
    }
}
