use std::collections::HashMap;

use crate::{Exercise, Solvable};

struct SeventhDay {
    exercise: Exercise,
}

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

fn parse_instructions(content: &str) -> Vec<Instruction> {
    content
        .lines()
        .map(|line| Instruction::try_from(line).unwrap())
        .collect::<Vec<Instruction>>()
}

/// Helper to resolve a value that might be either a number or a reference
fn resolve_operand(operand: &str, values: &HashMap<String, u16>) -> Option<u16> {
    operand
        .parse::<u16>()
        .ok()
        .or_else(|| values.get(operand).copied())
}

/// Helper to apply a binary operation if both operands are available
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

/// Calculate wire values using an iterative approach
fn calculate_values(instructions: Vec<Instruction>) -> HashMap<String, u16> {
    let mut values: HashMap<String, u16> = HashMap::new();
    let mut pending: HashMap<String, Operation> = instructions
        .into_iter()
        .map(|inst| (inst.target, inst.operation))
        .collect();

    // Keep processing until we can't resolve any more operations
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

        // Remove resolved operations
        for target in resolved {
            pending.remove(&target);
        }
    }

    values
}

impl Solvable for SeventhDay {
    fn solve_first(&self, is_prod: bool) -> i64 {
        if is_prod {
            self.first(&self.exercise.content)
        } else {
            self.first(&self.exercise.example)
        }
    }

    fn solve_second(&self, is_prod: bool) -> i64 {
        if is_prod {
            self.second(&self.exercise.content)
        } else {
            self.second(&self.exercise.example)
        }
    }

    fn first(&self, content: &str) -> i64 {
        let instructions = parse_instructions(content);
        let values = calculate_values(instructions);
        *values.get("a").unwrap_or(&0) as i64
    }

    fn second(&self, content: &str) -> i64 {
        let instructions = parse_instructions(content);
        let values = calculate_values(instructions.clone());
        let a_value = *values.get("a").unwrap_or(&0);

        // Update instructions to override wire 'b' with the value from 'a'
        let instructions = instructions
            .into_iter()
            .map(|inst| {
                if inst.target == "b" {
                    Instruction::new(String::from("b"), Operation::Assignment(a_value))
                } else {
                    inst
                }
            })
            .collect();

        let values = calculate_values(instructions);
        *values.get("a").unwrap_or(&0) as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("inputs/7_test.txt");
    const PROD: &str = include_str!("inputs/7_prod.txt");

    #[test]
    fn test_parse() {
        let instructions = parse_instructions(EXAMPLE);
        assert_eq!(instructions.len(), 8);
        assert_eq!(
            instructions[0],
            Instruction::new(String::from("x"), Operation::Assignment(123),)
        );
        assert_eq!(
            instructions[1],
            Instruction::new(String::from("y"), Operation::Assignment(456))
        );
        assert_eq!(
            instructions[4],
            Instruction::new(String::from("f"), Operation::LShift("x".to_owned(), 2))
        );
    }

    #[test]
    fn first_test() {
        let first_exercise = SeventhDay {
            exercise: Exercise {
                content: String::from(PROD),
                example: String::from(EXAMPLE),
            },
        };

        let expected_example = 0;
        let expected_prod = 46065;

        let result_example = first_exercise.solve_first(false);
        let result_prod = first_exercise.solve_first(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);

        let expected_example = 0;
        let expected_prod = 14134;
        let result_example = first_exercise.solve_second(false);
        let result_prod = first_exercise.solve_second(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);
    }
}
