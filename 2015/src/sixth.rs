use crate::{Excercise, Solvable};

struct SixthDay {
    exercise: Excercise,
}

#[derive(Debug, PartialEq, Eq)]
enum Action {
    On,
    Off,
    Toggle,
}

impl TryFrom<&str> for Action {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.starts_with("turn on") {
            return Ok(Action::On);
        } else if value.starts_with("turn off") {
            return Ok(Action::Off);
        } else if value.starts_with("toggle") {
            return Ok(Action::Toggle);
        }
        Err(String::from("Invalid action"))
    }
}

struct Instruction {
    action: Action,
    start: Point,
    end: Point,
}

impl TryFrom<&str> for Instruction {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut split = value.split_whitespace().collect::<Vec<&str>>();
        let end: Point = Point::try_from(split.pop().unwrap())?;
        split.pop();
        let start: Point = Point::try_from(split.pop().unwrap())?;
        let action: Action = Action::try_from(split.join(" ").as_str())?;
        Ok(Instruction { action, start, end })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Point(i32, i32);

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point(x, y)
    }
}

impl TryFrom<&str> for Point {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut split = value.split(',');
        let x = split.next().unwrap().parse::<i32>().unwrap();
        let y = split.next().unwrap().parse::<i32>().unwrap();
        Ok(Point::new(x, y))
    }
}

impl Solvable for SixthDay {
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
        2
    }

    fn second(&self, content: String) -> i32 {
        2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("6_test.txt");
    const PROD: &str = include_str!("6_prod.txt");

    #[test]
    fn instruction() {
        let instruction = Instruction::try_from("turn on 0,0 through 999,999").unwrap();
        assert_eq!(instruction.action, Action::On);
        assert_eq!(instruction.start, Point::new(0, 0));
        assert_eq!(instruction.end, Point::new(999, 999));
    }
    #[test]
    fn first_test() {
        let mut first_excersise = SixthDay {
            exercise: Excercise {
                content: String::from(PROD),
                example: String::from(EXAMPLE),
            },
        };

        let expected_example = 2;
        let expected_prod = 255;

        let result_example = first_excersise.solve_first(false);
        let result_prod = first_excersise.solve_first(true);
        // assert_eq!(expected_example, result_example);
        // assert_eq!(expected_prod, result_prod);

        // let expected_example = 2;
        // let expected_prod = 55;
        // let result_example = first_excersise.solve_second(false);
        // let result_prod = first_excersise.solve_second(true);
        // assert_eq!(expected_example, result_example);
        // assert_eq!(expected_prod, result_prod);
    }
}
