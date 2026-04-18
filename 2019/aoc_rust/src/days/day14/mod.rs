use anyhow::Result;
use aoc_rust_common::Solution;
use std::collections::HashMap;

pub struct Day14;

#[derive(Debug)]
struct Quantity {
    element: String,
    count: i64,
}

#[derive(Debug)]
struct Reaction {
    input: Vec<Quantity>,
    output: Quantity,
}

type Buckets = HashMap<String, (i64, i64)>; // total, in_use

fn ore_required(
    reactions: &[Reaction],
    element: &str,
    buckets: &mut Buckets,
    ore: &mut i64,
    amount: i64,
) {
    if element == "ORE" {
        *ore += amount;
        return;
    }

    if let Some(reaction) = reactions.iter().find(|&x| x.output.element == element) {
        let (total, in_use) = buckets.get(element).cloned().unwrap_or((0, 0));
        let surplus = total - in_use;

        if surplus >= amount {
            buckets.get_mut(element).unwrap().1 += amount;
            return;
        }

        let needed = amount - surplus;
        let num_reactions = (needed as f64 / reaction.output.count as f64).ceil() as i64;
        let produced = reaction.output.count * num_reactions;

        buckets.entry(element.to_string()).or_insert((0, 0)).0 += produced;
        buckets.get_mut(element).unwrap().1 += amount;

        for input_quantity in &reaction.input {
            ore_required(
                reactions,
                &input_quantity.element,
                buckets,
                ore,
                input_quantity.count * num_reactions,
            );
        }
    }
}

fn parse_reactions(input: &str) -> Vec<Reaction> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(" => ").collect();
            let output_part: Vec<&str> = parts[1].split(' ').collect();
            let output = Quantity {
                count: output_part[0].parse().unwrap(),
                element: output_part[1].to_string(),
            };

            let inputs = parts[0]
                .split(", ")
                .map(|s| {
                    let input_part: Vec<&str> = s.split(' ').collect();
                    Quantity {
                        count: input_part[0].parse().unwrap(),
                        element: input_part[1].to_string(),
                    }
                })
                .collect();

            Reaction {
                input: inputs,
                output,
            }
        })
        .collect()
}

impl Solution for Day14 {
    fn year(&self) -> u32 {
        2019
    }
    fn day(&self) -> u32 {
        14
    }

    fn part1(&self, input: &str) -> Result<String> {
        let reactions = parse_reactions(input);
        let mut buckets = Buckets::new();
        let mut ore = 0;
        ore_required(&reactions, "FUEL", &mut buckets, &mut ore, 1);
        Ok((ore).to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let reactions = parse_reactions(input);
        let target_ore = 1_000_000_000_000;
        let mut lower_bound = 1;
        let mut upper_bound = target_ore;

        while lower_bound < upper_bound {
            let mid = upper_bound - (upper_bound - lower_bound) / 2;
            let mut buckets = Buckets::new();
            let mut ore = 0;
            ore_required(&reactions, "FUEL", &mut buckets, &mut ore, mid);
            if ore > target_ore {
                upper_bound = mid - 1;
            } else {
                lower_bound = mid;
            }
        }
        Ok((lower_bound).to_string())
    }
}
