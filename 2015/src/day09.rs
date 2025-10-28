use std::collections::HashMap;

use iter_tools::Itertools;

use crate::{Excercise, Solvable};

struct NinthDay {
    exercise: Excercise,
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

fn read_content(content: String) -> Vec<Distance> {
    content
        .lines()
        .map(|line| Distance::try_from(line).unwrap())
        .collect()
}

impl Solvable for NinthDay {
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
        lengths(&evaluate_paths(read_content(content))).iter().min().unwrap().clone() as i32
    }

    fn second(&self, content: String) -> i32 {
        lengths(&evaluate_paths(read_content(content))).iter().max().unwrap().clone() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("9_test.txt");
    const PROD: &str = include_str!("9_prod.txt");

    #[test]
    fn test_distance_parse() {
        let distance = Distance::try_from("London to Dublin = 464").unwrap();
        assert_eq!(distance.from, "London");
        assert_eq!(distance.to, "Dublin");
        assert_eq!(distance.distance, 464);
    }

    #[test]
    fn first_test() {
        let mut first_excersise = NinthDay {
            exercise: Excercise {
                content: String::from(PROD),
                example: String::from(EXAMPLE),
            },
        };

        let expected_example = 605;
        let expected_prod = 117;

        let result_example = first_excersise.solve_first(false);
        let result_prod = first_excersise.solve_first(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);

        let expected_example = 982;
        let expected_prod = 909;
        let result_example = first_excersise.solve_second(false);
        let result_prod = first_excersise.solve_second(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);
    }
}
