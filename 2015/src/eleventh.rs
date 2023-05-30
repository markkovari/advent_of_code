use crate::{Excercise, Solvable};

struct EleventhDay {
    exercise: Excercise,
}

impl EleventhDay {
    fn solve_first(&self, is_prod: bool) -> String {
        if is_prod {
            self.first(self.exercise.content.to_owned())
        } else {
            self.first(self.exercise.example.to_owned())
        }
    }

    fn solve_second(&self, is_prod: bool) -> String {
        if is_prod {
            self.second(self.exercise.content.to_owned())
        } else {
            self.second(self.exercise.example.to_owned())
        }
    }

    fn first(&self, content: String) -> String {
        "asd".to_string()
    }

    fn second(&self, content: String) -> String {
        "asd".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("11_test.txt");
    const PROD: &str = include_str!("11_prod.txt");

    #[test]
    fn first_test() {
        let mut first_excersise = EleventhDay {
            exercise: Excercise {
                content: String::from(PROD),
                example: String::from(EXAMPLE),
            },
        };

        let expected_example = "asd";
        let expected_prod = "asd";

        let result_example = first_excersise.solve_first(false);
        let result_prod = first_excersise.solve_first(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);

        let expected_example = "asd";
        let expected_prod = "asd";
        let result_example = first_excersise.solve_second(false);
        let result_prod = first_excersise.solve_second(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);
    }
}
