use std::collections::HashMap;

use crate::{Excercise, Solvable};

struct SeventhDay {
    exercise: Excercise,
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

fn parse_instructions(content: String) -> Vec<Instruction> {
    content
        .lines()
        .map(|line| Instruction::try_from(line).unwrap())
        .collect::<Vec<Instruction>>()
}

fn calculate_values(instructions: Vec<Instruction>) -> HashMap<String, u16> {
    let mut values: HashMap<String, u16> = HashMap::new();
    let mut operations = instructions
        .iter()
        .map(|instruction| {
            (
                instruction.target.to_owned(),
                instruction.operation.to_owned(),
            )
        })
        .collect::<Vec<(String, Operation)>>();
    let mut current_operation_index = 0;
    loop {
        let (target, operation) = operations[current_operation_index].to_owned();
        match operation {
            Operation::Assignment(value) => {
                values.insert(target, value);
                operations.remove(current_operation_index);
            }
            Operation::AssignmentFromRefence(reference) => {
                if values.contains_key(&reference) {
                    let value = values.get(&reference).unwrap();
                    values.insert(target, *value);
                    operations.remove(current_operation_index);
                }
            }
            Operation::And(left, right) => {
                let left_as_number = left.parse::<u16>();
                let right_as_number = right.parse::<u16>();
                match (left_as_number, right_as_number) {
                    (Ok(left_value), Ok(right_value)) => {
                        values.insert(target, left_value & right_value);
                        operations.remove(current_operation_index);
                    }
                    (Ok(left_value), Err(_)) => {
                        if values.contains_key(&right) {
                            let right_value = values.get(&right).unwrap();
                            values.insert(target, left_value & right_value);
                            operations.remove(current_operation_index);
                        }
                    }
                    (Err(_), Ok(right_value)) => {
                        if values.contains_key(&left) {
                            let left_value = values.get(&left).unwrap();
                            values.insert(target, left_value & right_value);
                            operations.remove(current_operation_index);
                        }
                    }
                    _ => {
                        if values.contains_key(&left) && values.contains_key(&right) {
                            let left_value = values.get(&left).unwrap();
                            let right_value = values.get(&right).unwrap();
                            values.insert(target, left_value & right_value);
                            operations.remove(current_operation_index);
                        }
                    }
                }
            }
            Operation::Or(left, right) => {
                let left_as_number = left.parse::<u16>();
                let right_as_number = right.parse::<u16>();
                match (left_as_number, right_as_number) {
                    (Ok(left_value), Ok(right_value)) => {
                        values.insert(target, left_value | right_value);
                        operations.remove(current_operation_index);
                    }
                    (Ok(left_value), Err(_)) => {
                        if values.contains_key(&right) {
                            let right_value = values.get(&right).unwrap();
                            values.insert(target, left_value | right_value);
                            operations.remove(current_operation_index);
                        }
                    }
                    (Err(_), Ok(right_value)) => {
                        if values.contains_key(&left) {
                            let left_value = values.get(&left).unwrap();
                            values.insert(target, left_value | right_value);
                            operations.remove(current_operation_index);
                        }
                    }
                    _ => {
                        if values.contains_key(&left) && values.contains_key(&right) {
                            let left_value = values.get(&left).unwrap();
                            let right_value = values.get(&right).unwrap();
                            values.insert(target, left_value | right_value);
                            operations.remove(current_operation_index);
                        }
                    }
                }
            }
            Operation::LShift(left, right) => {
                if values.contains_key(&left) {
                    let left_value = values.get(&left).unwrap();
                    values.insert(target, left_value << right);
                    operations.remove(current_operation_index);
                }
            }
            Operation::RShift(left, right) => {
                if values.contains_key(&left) {
                    let left_value = values.get(&left).unwrap();
                    values.insert(target, left_value >> right);
                    operations.remove(current_operation_index);
                }
            }
            Operation::Not(left) => {
                if values.contains_key(&left) {
                    let left_value = values.get(&left).unwrap();
                    values.insert(target, !left_value);
                    operations.remove(current_operation_index);
                }
            }
        }
        if values.contains_key("a") {
            break;
        }
        if operations.len() == 0 {
            break;
        }
        current_operation_index += 1;
        current_operation_index %= operations.len();
    }
    values
}

impl Solvable for SeventhDay {
    fn solve_first(&self, is_prod: bool) -> i32 {
        if is_prod {
            self.first(self.exercise.content.to_owned())
        } else {
            self.first(self.exercise.example.to_owned())
        }
    }

    fn solve_second(&self, is_prod: bool) -> i32 {
        if is_prod {
            self.second(self.exercise.content.to_owned())
        } else {
            self.second(self.exercise.example.to_owned())
        }
    }

    fn first(&self, content: String) -> i32 {
        let instructions = parse_instructions(content);
        let values = calculate_values(instructions);
        *values.get("a").unwrap_or(&0) as i32
    }

    fn second(&self, content: String) -> i32 {
        let mut instructions = parse_instructions(content);
        let mut values = calculate_values(instructions.clone());
        let a_value = *values.get("a").unwrap_or(&0);
        values.insert("b".to_owned(), a_value);

        instructions = instructions
            .iter()
            .map(|x| match x.target.as_str() {
                "b" => Instruction::new(String::from("b"), Operation::Assignment(a_value as u16)),
                _ => x.clone(),
            })
            .collect();

        let values = calculate_values(instructions);
        *values.get("a").unwrap_or(&0) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("7_test.txt");
    const PROD: &str = include_str!("7_prod.txt");

    #[test]
    fn test_parse() {
        let instructions = parse_instructions(String::from(EXAMPLE));
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
        let mut first_excersise = SeventhDay {
            exercise: Excercise {
                content: String::from(PROD),
                example: String::from(EXAMPLE),
            },
        };

        let expected_example = 0;
        let expected_prod = 46065;

        let result_example = first_excersise.solve_first(false);
        let result_prod = first_excersise.solve_first(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);

        let expected_example = 0;
        let expected_prod = 14134;
        let result_example = first_excersise.solve_second(false);
        let result_prod = first_excersise.solve_second(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);
    }
}
