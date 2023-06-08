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

fn minimum_containers(combinations: Vec<Vec<u64>>) -> i64 {
    let mut map: HashMap<usize, Vec<Vec<u64>>> = HashMap::new();
    for combination in combinations {
        map.entry(combination.len())
            .and_modify(|e| e.push(combination.clone()))
            .or_insert(vec![combination]);
    }
    let min = map.keys().min().unwrap();
    map.get(min).unwrap().len() as i64
}

impl SeventeenthDay {
    fn solve_first(&self, is_prod: bool, amount: usize) -> i64 {
        if is_prod {
            self.first(self.exercise.content.to_owned(), amount)
        } else {
            self.first(self.exercise.example.to_owned(), amount)
        }
    }

    fn solve_second(&self, is_prod: bool, amount: u64) -> i64 {
        if is_prod {
            self.second(self.exercise.content.to_owned(), amount)
        } else {
            self.second(self.exercise.example.to_owned(), amount)
        }
    }

    fn first(&self, content: String, amount: usize) -> i64 {
        let containers = read_containers(content);
        get_combinations(containers, amount as u64).len() as i64
    }

    fn second(&self, content: String, amount: u64) -> i64 {
        let containers = read_containers(content);
        minimum_containers(get_combinations(containers, amount as u64))
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

        let expected_example = 3;
        let expected_prod = 17;
        let result_example = first_excersise.solve_second(false, 25);
        let result_prod = first_excersise.solve_second(true, 150);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);
    }
}
