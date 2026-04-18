use aoc_rust_common::Solution;
use regex::Regex;
use std::collections::HashMap;
use std::fmt::Display;

pub struct Day14;

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
        distance += self.speed * remaining_time.min(self.fly_time);
        distance
    }
}

impl TryFrom<&str> for Raindeer {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let re = Regex::new(
            r"(?P<name>\w+) can fly (?P<speed>\d+) km/s for (?P<duration>\d+) seconds, but then must rest for (?P<rest_time>\d+) seconds.",
        ).unwrap();
        let caps = re.captures(value).ok_or("No match")?;
        let name = caps.name("name").unwrap().as_str().to_owned();
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
            name,
            speed,
            fly_time,
            rest_time,
        })
    }
}

impl Solution for Day14 {
    fn year(&self) -> u32 {
        2015
    }
    fn day(&self) -> u32 {
        14
    }

    fn part1(&self, input: &str) -> Box<dyn Display> {
        let duration = 2503;
        let raindeer: Vec<Raindeer> = input
            .lines()
            .map(|line| Raindeer::try_from(line).unwrap())
            .collect();
        Box::new(raindeer.iter().map(|r| r.distance(duration)).max().unwrap() as i64)
    }

    fn part2(&self, input: &str) -> Box<dyn Display> {
        let duration = 2503;
        let raindeer: Vec<Raindeer> = input
            .lines()
            .map(|line| Raindeer::try_from(line).unwrap())
            .collect();
        let mut scores: HashMap<String, usize> =
            raindeer.iter().map(|r| (r.name.clone(), 0)).collect();

        for i in 1..=duration {
            let distances: Vec<(String, i32)> = raindeer
                .iter()
                .map(|r| (r.name.clone(), r.distance(i)))
                .collect();
            let max_distance = distances.iter().map(|(_, d)| *d).max().unwrap();
            for (name, distance) in distances {
                if distance == max_distance {
                    *scores.get_mut(&name).unwrap() += 1;
                }
            }
        }
        Box::new(*scores.values().max().unwrap() as i64)
    }
}
