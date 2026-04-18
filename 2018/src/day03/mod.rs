use anyhow::Result;
use aoc_rust_common::Solution;
use std::collections::HashMap;

pub struct Day03;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Rule {
    id: String,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl TryFrom<&str> for Rule {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split(' ');
        let id = parts.next().ok_or("No id")?.to_owned();
        let _ = parts.next(); // @
        let mut coords = parts.next().ok_or("No coords")?.split(',');
        let x = coords
            .next()
            .ok_or("No x")?
            .parse::<i32>()
            .map_err(|e| e.to_string())?;
        let y = coords
            .next()
            .ok_or("No y")?
            .trim_end_matches(':')
            .parse::<i32>()
            .map_err(|e| e.to_string())?;
        let mut size = parts.next().ok_or("No size")?.split('x');
        let width = size
            .next()
            .ok_or("No width")?
            .parse::<i32>()
            .map_err(|e| e.to_string())?;
        let height = size
            .next()
            .ok_or("No height")?
            .parse::<i32>()
            .map_err(|e| e.to_string())?;
        Ok(Rule {
            id,
            x,
            y,
            width,
            height,
        })
    }
}

impl Solution for Day03 {
    fn year(&self) -> u32 {
        2018
    }
    fn day(&self) -> u32 {
        3
    }

    fn part1(&self, input: &str) -> Result<String> {
        let rules: Vec<Rule> = input.lines().map(|s| Rule::try_from(s).unwrap()).collect();
        let mut touched_fields = HashMap::new();
        for rule in rules {
            for x in rule.x..rule.x + rule.width {
                for y in rule.y..rule.y + rule.height {
                    *touched_fields.entry((x, y)).or_insert(0) += 1;
                }
            }
        }
        Ok((touched_fields.values().filter(|&&v| v > 1).count()).to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let rules: Vec<Rule> = input.lines().map(|s| Rule::try_from(s).unwrap()).collect();
        let mut touched_fields = HashMap::new();
        for rule in &rules {
            for x in rule.x..rule.x + rule.width {
                for y in rule.y..rule.y + rule.height {
                    touched_fields
                        .entry((x, y))
                        .or_insert_with(Vec::new)
                        .push(rule.id.clone());
                }
            }
        }

        for rule in &rules {
            let mut is_overlapping = false;
            for x in rule.x..rule.x + rule.width {
                for y in rule.y..rule.y + rule.height {
                    if touched_fields.get(&(x, y)).unwrap().len() > 1 {
                        is_overlapping = true;
                        break;
                    }
                }
                if is_overlapping {
                    break;
                }
            }
            if !is_overlapping {
                return Ok((rule.id.clone()).to_string());
            }
        }
        Ok(("Not found").to_string())
    }
}
