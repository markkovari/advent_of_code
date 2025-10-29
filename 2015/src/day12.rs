use crate::Exercise;
use serde_json::Value;

struct TwelvfthDay {
    exercise: Exercise,
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
        let value: Value = serde_json::from_str(content).unwrap();
        count_value(&value) as i64
    }

    fn second(&self, content: &str) -> i64 {
        let value: Value = serde_json::from_str(content).unwrap();
        skip_red(&value) as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("inputs/12_test.txt");
    const PROD: &str = include_str!("inputs/12_prod.txt");

    #[test]
    fn first_test() {
        let mut first_exercise = TwelvfthDay {
            exercise: Exercise {
                content: String::from(PROD),
                example: String::from(EXAMPLE),
            },
        };

        let expected_example = 3;
        let expected_prod = 156366;

        let result_example = first_exercise.solve_first(false);
        let result_prod = first_exercise.solve_first(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);

        let expected_example = 3;
        let expected_prod = 96852;

        let result_example = first_exercise.solve_second(false);
        let result_prod = first_exercise.solve_second(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);
    }
}
