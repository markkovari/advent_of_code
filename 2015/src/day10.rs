use crate::{Excercise, Solvable};

struct TenthDay {
    exercise: Excercise,
}

fn look_and_say(current: String) -> String {
    let mut result = String::new();
    let mut current_char = current.chars().nth(0).unwrap();
    let mut current_count = 0;
    for c in current.chars() {
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
    result
}

impl Solvable for TenthDay {
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
        let mut content = content;
        for _ in 0..40 {
            content = look_and_say(content);
        }
        content.len() as i32
    }

    fn second(&self, content: String) -> i32 {
        let mut content = content;
        for _ in 0..50 {
            content = look_and_say(content);
        }
        content.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("10_test.txt");
    const PROD: &str = include_str!("10_prod.txt");

    #[test]
    #[ignore = "takes too long"]
    fn first_test() {
        let mut first_excersise = TenthDay {
            exercise: Excercise {
                content: String::from(PROD),
                example: String::from(EXAMPLE),
            },
        };

        let expected_example = 82350;
        let expected_prod = 360154;

        let result_example = first_excersise.solve_first(false);
        let result_prod = first_excersise.solve_first(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);

        let expected_example = 1166642;
        let expected_prod = 5103798;
        let result_example = first_excersise.solve_second(false);
        let result_prod = first_excersise.solve_second(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);
    }
}
