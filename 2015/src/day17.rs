use std::collections::HashMap;

use crate::Exercise;
use iter_tools::Itertools;
use rayon::prelude::*;

struct SeventeenthDay {
    exercise: Exercise,
}

fn read_containers(content: &str) -> Vec<u64> {
    content
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect()
}

fn get_combinations(containers: Vec<u64>, liters: u64) -> Vec<Vec<u64>> {
    (1..=containers.len())
        .into_par_iter()
        .flat_map(|i| {
            containers
                .iter()
                .combinations(i)
                .par_bridge()
                .filter_map(move |combination| {
                    if combination.iter().map(|e| **e).sum::<u64>() == liters {
                        Some(combination.iter().map(|e| **e).collect())
                    } else {
                        None
                    }
                })
                .collect::<Vec<Vec<u64>>>()
        })
        .collect()
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
            self.first(&self.exercise.content, amount)
        } else {
            self.first(&self.exercise.example, amount)
        }
    }

    fn solve_second(&self, is_prod: bool, amount: u64) -> i64 {
        if is_prod {
            self.second(&self.exercise.content, amount)
        } else {
            self.second(&self.exercise.example, amount)
        }
    }

    fn first(&self, content: &str, amount: usize) -> i64 {
        let containers = read_containers(content);
        get_combinations(containers, amount as u64).len() as i64
    }

    fn second(&self, content: &str, amount: u64) -> i64 {
        let containers = read_containers(content);
        minimum_containers(get_combinations(containers, amount))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("inputs/17_test.txt");
    const PROD: &str = include_str!("inputs/17_prod.txt");

    #[test]
    #[ignore = "take too long"]
    fn first_test() {
        let mut first_exercise = SeventeenthDay {
            exercise: Exercise {
                content: String::from(PROD),
                example: String::from(EXAMPLE),
            },
        };

        let expected_example = 4;
        let expected_prod = 1638;
        let result_example = first_exercise.solve_first(false, 25);
        let result_prod = first_exercise.solve_first(true, 150);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);

        let expected_example = 3;
        let expected_prod = 17;
        let result_example = first_exercise.solve_second(false, 25);
        let result_prod = first_exercise.solve_second(true, 150);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);
    }
}
