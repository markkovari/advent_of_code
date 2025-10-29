use std::collections::HashMap;

use iter_tools::Itertools;
use rayon::prelude::*;

use crate::{Exercise, Solvable};

struct NinthDay {
    exercise: Exercise,
}

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
        let mut split = value.split(" ");
        let from = split.next().unwrap().to_owned();
        let to = split.nth(1).unwrap().to_owned();
        let distance = split.nth(1).unwrap().parse::<usize>().unwrap();
        Ok(Self::new(from, to, distance))
    }
}
type RoutesMap = HashMap<String, HashMap<String, usize>>;

fn evaluate_paths(distances: Vec<Distance>) -> RoutesMap {
    let mut routes: RoutesMap = HashMap::new();
    for distance in distances {
        let Distance { from, to, distance } = distance;
        if !routes.contains_key(&from) {
            routes.insert(from.clone(), HashMap::new());
        }

        if !routes.contains_key(&to) {
            routes.insert(to.clone(), HashMap::new());
        }
        routes
            .get_mut(&from)
            .unwrap()
            .insert(to.to_owned(), distance);
        routes
            .get_mut(&to)
            .unwrap()
            .insert(from.to_owned(), distance);
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

fn read_content(content: &str) -> Vec<Distance> {
    content
        .lines()
        .map(|line| Distance::try_from(line).unwrap())
        .collect()
}

impl Solvable for NinthDay {
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
        *lengths(&evaluate_paths(read_content(content)))
            .iter()
            .min()
            .unwrap() as i64
    }

    fn second(&self, content: &str) -> i64 {
        *lengths(&evaluate_paths(read_content(content)))
            .iter()
            .max()
            .unwrap() as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("inputs/9_test.txt");
    const PROD: &str = include_str!("inputs/9_prod.txt");

    #[test]
    fn test_distance_parse() {
        let distance = Distance::try_from("London to Dublin = 464").unwrap();
        assert_eq!(distance.from, "London");
        assert_eq!(distance.to, "Dublin");
        assert_eq!(distance.distance, 464);
    }

    #[test]
    fn first_test() {
        let mut first_exercise = NinthDay {
            exercise: Exercise {
                content: String::from(PROD),
                example: String::from(EXAMPLE),
            },
        };

        let expected_example = 605;
        let expected_prod = 117;

        let result_example = first_exercise.solve_first(false);
        let result_prod = first_exercise.solve_first(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);

        let expected_example = 982;
        let expected_prod = 909;
        let result_example = first_exercise.solve_second(false);
        let result_prod = first_exercise.solve_second(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);
    }
}
