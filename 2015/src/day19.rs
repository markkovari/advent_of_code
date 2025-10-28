use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    ops::AddAssign,
};

use iter_tools::Itertools;

use crate::{Excercise, Solvable};
// use iter_tools::{dependency::itertools::Product, Itertools};
// use regex::Regex;

struct NineteenthDay {
    exercise: Excercise,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Rule {
    from: String,
    to: String,
}

impl TryFrom<&str> for Rule {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut split = value.split(" => ");
        let from = split.next().ok_or("No from")?;
        let to = split.next().ok_or("No to")?;
        Ok(Rule {
            from: from.to_owned(),
            to: to.to_owned(),
        })
    }
}

fn read_rules(content: String) -> Vec<Rule> {
    content
        .lines()
        .map(|line| Rule::try_from(line).unwrap())
        .collect()
}

fn apply(replacements: &Vec<Rule>, puzzle: &str) -> HashSet<String> {
    let mut mols: HashSet<String> = HashSet::new();
    for Rule { from, to } in replacements.iter() {
        for res in variants(puzzle, from, to) {
            mols.insert(res);
        }
    }
    mols
}

fn variants(puzzle: &str, from: &str, to: &str) -> HashSet<String> {
    HashSet::from_iter(
        puzzle
            .single_replacements(from, to)
            .iter()
            .map(|s| s.to_owned()),
    )
}

trait SingleReplacements {
    fn single_replacements(&self, from: &str, to: &str) -> Vec<String>;
}

impl<'a> SingleReplacements for &'a str {
    fn single_replacements(&self, from: &str, to: &str) -> Vec<String> {
        let mut results = Vec::new();
        for (start, part) in self.match_indices(from) {
            let mut string = String::new();
            string.push_str(self.get(0..start).unwrap());
            string.push_str(to);
            string.push_str(self.get(start + part.len()..self.len()).unwrap());
            results.push(string);
        }
        results
    }
}

fn find_production(replacements: &Vec<Rule>, input: String, depth: usize) -> Option<usize> {
    let input = input.as_str();
    if input == "e" {
        return Some(depth);
    }
    for next_step in replacements
        .iter()
        .flat_map(|Rule { from, to }| input.single_replacements(to, from).into_iter())
        .unique()
    {
        if let Some(count) = find_production(replacements, next_step, depth + 1) {
            return Some(count);
        }
    }
    None
}

impl NineteenthDay {
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
        let rules = read_rules(content);
        let mols = apply(&rules, &starting);
        mols.len() as i64
    }

    fn second(&self, content: String, starting: String) -> i64 {
        let rules = read_rules(content);

        find_production(&rules, starting, 0).unwrap() as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("19_test.txt");
    const PROD: &str = include_str!("19_prod.txt");

    #[test]
    fn first_test() {
        let BASE: std::string::String = "CRnSiRnCaPTiMgYCaPTiRnFArSiThFArCaSiThSiThPBCaCaSiRnSiRnTiTiMgArPBCaPMgYPTiRnFArFArCaSiRnBPMgArPRnCaPTiRnFArCaSiThCaCaFArPBCaCaPTiTiRnFArCaSiRnSiAlYSiThRnFArArCaSiRnBFArCaCaSiRnSiThCaCaCaFYCaPTiBCaSiThCaSiThPMgArSiRnCaPBFYCaCaFArCaCaCaCaSiThCaSiRnPRnFArPBSiThPRnFArSiRnMgArCaFYFArCaSiRnSiAlArTiTiTiTiTiTiTiRnPMgArPTiTiTiBSiRnSiAlArTiTiRnPMgArCaFYBPBPTiRnSiRnMgArSiThCaFArCaSiThFArPRnFArCaSiRnTiBSiThSiRnSiAlYCaFArPRnFArSiThCaFArCaCaSiThCaCaCaSiRnPRnCaFArFYPMgArCaPBCaPBSiRnFYPBCaFArCaSiAl".to_owned();
        let example_second: std::string::String = "e => H
e => O
H => HO
H => OH
O => HH"
            .to_owned();

        let mut first_excersise = NineteenthDay {
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
