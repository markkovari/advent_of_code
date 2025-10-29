use crate::{Exercise, Solvable};

struct TenthDay {
    exercise: Exercise,
}

/// Perform look-and-say transformation on the input
fn look_and_say(current: &str) -> String {
    let mut result = String::with_capacity(current.len() * 2);
    let mut chars = current.chars();

    if let Some(mut current_char) = chars.next() {
        let mut current_count = 1;

        for c in chars {
            if c == current_char {
                current_count += 1;
            } else {
                result.push_str(&current_count.to_string());
                result.push(current_char);
                current_char = c;
                current_count = 1;
            }
        }
        result.push_str(&current_count.to_string());
        result.push(current_char);
    }

    result
}

impl Solvable for TenthDay {
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
        let mut result = content.to_string();
        for _ in 0..40 {
            result = look_and_say(&result);
        }
        result.len() as i64
    }

    fn second(&self, content: &str) -> i64 {
        let mut result = content.to_string();
        for _ in 0..50 {
            result = look_and_say(&result);
        }
        result.len() as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("inputs/10_test.txt");
    const PROD: &str = include_str!("inputs/10_prod.txt");

    #[test]
    #[ignore = "takes too long"]
    fn first_test() {
        let first_exercise = TenthDay {
            exercise: Exercise {
                content: String::from(PROD),
                example: String::from(EXAMPLE),
            },
        };

        let expected_example = 82350;
        let expected_prod = 360154;

        let result_example = first_exercise.solve_first(false);
        let result_prod = first_exercise.solve_first(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);

        let expected_example = 1166642;
        let expected_prod = 5103798;
        let result_example = first_exercise.solve_second(false);
        let result_prod = first_exercise.solve_second(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);
    }
}
