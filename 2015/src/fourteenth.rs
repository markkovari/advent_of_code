use std::{collections::HashMap, ops::AddAssign};

use crate::{Excercise, Solvable};
use iter_tools::Itertools;
use regex::Regex;

struct FourteenthDay {
    exercise: Excercise,
}

struct Raindeer {
    name: String,
    speed: i32,
    fly_time: i32,
    rest_time: i32,
}

impl Raindeer {
    fn distance(&self, time: i32) -> i32 {
        let cycle_time = self.fly_time + self.rest_time;
        let cycles = time / cycle_time;
        let remaining_time = time % cycle_time;
        let mut distance = cycles * self.speed * self.fly_time;
        if remaining_time > self.fly_time {
            distance += self.speed * self.fly_time;
        } else {
            distance += self.speed * remaining_time;
        }
        distance
    }
}

impl TryFrom<&str> for Raindeer {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let re = Regex::new(
            r"(?P<name>\w+) can fly (?P<speed>\d+) km/s for (?P<duration>\d+) seconds, but then must rest for (?P<rest_time>\d+) seconds.",
        );
        let caps = re.unwrap().captures(value).unwrap();
        let name = caps.name("name").unwrap().as_str();
        let speed = caps.name("speed").unwrap().as_str().parse::<i32>().unwrap();
        let fly_time = caps
            .name("duration")
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();
        let rest_time = caps
            .name("rest_time")
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();
        Ok(Raindeer {
            name: String::from(name),
            speed,
            fly_time,
            rest_time,
        })
    }
}

fn highest_distance(raindeer: Vec<Raindeer>, duration: usize) -> usize {
    raindeer
        .iter()
        .map(|r| r.distance(duration as i32))
        .max()
        .unwrap() as usize
}

impl FourteenthDay {
    fn solve_first(&self, is_prod: bool, duration: usize) -> i32 {
        if is_prod {
            self.first(self.exercise.content.to_owned(), duration)
        } else {
            self.first(self.exercise.example.to_owned(), duration)
        }
    }

    fn solve_second(&self, is_prod: bool) -> i32 {
        if is_prod {
            self.second(self.exercise.content.to_owned())
        } else {
            self.second(self.exercise.example.to_owned())
        }
    }

    fn first(&self, content: String, duration: usize) -> i32 {
        highest_distance(
            content
                .lines()
                .map(|line| Raindeer::try_from(line).unwrap())
                .collect(),
            duration,
        ) as i32
    }

    fn second(&self, content: String) -> i32 {
        2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("14_test.txt");
    const PROD: &str = include_str!("14_prod.txt");

    #[test]
    fn first_test() {
        let mut first_excersise = FourteenthDay {
            exercise: Excercise {
                content: String::from(PROD),
                example: String::from(EXAMPLE),
            },
        };

        let expected_example = 1120;
        let expected_prod = 2655;
        let result_example = first_excersise.solve_first(false, 1000);
        let result_prod = first_excersise.solve_first(true, 2503);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);

        let expected_example = 286;
        let expected_prod = 668;
        let result_example = first_excersise.solve_second(false);
        let result_prod = first_excersise.solve_second(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);
    }
}
