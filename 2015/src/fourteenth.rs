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

fn highest_score(raindeer: Vec<Raindeer>, duration: usize) -> usize {
    let mut scores: HashMap<String, usize> = HashMap::new();
    for name in raindeer.iter().map(|r| r.name.to_owned()) {
        scores.insert(name, 0);
    }
    for i in 1..=duration {
        let mut distances: HashMap<String, i32> = HashMap::new();
        for raindeer in raindeer.iter() {
            distances.insert(raindeer.name.to_owned(), raindeer.distance(i as i32));
        }
        let max_distance = distances.values().max().unwrap();
        for (name, distance) in distances.iter() {
            if distance == max_distance {
                scores.entry(name.to_owned()).and_modify(|e| *e += 1);
            }
        }
    }

    *scores.values().max().unwrap()
}

impl FourteenthDay {
    fn solve_first(&self, is_prod: bool, duration: usize) -> i32 {
        if is_prod {
            self.first(self.exercise.content.to_owned(), duration)
        } else {
            self.first(self.exercise.example.to_owned(), duration)
        }
    }

    fn solve_second(&self, is_prod: bool, duration: usize) -> i32 {
        if is_prod {
            self.second(self.exercise.content.to_owned(), duration)
        } else {
            self.second(self.exercise.example.to_owned(), duration)
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

    fn second(&self, content: String, duration: usize) -> i32 {
        highest_score(
            content
                .lines()
                .map(|line| Raindeer::try_from(line).unwrap())
                .collect(),
            duration,
        ) as i32
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

        let expected_example = 689;
        let expected_prod = 668;
        let result_example = first_excersise.solve_second(false, 1000);
        let result_prod = first_excersise.solve_second(true, 2503);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);
    }
}
