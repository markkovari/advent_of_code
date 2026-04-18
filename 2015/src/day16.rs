use aoc_rust_common::Solution;
use regex::Regex;
use std::collections::HashMap;
use std::fmt::Display;

pub struct Day16;

struct Sue {
    name: String,
    items: HashMap<String, i64>,
}

impl TryFrom<&str> for Sue {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let re = Regex::new(r"^Sue (?P<name>\d+): (?P<items>.+)$").map_err(|_| "Invalid regex")?;
        let caps = re.captures(value).ok_or("No match")?;
        let name = caps.name("name").unwrap().as_str().to_owned();
        let data = caps.name("items").unwrap().as_str();
        let mut map = HashMap::new();
        for datum in data.split(", ") {
            let parts: Vec<&str> = datum.split(": ").collect();
            map.insert(parts[0].to_string(), parts[1].parse().unwrap());
        }
        Ok(Self { name, items: map })
    }
}

fn get_search_elements() -> HashMap<String, i64> {
    HashMap::from([
        ("children".into(), 3),
        ("cats".into(), 7),
        ("samoyeds".into(), 2),
        ("pomeranians".into(), 3),
        ("akitas".into(), 0),
        ("vizslas".into(), 0),
        ("goldfish".into(), 5),
        ("trees".into(), 3),
        ("cars".into(), 2),
        ("perfumes".into(), 1),
    ])
}

impl Solution for Day16 {
    fn year(&self) -> u32 {
        2015
    }
    fn day(&self) -> u32 {
        16
    }

    fn part1(&self, input: &str) -> Box<dyn Display> {
        let search = get_search_elements();
        let sues: Vec<Sue> = input
            .lines()
            .map(|line| Sue::try_from(line).unwrap())
            .collect();
        for sue in sues {
            let mut possible = true;
            for (key, val) in &search {
                if let Some(&sue_val) = sue.items.get(key) {
                    if sue_val != *val {
                        possible = false;
                        break;
                    }
                }
            }
            if possible {
                return Box::new(sue.name);
            }
        }
        Box::new("Not found")
    }

    fn part2(&self, input: &str) -> Box<dyn Display> {
        let search = get_search_elements();
        let sues: Vec<Sue> = input
            .lines()
            .map(|line| Sue::try_from(line).unwrap())
            .collect();
        for sue in sues {
            let mut possible = true;
            for (key, val) in &search {
                if let Some(&sue_val) = sue.items.get(key) {
                    let match_ok = match key.as_str() {
                        "cats" | "trees" => sue_val > *val,
                        "pomeranians" | "goldfish" => sue_val < *val,
                        _ => sue_val == *val,
                    };
                    if !match_ok {
                        possible = false;
                        break;
                    }
                }
            }
            if possible {
                return Box::new(sue.name);
            }
        }
        Box::new("Not found")
    }
}
