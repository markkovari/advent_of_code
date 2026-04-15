use std::collections::HashMap;
use iter_tools::Itertools;
use rayon::prelude::*;
use aoc_rust_common::Solution;
use std::fmt::Display;

pub struct Day09;

struct Distance {
    from: String,
    to: String,
    distance: usize,
}

impl Distance {
    fn new(from: String, to: String, distance: usize) -> Self {
        Self { from, to, distance }
    }
}

impl TryFrom<&str> for Distance {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut split = value.split(' ');
        let from = split.next().ok_or("Missing from")?.to_owned();
        let to = split.nth(1).ok_or("Missing to")?.to_owned();
        let distance = split.nth(1).ok_or("Missing distance")?.parse::<usize>().map_err(|_| "Invalid distance")?;
        Ok(Self::new(from, to, distance))
    }
}
type RoutesMap = HashMap<String, HashMap<String, usize>>;

fn evaluate_paths(distances: Vec<Distance>) -> RoutesMap {
    let mut routes: RoutesMap = HashMap::new();
    for distance in distances {
        let Distance { from, to, distance } = distance;
        routes.entry(from.clone()).or_default().insert(to.clone(), distance);
        routes.entry(to).or_default().insert(from, distance);
    }
    routes
}

fn lengths(routes: &RoutesMap) -> Vec<usize> {
    routes
        .keys()
        .permutations(routes.keys().len())
        .par_bridge()
        .map(|permutation| {
            let mut length = 0;
            for route in permutation.windows(2) {
                let from = route[0];
                let to = route[1];
                length += routes.get(from).unwrap().get(to).unwrap();
            }
            length
        })
        .collect()
}

impl Solution for Day09 {
    fn year(&self) -> u32 { 2015 }
    fn day(&self) -> u32 { 9 }

    fn part1(&self, input: &str) -> Box<dyn Display> {
        let distances = input.lines().map(|l| Distance::try_from(l).unwrap()).collect();
        Box::new(*lengths(&evaluate_paths(distances)).iter().min().unwrap() as i64)
    }

    fn part2(&self, input: &str) -> Box<dyn Display> {
        let distances = input.lines().map(|l| Distance::try_from(l).unwrap()).collect();
        Box::new(*lengths(&evaluate_paths(distances)).iter().max().unwrap() as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day09() {
        let day = Day09;
        let example = "London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141";
        assert_eq!(day.part1(example).to_string(), "605");
        assert_eq!(day.part2(example).to_string(), "982");
    }
}
