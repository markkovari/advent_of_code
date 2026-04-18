use aoc_rust_common::Solution;
use std::collections::HashSet;
use std::fmt::Display;

pub struct Day19;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Rule {
    from: String,
    to: String,
}

fn read_rules_and_molecule(input: &str) -> (Vec<Rule>, String) {
    let mut rules = Vec::new();
    let mut molecule = String::new();
    let mut reading_rules = true;
    for line in input.lines() {
        if line.is_empty() {
            if !rules.is_empty() {
                reading_rules = false;
            }
            continue;
        }
        if reading_rules {
            if let Some(pos) = line.find(" => ") {
                rules.push(Rule {
                    from: line[..pos].to_owned(),
                    to: line[pos + 4..].to_owned(),
                });
            } else {
                molecule = line.to_owned();
                reading_rules = false;
            }
        } else {
            molecule = line.to_owned();
        }
    }
    (rules, molecule)
}

fn single_replacements(s: &str, from: &str, to: &str) -> Vec<String> {
    let mut results = Vec::new();
    for (start, part) in s.match_indices(from) {
        let mut string = String::new();
        string.push_str(&s[0..start]);
        string.push_str(to);
        string.push_str(&s[start + part.len()..]);
        results.push(string);
    }
    results
}

impl Solution for Day19 {
    fn year(&self) -> u32 {
        2015
    }
    fn day(&self) -> u32 {
        19
    }

    fn part1(&self, input: &str) -> Box<dyn Display> {
        let (rules, molecule) = read_rules_and_molecule(input);
        let mut results = HashSet::new();
        for rule in rules {
            for res in single_replacements(&molecule, &rule.from, &rule.to) {
                results.insert(res);
            }
        }
        Box::new(results.len() as i64)
    }

    fn part2(&self, input: &str) -> Box<dyn Display> {
        let (rules, molecule) = read_rules_and_molecule(input);
        // Greedy reduction from target molecule back to 'e'
        let mut target = molecule;
        let mut steps = 0;
        while target != "e" {
            let mut changed = false;
            for rule in &rules {
                if let Some(pos) = target.find(&rule.to) {
                    target.replace_range(pos..pos + rule.to.len(), &rule.from);
                    steps += 1;
                    changed = true;
                    break;
                }
            }
            if !changed {
                break;
            }
        }
        Box::new(steps as i64)
    }
}
