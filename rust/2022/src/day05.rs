use anyhow::Result;
use aoc_rust_common::Solution;
use std::str::FromStr;

pub struct Day05;

#[derive(Debug, Clone)]
struct CargoStack {
    elements: Vec<String>,
}

#[derive(Debug, Clone)]
struct Shipment {
    stacks: Vec<CargoStack>,
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

impl FromStr for Shipment {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let last_line = lines
            .last()
            .ok_or_else(|| anyhow::anyhow!("empty shipment input"))?;
        let num_stacks = (last_line.len() + 2) / 4;
        let mut stacks = vec![
            CargoStack {
                elements: Vec::new()
            };
            num_stacks
        ];
        for line in lines.iter().rev().skip(1) {
            for i in 0..num_stacks {
                let char_idx = 1 + i * 4;
                if let Some(c) = line.chars().nth(char_idx) {
                    if c != ' ' {
                        stacks[i].elements.push(c.to_string());
                    }
                }
            }
        }
        Ok(Shipment { stacks })
    }
}

impl FromStr for Instruction {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() < 6 {
            return Err(anyhow::anyhow!("invalid instruction: {}", s));
        }
        Ok(Instruction {
            amount: parts[1].parse()?,
            from: parts[3].parse::<usize>()? - 1,
            to: parts[5].parse::<usize>()? - 1,
        })
    }
}

impl Solution for Day05 {
    fn year(&self) -> u32 {
        2022
    }
    fn day(&self) -> u32 {
        5
    }

    fn part1(&self, input: &str) -> Result<String> {
        let (mut shipment, instructions) = parse(input)?;
        for inst in instructions {
            for _ in 0..inst.amount {
                let item = shipment.stacks[inst.from]
                    .elements
                    .pop()
                    .ok_or_else(|| anyhow::anyhow!("popping from empty stack"))?;
                shipment.stacks[inst.to].elements.push(item);
            }
        }
        Ok(shipment
            .stacks
            .iter()
            .map(|s| s.elements.last().cloned().unwrap_or_default())
            .collect::<String>())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let (mut shipment, instructions) = parse(input)?;
        for inst in instructions {
            let mut moved = Vec::new();
            for _ in 0..inst.amount {
                moved.push(
                    shipment.stacks[inst.from]
                        .elements
                        .pop()
                        .ok_or_else(|| anyhow::anyhow!("popping from empty stack"))?,
                );
            }
            while let Some(item) = moved.pop() {
                shipment.stacks[inst.to].elements.push(item);
            }
        }
        Ok(shipment
            .stacks
            .iter()
            .map(|s| s.elements.last().cloned().unwrap_or_default())
            .collect::<String>())
    }
}

fn parse(input: &str) -> Result<(Shipment, Vec<Instruction>)> {
    let parts: Vec<&str> = input.split("\n\n").collect();
    if parts.len() < 2 {
        return Err(anyhow::anyhow!(
            "invalid input: missing shipment or instructions"
        ));
    }
    let shipment = parts[0].parse()?;
    let instructions = parts[1]
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<Instruction>>>()?;
    Ok((shipment, instructions))
}

aoc_rust_common::aoc_test!(Day05);
