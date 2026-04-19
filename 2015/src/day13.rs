use anyhow::Result;
use aoc_rust_common::Solution;
use iter_tools::Itertools;
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;

pub struct Day13;

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

type SittingMap = HashMap<String, HashMap<String, i32>>;

fn evaluate_sitting(relations: Vec<Relation>) -> SittingMap {
    let mut sitting: SittingMap = HashMap::new();
    for relation in relations {
        sitting
            .entry(relation.from)
            .or_default()
            .insert(relation.to, relation.happiness);
    }
    sitting
}

fn calc_happiness(sitting: &SittingMap, order: &[&String]) -> i32 {
    let mut total = 0;
    for i in 0..order.len() {
        let from = order[i];
        let to = order[(i + 1) % order.len()];
        total += sitting.get(from).unwrap().get(to).unwrap_or(&0);
        total += sitting.get(to).unwrap().get(from).unwrap_or(&0);
    }
    total
}

impl Solution for Day13 {
    fn year(&self) -> u32 {
        2015
    }
    fn day(&self) -> u32 {
        13
    }

    fn part1(&self, input: &str) -> Result<String> {
        let relations_vec: Vec<Relation> = input
            .lines()
            .map(|l| Relation::try_from(l).unwrap())
            .collect();
        let relations = evaluate_sitting(relations_vec);
        let people: Vec<String> = relations.keys().map(|k| k.to_owned()).collect();
        let amount = people.len();
        Ok(people
            .iter()
            .permutations(amount)
            .par_bridge()
            .map(|permutation| calc_happiness(&relations, &permutation) as i64)
            .max()
            .unwrap_or(0)
            .to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let relations_vec: Vec<Relation> = input
            .lines()
            .map(|l| Relation::try_from(l).unwrap())
            .collect();
        let mut relations = evaluate_sitting(relations_vec);
        let people: Vec<String> = relations.keys().map(|k| k.to_owned()).collect();

        relations.insert("me".to_owned(), HashMap::new());
        for name in people {
            relations.get_mut(&name).unwrap().insert("me".to_owned(), 0);
            relations.get_mut("me").unwrap().insert(name, 0);
        }

        let people: Vec<String> = relations.keys().map(|k| k.to_owned()).collect();
        let amount = people.len();
        Ok(people
            .iter()
            .permutations(amount)
            .par_bridge()
            .map(|permutation| calc_happiness(&relations, &permutation) as i64)
            .max()
            .unwrap_or(0)
            .to_string())
    }
}

aoc_rust_common::aoc_test!(Day13);
