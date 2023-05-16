use crate::{Excercise, Solvable};

struct FifthDay {
    exercise: Excercise,
    vowels: Vec<char>,
    filters: Vec<String>,
}

impl Solvable for FifthDay {
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
        content
            .lines()
            .filter(|line: &&str| line.chars().filter(|c| self.vowels.contains(c)).count() >= 3)
            .filter(|line: &&str| {
                line.chars()
                    .enumerate()
                    .any(|(i, c)| i > 0 && c == line.chars().nth(i - 1).unwrap())
            })
            .filter(|line: &&str| !self.filters.iter().any(|filter| line.contains(filter)))
            .count() as i32
    }

    fn second(&self, content: String) -> i32 {
        2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("5_test.txt");
    const PROD: &str = include_str!("5_prod.txt");

    #[test]
    fn first_test() {
        let first_excersise = FifthDay {
            exercise: Excercise {
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

        let result_example = first_excersise.solve_first(false);
        let result_prod = first_excersise.solve_first(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);

        let expected_example = 5714438;
        let expected_prod = 9958218;
        let result_example = first_excersise.solve_second(false);
        let result_prod = first_excersise.solve_second(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);
    }
}
