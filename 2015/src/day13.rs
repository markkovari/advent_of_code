use std::{collections::HashMap, ops::AddAssign};

use crate::Exercise;
use iter_tools::Itertools;
use rayon::prelude::*;
use regex::Regex;

struct ThirteenthDay {
    exercise: Exercise,
}

#[derive(Debug, Clone)]
struct Relation {
    from: String,
    to: String,
    happiness: i32,
}

impl TryFrom<&str> for Relation {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let re =
            Regex::new(r"^(?P<from>\w+) would (?P<signum>gain|lose) (?P<amount>\d+) happiness units by sitting next to (?P<to>\w+).")
                .unwrap();
        let caps = re.captures(value).ok_or("No match")?;
        let from = caps.name("from").ok_or("No from")?.as_str().to_owned();
        let signum = if caps.name("signum").ok_or("No signum")?.as_str() == "gain" {
            1
        } else {
            -1
        };
        let mut happiness = caps
            .name("amount")
            .ok_or("No happiness")?
            .as_str()
            .parse::<i32>()
            .unwrap();

        let to = caps.name("to").ok_or("No to")?.as_str().to_owned();
        happiness *= signum;
        Ok(Relation {
            from,
            to,
            happiness,
        })
    }
}

fn read_relations(content: &str) -> Vec<Relation> {
    let mut result = Vec::new();
    for line in content.lines() {
        result.push(Relation::try_from(line).unwrap());
    }
    result
}

type SittingMap = HashMap<String, HashMap<String, i32>>;

fn evaluate_sitting(relations: Vec<Relation>) -> SittingMap {
    let mut sitting: SittingMap = HashMap::new();
    for relation in relations {
        let Relation {
            from,
            to,
            happiness,
        } = relation;
        if !sitting.contains_key(&from) {
            sitting.insert(from.clone(), HashMap::new());
        }

        if !sitting.contains_key(&to) {
            sitting.insert(to.clone(), HashMap::new());
        }
        sitting
            .get_mut(&from)
            .unwrap()
            .insert(to.to_owned(), happiness);
    }
    sitting
}

fn calc_happiness(sitting: &SittingMap, order: &Vec<&String>) -> i32 {
    let mut by_seats: HashMap<String, i32> = HashMap::new();
    for name in order {
        by_seats.insert((*name).to_owned(), 0);
    }
    for i in 0..order.len() {
        let from = &order[i];
        let to = &order[(i + 1) % order.len()];
        let from_happiness = sitting.get(*from).unwrap().get(*to).unwrap();
        let to_happiness = sitting.get(*to).unwrap().get(*from).unwrap();
        by_seats.get_mut(*from).unwrap().add_assign(to_happiness);
        by_seats.get_mut(*to).unwrap().add_assign(from_happiness);
    }
    by_seats.values().sum()
}

impl ThirteenthDay {
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
        let relations = evaluate_sitting(read_relations(content));
        let people: Vec<String> = relations.keys().map(|k| k.to_owned()).collect();
        let amount = people.len();
        people
            .iter()
            .permutations(amount)
            .par_bridge()
            .map(|permutation| calc_happiness(&relations, &permutation) as i64)
            .max()
            .unwrap_or(0)
    }

    fn second(&self, content: &str) -> i64 {
        let me = "me".to_owned();
        let withoutme = read_relations(content);
        let mut relations = evaluate_sitting(withoutme);
        relations.insert(me, HashMap::new());

        for name in relations.clone().keys() {
            relations
                .get_mut(&name.to_owned())
                .unwrap()
                .insert("me".to_owned(), 0);
            relations
                .get_mut("me")
                .unwrap()
                .insert(name.clone(), 0);
        }

        let people: Vec<String> = relations.keys().map(|k| k.to_owned()).collect();
        let amount = people.len();
        people
            .iter()
            .permutations(amount)
            .par_bridge()
            .map(|permutation| calc_happiness(&relations, &permutation) as i64)
            .max()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("inputs/13_test.txt");
    const PROD: &str = include_str!("inputs/13_prod.txt");

    #[test]
    fn test_relation() {
        let relation =
            Relation::try_from("Alice would gain 54 happiness units by sitting next to Bob.")
                .unwrap();
        assert_eq!(relation.from, "Alice");
        assert_eq!(relation.to, "Bob");
        assert_eq!(relation.happiness, 54);

        let relation =
            Relation::try_from("Bob would lose 2 happiness units by sitting next to Alice.")
                .unwrap();
        assert_eq!(relation.from, "Bob");
        assert_eq!(relation.to, "Alice");
        assert_eq!(relation.happiness, -2);
    }

    #[test]
    fn first_test() {
        let mut first_exercise = ThirteenthDay {
            exercise: Exercise {
                content: String::from(PROD),
                example: String::from(EXAMPLE),
            },
        };

        let expected_example = 330;
        let expected_prod = 709;
        let result_example = first_exercise.solve_first(false);
        let result_prod = first_exercise.solve_first(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);

        let expected_example = 286;
        let expected_prod = 668;
        let result_example = first_exercise.solve_second(false);
        let result_prod = first_exercise.solve_second(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);
    }
}
