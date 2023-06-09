use onig::Regex;

use crate::{Excercise, Solvable};

struct EightDay {
    exercise: Excercise,
}

pub fn raw_and_unescaped_len(s: &str) -> (usize, usize) {
    if s.chars().nth(0) != Some('"') || s.chars().last() != Some('"') {
        panic!("invalid format (not quoted)");
    }
    let raw_len = s.len();
    let re = Regex::new(r#"\\(\\|"|x[0-9a-f]{2})"#).unwrap();
    let ss = &s[1..s.len() - 1];
    let (esc_count, esc_size) =
        re.find_iter(ss)
            .fold((0, 0), |(esc_count, esc_size), (start_pos, end_pos)| {
                (esc_count + 1, esc_size + (end_pos - start_pos))
            });
    (raw_len, raw_len - 2 - esc_size + esc_count)
}

pub fn extra_chars_unescaped(text: &str) -> usize {
    text.lines().fold(0, |extra_chars, line| {
        let (raw_len, unescaped_len) = raw_and_unescaped_len(line);
        extra_chars + (raw_len - unescaped_len)
    })
}

pub fn raw_and_reescaped_len(s: &str) -> (usize, usize) {
    let raw_len = s.len();
    let re = Regex::new(r#"[\\"]"#).unwrap();
    let esc_count = re.find_iter(s).count();
    (raw_len, raw_len + 2 + esc_count)
}

pub fn extra_chars_reescaped(text: &str) -> usize {
    text.lines().fold(0, |extra_chars, line| {
        let (raw_len, reescaped_len) = raw_and_reescaped_len(line);
        extra_chars + (reescaped_len - raw_len)
    })
}

impl Solvable for EightDay {
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
        extra_chars_unescaped(&content) as i32
    }

    fn second(&self, content: String) -> i32 {
        extra_chars_reescaped(&content) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("8_test.txt");
    const PROD: &str = include_str!("8_prod.txt");

    #[test]
    fn first_test() {
        let mut first_excersise = EightDay {
            exercise: Excercise {
                content: String::from(PROD),
                example: String::from(EXAMPLE),
            },
        };

        let expected_example = 12;
        let expected_prod = 1350;

        let result_example = first_excersise.solve_first(false);
        let result_prod = first_excersise.solve_first(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);

        let expected_example = 19;
        let expected_prod = 2085;
        let result_example = first_excersise.solve_second(false);
        let result_prod = first_excersise.solve_second(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);
    }
}
