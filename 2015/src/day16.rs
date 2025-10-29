use std::collections::HashMap;

use crate::Exercise;
use iter_tools::Itertools;
use regex::Regex;

struct SixteenthDay {
    exercise: Exercise,
}

struct Sue {
    name: String,
    items: HashMap<String, i64>,
}

impl TryFrom<&str> for Sue {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let re = Regex::new(r"^Sue (?P<name>\d+): (?P<items>.+)$");

        if re.is_err() {
            return Err("Regex is not valid");
        }
        let caps = re.unwrap().captures(value).unwrap();
        let name = caps.name("name").unwrap().as_str().parse().unwrap();
        let data = caps.name("items").unwrap().as_str().to_string();
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

impl SixteenthDay {
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
        let search = get_search_elements();
        let sues = content
            .lines()
            .map(|line| Sue::try_from(line).unwrap())
            .collect_vec();
        for sue in sues {
            let mut possible = true;
            for key in search.keys() {
                if sue.items.contains_key(key) && sue.items.get(key) != search.get(key) {
                    possible = false;
                    break;
                }
            }
            if possible {
                return sue.name.parse().unwrap();
            }
        }
        -1
    }

    fn second(&self, content: &str) -> i64 {
        let search = get_search_elements();
        let sues = content
            .lines()
            .map(|line| Sue::try_from(line).unwrap())
            .collect_vec();
        for sue in sues {
            let mut possible = true;
            for key in search.keys() {
                if sue.items.contains_key(key)
                    && match key.as_str() {
                        "cats" | "trees" => sue.items.get(key) <= search.get(key),
                        "pomeranians" | "goldfish" => sue.items.get(key) >= search.get(key),
                        _ => sue.items.get(key) != search.get(key),
                    }
                {
                    possible = false;
                    break;
                }
            }
            if possible {
                return sue.name.parse().unwrap();
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("inputs/16_test.txt");
    const PROD: &str = include_str!("inputs/16_prod.txt");

    #[test]
    fn first_test() {
        let mut first_exercise = SixteenthDay {
            exercise: Exercise {
                content: String::from(PROD),
                example: String::from(EXAMPLE),
            },
        };

        // let expected_example = 62842880;
        let expected_prod = 103;
        // let result_example = first_exercise.solve_first(false);
        let result_prod = first_exercise.solve_first(true);
        // assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);

        // let expected_example = 57600000;
        let expected_prod = 405;
        // let result_example = first_exercise.solve_second(false);
        let result_prod = first_exercise.solve_second(true);
        // assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);
    }
}
