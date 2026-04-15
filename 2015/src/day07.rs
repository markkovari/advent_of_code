use std::collections::HashMap;
use aoc_rust_common::Solution;
use std::fmt::Display;

pub struct Day07;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Operation {
    Assignment(u16),
    AssignmentFromRefence(String),
    And(String, String),
    Or(String, String),
    LShift(String, usize),
    RShift(String, usize),
    Not(String),
}

impl TryFrom<&str> for Operation {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts = value.split_whitespace().collect::<Vec<&str>>();
        match parts.len() {
            1 => match parts[0].parse::<u16>() {
                Ok(value) => Ok(Operation::Assignment(value)),
                Err(_) => Ok(Operation::AssignmentFromRefence(parts[0].to_owned())),
            },
            2 => match parts[0] {
                "NOT" => Ok(Operation::Not(parts[1].to_owned())),
                _ => Err("Invalid operation"),
            },
            3 => match parts[1] {
                "AND" => Ok(Operation::And(parts[0].to_owned(), parts[2].to_owned())),
                "OR" => Ok(Operation::Or(parts[0].to_owned(), parts[2].to_owned())),
                "LSHIFT" => Ok(Operation::LShift(
                    parts[0].to_owned(),
                    parts[2].parse::<usize>().unwrap(),
                )),
                "RSHIFT" => Ok(Operation::RShift(
                    parts[0].to_owned(),
                    parts[2].parse::<usize>().unwrap(),
                )),
                _ => Err("Invalid operation"),
            },
            _ => Err("Invalid operation"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Instruction {
    target: String,
    operation: Operation,
}

impl Instruction {
    fn new(target: String, operation: Operation) -> Self {
        Instruction { target, operation }
    }
}

impl TryFrom<&str> for Instruction {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts = value.split(" -> ").collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err("Invalid instruction");
        }
        let target = parts[1].to_owned();
        let operation = Operation::try_from(parts[0])?;
        Ok(Instruction::new(target, operation))
    }
}

fn resolve_operand(operand: &str, values: &HashMap<String, u16>) -> Option<u16> {
    operand
        .parse::<u16>()
        .ok()
        .or_else(|| values.get(operand).copied())
}

fn apply_binary_op<F>(
    left: &str,
    right: &str,
    values: &HashMap<String, u16>,
    op: F,
) -> Option<u16>
where
    F: Fn(u16, u16) -> u16,
{
    resolve_operand(left, values)
        .and_then(|l| resolve_operand(right, values).map(|r| op(l, r)))
}

fn calculate_values(instructions: &[Instruction]) -> HashMap<String, u16> {
    let mut values: HashMap<String, u16> = HashMap::new();
    let mut pending: HashMap<String, Operation> = instructions
        .iter()
        .map(|inst| (inst.target.clone(), inst.operation.clone()))
        .collect();

    while !pending.is_empty() {
        let mut resolved = Vec::new();

        for (target, operation) in pending.iter() {
            let result = match operation {
                Operation::Assignment(value) => Some(*value),
                Operation::AssignmentFromRefence(reference) => values.get(reference).copied(),
                Operation::And(left, right) => apply_binary_op(left, right, &values, |l, r| l & r),
                Operation::Or(left, right) => apply_binary_op(left, right, &values, |l, r| l | r),
                Operation::LShift(left, amount) => {
                    values.get(left).map(|v| v << amount)
                }
                Operation::RShift(left, amount) => {
                    values.get(left).map(|v| v >> amount)
                }
                Operation::Not(left) => values.get(left).map(|v| !v),
            };

            if let Some(value) = result {
                values.insert(target.clone(), value);
                resolved.push(target.clone());
            }
        }

        if resolved.is_empty() { break; }

        for target in resolved {
            pending.remove(&target);
        }
    }

    values
}

impl Solution for Day07 {
    fn year(&self) -> u32 { 2015 }
    fn day(&self) -> u32 { 7 }

    fn part1(&self, input: &str) -> Box<dyn Display> {
        let instructions: Vec<Instruction> = input.lines()
            .map(|l| Instruction::try_from(l).unwrap())
            .collect();
        let values = calculate_values(&instructions);
        Box::new(*values.get("a").unwrap_or(&0) as i64)
    }

    fn part2(&self, input: &str) -> Box<dyn Display> {
        let instructions: Vec<Instruction> = input.lines()
            .map(|l| Instruction::try_from(l).unwrap())
            .collect();
        let values = calculate_values(&instructions);
        let a_value = *values.get("a").unwrap_or(&0);

        let new_instructions: Vec<Instruction> = instructions
            .into_iter()
            .map(|inst| {
                if inst.target == "b" {
                    Instruction::new(String::from("b"), Operation::Assignment(a_value))
                } else {
                    inst
                }
            })
            .collect();

        let values = calculate_values(&new_instructions);
        Box::new(*values.get("a").unwrap_or(&0) as i64)
    }
}
