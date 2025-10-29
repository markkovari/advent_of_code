use crate::{Exercise, Solvable};
use iter_tools::Itertools;
use rayon::prelude::*;

struct FifthDay {
    exercise: Exercise,
    vowels: Vec<char>,
    filters: Vec<String>,
}

impl Solvable for FifthDay {
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
        content
            .par_lines()
            .filter(|line: &&str| line.chars().filter(|c| self.vowels.contains(c)).count() >= 3)
            .filter(|line: &&str| {
                line.chars()
                    .enumerate()
                    .any(|(i, c)| i > 0 && c == line.chars().nth(i - 1).unwrap())
            })
            .filter(|line: &&str| !self.filters.iter().any(|filter| line.contains(filter)))
            .count() as i64
    }

    fn second(&self, content: &str) -> i64 {
        content
            .par_lines()
            .filter(|line| line.len() >= 3)
            .filter(|line: &&str| {
                line.chars()
                    .enumerate()
                    .any(|(i, c)| i > 2 && c == line.chars().nth(i - 2).unwrap())
            })
            .filter(|line: &&str| {
                line.chars()
                    .tuple_windows()
                    .any(|(c1, c2)| line.matches(&format!("{}{}", c1, c2)).count() > 1)
            })
            .count() as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("inputs/5_test.txt");
    const PROD: &str = include_str!("inputs/5_prod.txt");

    #[test]
    fn first_test() {
        let mut first_exercise = FifthDay {
            exercise: Exercise {
                content: String::from(PROD),
                example: String::from(EXAMPLE),
            },
            vowels: vec!['a', 'e', 'i', 'o', 'u'],
            filters: vec![
                String::from("ab"),
                String::from("cd"),
                String::from("pq"),
                String::from("xy"),
            ],
        };

        let expected_example = 2;
        let expected_prod = 255;

        let result_example = first_exercise.solve_first(false);
        let result_prod = first_exercise.solve_first(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);

        first_exercise.exercise.example =
            String::from("qjhvhtzxzqqjkmpb\nxxyxx\nuurcxstgmygtbstg\nieodomkazucvgmuy");

        let expected_example = 2;
        let expected_prod = 55;
        let result_example = first_exercise.solve_second(false);
        let result_prod = first_exercise.solve_second(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);
    }
}
