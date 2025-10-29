use crate::{Exercise, Solvable};
use md5;

struct FourthDay {
    exercise: Exercise,
}

impl Solvable for FourthDay {
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
        let mut counter = 0;
        loop {
            let data = format!("{}{}", content, counter);
            let calculated_hash = md5::compute(data);
            let hash_as_string = format!("{:x}", calculated_hash);
            if hash_as_string.starts_with("00000") {
                return counter;
            }
            counter += 1;
        }
    }

    fn second(&self, content: &str) -> i64 {
        let mut counter = 0;
        loop {
            let data = format!("{}{}", content, counter);
            let calculated_hash = md5::compute(data);
            let hash_as_string = format!("{:x}", calculated_hash);
            if hash_as_string.starts_with("000000") {
                return counter;
            }
            counter += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("inputs/4_test.txt");
    const PROD: &str = include_str!("inputs/4_prod.txt");

    #[test]
    #[ignore = "Takes too long"]
    fn first_test() {
        let first_exercise = FourthDay {
            exercise: Exercise {
                content: String::from(PROD),
                example: String::from(EXAMPLE),
            },
        };

        let expected_example = 1048970;
        let expected_prod = 346386;

        let result_example = first_exercise.solve_first(false);
        let result_prod = first_exercise.solve_first(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);

        let expected_example = 5714438;
        let expected_prod = 9958218;
        let result_example = first_exercise.solve_second(false);
        let result_prod = first_exercise.solve_second(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);
    }
}
