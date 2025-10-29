use super::{Exercise, Solvable};
use rayon::prelude::*;

pub struct SecondDay {
    exercise: Exercise,
}

/// Calculate wrapping paper and ribbon needed for a present
/// Returns (wrapping_paper_area, ribbon_length)
fn calculate_wrapper_needed_materials(line: &str) -> (i64, i64) {
    let dimensions = line
        .split("x")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let l = dimensions[0];
    let w = dimensions[1];
    let h = dimensions[2];
    let mut smallest = l * w;
    let mut smallest_perimeter = l + w;

    if w * h < smallest {
        smallest = w * h;
        smallest_perimeter = w + h;
    }
    if h * l < smallest {
        smallest = h * l;
        smallest_perimeter = h + l;
    }
    (
        2 * l * w + 2 * w * h + 2 * h * l + smallest,
        smallest_perimeter * 2 + l * w * h,
    )
}

impl Solvable for SecondDay {
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
            .map(calculate_wrapper_needed_materials)
            .map(|(wrapping, _)| wrapping)
            .sum()
    }

    fn second(&self, content: &str) -> i64 {
        content
            .par_lines()
            .map(calculate_wrapper_needed_materials)
            .map(|(_, ribbon)| ribbon)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = include_str!("inputs/2_test.txt");
    const PROD_1: &str = include_str!("inputs/2_prod.txt");

    #[test]
    fn first_test() {
        let first_exercise = SecondDay {
            exercise: Exercise {
                content: String::from(PROD_1),
                example: String::from(EXAMPLE_1),
            },
        };

        let expected_example = 58;
        let expected_prod = 1588178;

        let result_example = first_exercise.solve_first(false);
        let result_prod = first_exercise.solve_first(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);

        let expected_example = 34;
        let expected_prod = 3783758;
        let result_example = first_exercise.solve_second(false);
        let result_prod = first_exercise.solve_second(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);
    }
}
