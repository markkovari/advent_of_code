use crate::{Excercise, Solvable};

struct FirstDay {
    exercise: Excercise,
}

impl Solvable for FirstDay {
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
        let mut result: i32 = 0;
        for c in content.chars() {
            if c == '(' {
                result += 1;
            } else if c == ')' {
                result -= 1;
            }
        }
        result
    }

    fn second(&self, content: String) -> i32 {
        let mut result: i32 = 0;
        for (at, c) in content.chars().enumerate() {
            if c == '(' {
                result += 1;
            } else if c == ')' {
                result -= 1;
            }
            if result == -1 {
                return (at + 1) as i32;
            }
        }
        return (content.len() + 1) as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = include_str!("1_test.txt");
    const PROD_1: &str = include_str!("1_prod.txt");

    #[test]
    fn first_test() {
        let first_excersise = FirstDay {
            exercise: Excercise {
                content: String::from(PROD_1),
                example: String::from(EXAMPLE_1),
            },
        };

        let expected_example = 3;
        let expected_prod = 232;

        let result_example = first_excersise.solve_first(false);
        let result_prod = first_excersise.solve_first(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);

        let expected_example = 1;
        let expected_prod = 1783;
        let result_example = first_excersise.solve_second(false);
        let result_prod = first_excersise.solve_second(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);
    }
}
