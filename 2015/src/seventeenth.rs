use std::{collections::HashMap, ops::AddAssign};

use crate::{Excercise, Solvable};
use iter_tools::{dependency::itertools::Product, Itertools};
use regex::Regex;

struct SeventeenthDay {
    exercise: Excercise,
}

fn read_containers(content: String) -> Vec<u64> {
    content
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect()
}

fn get_combinations(containers: Vec<u64>, liters: u64) -> Vec<Vec<u64>> {
    let mut combinations: Vec<Vec<u64>> = Vec::new();
    for i in 1..=containers.len() {
        for combination in containers.iter().combinations(i) {
            if combination.iter().map(|e| **e).sum::<u64>() == liters {
                combinations.push(combination.iter().map(|e| **e).collect());
            }
        }
    }
    combinations
}

impl SeventeenthDay {
    fn solve_first(&self, is_prod: bool, amount: usize) -> i64 {
        if is_prod {
            self.first(self.exercise.content.to_owned(), amount)
        } else {
            self.first(self.exercise.example.to_owned(), amount)
        }
    }

    fn solve_second(&self, is_prod: bool) -> i64 {
        if is_prod {
            self.second(self.exercise.content.to_owned())
        } else {
            self.second(self.exercise.example.to_owned())
        }
    }

    fn first(&self, content: String, amount: usize) -> i64 {
        let containers = read_containers(content);
        get_combinations(containers, amount as u64).len() as i64
    }

    fn second(&self, content: String) -> i64 {
        2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("17_test.txt");
    const PROD: &str = include_str!("17_prod.txt");

    #[test]
    fn first_test() {
        let mut first_excersise = SeventeenthDay {
            exercise: Excercise {
                content: String::from(PROD),
                example: String::from(EXAMPLE),
            },
        };

        let expected_example = 4;
        let expected_prod = 1638;
        let result_example = first_excersise.solve_first(false, 25);
        let result_prod = first_excersise.solve_first(true, 150);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);

        // // let expected_example = 57600000;
        // let expected_prod = 11171160;
        // // let result_example = first_excersise.solve_second(false);
        // let result_prod = first_excersise.solve_second(true);
        // // assert_eq!(expected_example, result_example);
        // assert_eq!(expected_prod, result_prod);
    }
}
