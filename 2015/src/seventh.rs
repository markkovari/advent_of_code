use crate::{Excercise, Solvable};

struct SeventhDay {
    exercise: Excercise,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Operation {
    Assignment(u32),
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
            1 => Ok(Operation::Assignment(parts[0].parse::<u32>().unwrap())),
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
        2
    }

    fn second(&self, content: String) -> i32 {
        2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("6_test.txt");
    const PROD: &str = include_str!("6_prod.txt");

    #[test]
    fn test_parse() {
        let operation = Operation::try_from("123 -> x").unwrap();
        assert_eq!(operation, Operation::Assignment(123));
    }

    #[test]
    #[ignore]
    fn first_test() {
        let mut first_excersise = SeventhDay {
            exercise: Excercise {
                content: String::from(PROD),
                example: String::from(EXAMPLE),
            },
        };

        let expected_example = 998996;
        let expected_prod = 569999;

        let result_example = first_excersise.solve_first(false);
        let result_prod = first_excersise.solve_first(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);

        first_excersise.exercise.example =
            String::from("turn on 0,0 through 0,0\ntoggle 0,0 through 999,999\n");

        let expected_example = 2000001;
        let expected_prod = 17836115;
        let result_example = first_excersise.solve_second(false);
        let result_prod = first_excersise.solve_second(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);
    }
}
