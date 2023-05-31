use crate::{Excercise, Solvable};
use serde_json::Value;

struct TwelvfthDay {
    exercise: Excercise,
}

fn count_value(value: &Value) -> i32 {
    match value {
        Value::Number(e) => e.as_i64().unwrap() as i32,
        Value::Array(array) => array.iter().map(count_value).sum(),
        Value::Object(object) => object.values().map(count_value).sum(),
        _ => 0,
    }
}

fn skip_red(value: &Value) -> i32 {
    match value {
        Value::Number(e) => e.as_i64().unwrap() as i32,
        Value::Array(array) => array.iter().map(skip_red).sum(),
        Value::Object(object) => {
            if object.values().any(|v| v == "red") {
                0
            } else {
                object.values().map(skip_red).sum()
            }
        }
        _ => 0,
    }
}

impl TwelvfthDay {
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
        let value: Value = serde_json::from_str(&content).unwrap();
        count_value(&value) as i32
    }

    fn second(&self, content: String) -> i32 {
        let value: Value = serde_json::from_str(&content).unwrap();
        skip_red(&value) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("12_test.txt");
    const PROD: &str = include_str!("12_prod.txt");

    #[test]
    fn first_test() {
        let mut first_excersise = TwelvfthDay {
            exercise: Excercise {
                content: String::from(PROD),
                example: String::from(EXAMPLE),
            },
        };

        let expected_example = 3;
        let expected_prod = 156366;

        let result_example = first_excersise.solve_first(false);
        let result_prod = first_excersise.solve_first(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);

        let expected_example = 3;
        let expected_prod = 96852;

        let result_example = first_excersise.solve_second(false);
        let result_prod = first_excersise.solve_second(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);
    }
}
