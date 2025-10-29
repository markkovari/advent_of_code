use lazy_static::lazy_static;
use regex::Regex;

use crate::Exercise;

struct TwentyFifthDay {
    exercise: Exercise,
}

struct DiagonalIterator {
    row: usize,
    col: usize,
}

impl DiagonalIterator {
    fn from(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

impl Iterator for DiagonalIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.row == 1 {
            self.row = self.col + 1;
            self.col = 1;
        } else {
            self.row -= 1;
            self.col += 1;
        }

        Some((self.row, self.col))
    }
}

impl TwentyFifthDay {
    fn solve_first(&self, input: &str) -> usize {
        self.first(input)
    }

    fn solve_second(&self, input: &str) -> usize {
        self.second(input)
    }

    fn first(&self, input: &str) -> usize {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^To continue, please consult the code grid in the manual.  Enter the code at row (\d+), column (\d+).$").unwrap();
        }
        let captures = RE.captures(input).unwrap();

        let row = captures.get(1).unwrap().as_str().parse().unwrap();
        let col = captures.get(2).unwrap().as_str().parse().unwrap();
        let input_position = &(row, col);

        let mut value = 20151125;

        for position in DiagonalIterator::from(1, 1) {
            value = (value * 252533) % 33554393;

            if position == *input_position {
                break;
            }
        }

        value
    }

    fn second(&self, input: &str) -> usize {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^To continue, please consult the code grid in the manual.  Enter the code at row (\d+), column (\d+).$").unwrap();
        }
        let captures = RE.captures(input).unwrap();

        let row = captures.get(1).unwrap().as_str().parse().unwrap();
        let col = captures.get(2).unwrap().as_str().parse().unwrap();
        let input_position = &(row, col);

        let mut value = 20151125;

        for position in DiagonalIterator::from(1, 1) {
            value = (value * 252533) % 33554393;

            if position == *input_position {
                break;
            }
        }

        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("inputs/25_prod.txt");
    const PROD: &str = include_str!("inputs/25_prod.txt");

    #[test]
    fn first_test() {
        let mut first_exercise = TwentyFifthDay {
            exercise: Exercise {
                content: String::from(PROD),
                example: String::from(EXAMPLE),
            },
        };

        let expected_example = 2650453;
        let expected_prod = 2650453;
        let result_example = first_exercise.first(PROD);
        let result_prod = first_exercise.first(PROD);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);

        let expected_example = 2650453;
        let expected_prod = 2650453;
        let result_example = first_exercise.second(PROD);
        let result_prod = first_exercise.second(PROD);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);
    }
}
