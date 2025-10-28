use super::{Excercise, Solvable};

pub struct SecondDay {
    exercise: Excercise,
}

fn calculate_wrapper_needed_materials(line: &str) -> (i32, i32) {
    let dimensions = line
        .split("x")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
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
        let materials = content
            .lines()
            .map(calculate_wrapper_needed_materials)
            .collect::<Vec<_>>();
        let mut wrapper = 0;
        for (w, _) in materials {
            wrapper += w;
        }
        wrapper
    }

    fn second(&self, content: String) -> i32 {
        let materials = content
            .lines()
            .map(calculate_wrapper_needed_materials)
            .collect::<Vec<_>>();
        let mut ribbon = 0;
        for (_, r) in materials {
            ribbon += r;
        }
        ribbon
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = include_str!("2_test.txt");
    const PROD_1: &str = include_str!("2_prod.txt");

    #[test]
    fn first_test() {
        let first_excersise = SecondDay {
            exercise: Excercise {
                content: String::from(PROD_1),
                example: String::from(EXAMPLE_1),
            },
        };

        let expected_example = 58;
        let expected_prod = 1588178;

        let result_example = first_excersise.solve_first(false);
        let result_prod = first_excersise.solve_first(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);

        let expected_example = 34;
        let expected_prod = 3783758;
        let result_example = first_excersise.solve_second(false);
        let result_prod = first_excersise.solve_second(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);
    }
}
