use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    ops::AddAssign,
};

use iter_tools::Itertools;

use crate::{Excercise, Solvable};
// use iter_tools::{dependency::itertools::Product, Itertools};
// use regex::Regex;

struct TwentiethDay {
    exercise: Excercise,
}

fn presents_at1(house_nr: usize) -> usize {
    divisors(house_nr).into_iter().sum::<usize>() * 10
}

fn divisors(until: usize) -> Vec<usize> {
    let small_divisors: Vec<usize> = Vec::from_iter(1..((until as f64).sqrt() as usize + 1))
        .into_iter()
        .filter(|i| until % *i == 0)
        .collect();

    let large_divisors: Vec<usize> = small_divisors
        .iter()
        .filter(|d| until != **d * **d)
        .map(|d| until / d)
        .collect();
    [small_divisors, large_divisors].concat()
}

impl TwentiethDay {
    fn solve_first(&self, until: usize) -> usize {
        self.first(&until)
    }

    fn solve_second(&self, until: usize) -> usize {
        self.second(&until)
    }

    fn first(&self, input: &usize) -> usize {
        for house_nr in 1..usize::MAX {
            if presents_at1(house_nr) >= *input {
                return house_nr;
            }
        }
        0
    }

    fn second(&self, until: &usize) -> usize {
        2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("19_test.txt");
    const PROD: &str = include_str!("19_prod.txt");

    #[test]
    fn first_test() {
        let mut first_excersise = TwentiethDay {
            exercise: Excercise {
                content: String::from(PROD),
                example: String::from(EXAMPLE),
            },
        };

        let expected_example = 4;
        let expected_prod = 831600;
        let result_example = first_excersise.first(&70);
        let result_prod = first_excersise.first(&36000000);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);

        let expected_example = 3;
        let expected_prod = 200;
        let result_example = first_excersise.second(&100);
        let result_prod = first_excersise.second(&36000000);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);
    }
}
