use crate::{Exercise, Solvable};

struct FirstDay {
    exercise: Exercise,
}

impl Solvable for FirstDay {
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
        content.chars().fold(0, |acc, c| match c {
            '(' => acc + 1,
            ')' => acc - 1,
            _ => acc,
        })
    }

    fn second(&self, content: &str) -> i64 {
        let mut floor = 0;
        for (pos, c) in content.chars().enumerate() {
            floor += match c {
                '(' => 1,
                ')' => -1,
                _ => 0,
            };
            if floor == -1 {
                return (pos + 1) as i64;
            }
        }
        (content.len() + 1) as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = include_str!("inputs/1_test.txt");
    const PROD_1: &str = include_str!("inputs/1_prod.txt");

    #[test]
    fn first_test() {
        let first_exercise = FirstDay {
            exercise: Exercise {
                content: String::from(PROD_1),
                example: String::from(EXAMPLE_1),
            },
        };

        let expected_example = 3;
        let expected_prod = 232;

        let result_example = first_exercise.solve_first(false);
        let result_prod = first_exercise.solve_first(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);

        let expected_example = 1;
        let expected_prod = 1783;
        let result_example = first_exercise.solve_second(false);
        let result_prod = first_exercise.solve_second(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);
    }
}
