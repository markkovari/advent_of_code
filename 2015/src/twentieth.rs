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

impl TwentiethDay {
    fn solve_first(&self, is_prod: bool, starting: String) -> i64 {
        if is_prod {
            self.first(self.exercise.content.to_owned(), starting)
        } else {
            self.first(self.exercise.example.to_owned(), starting)
        }
    }

    fn solve_second(&self, is_prod: bool, starting: String) -> i64 {
        if is_prod {
            self.second(self.exercise.content.to_owned(), starting)
        } else {
            self.second(self.exercise.example.to_owned(), starting)
        }
    }

    fn first(&self, content: String, starting: String) -> i64 {
        2
    }

    fn second(&self, content: String, starting: String) -> i64 {
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

        let expected_example = 7;
        let expected_prod = 518;
        let result_example = first_excersise.solve_first(false, "HOHOHO".to_owned());
        let result_prod = first_excersise.solve_first(true, BASE.clone());
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);

        let expected_example = 3;
        let expected_prod = 200;
        let result_example = first_excersise.second(example_second, "HOH".to_owned());
        let result_prod = first_excersise.solve_second(true, BASE);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);
    }
}
