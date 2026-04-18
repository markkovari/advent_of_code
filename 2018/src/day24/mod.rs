use anyhow::Result;
use aoc_rust_common::Solution;
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub struct Day24;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Team {
    ImmuneSystem,
    Infection,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Group {
    id: usize,
    team: Team,
    units: i32,
    hp: i32,
    attack: i32,
    attack_type: String,
    initiative: i32,
    weaknesses: HashSet<String>,
    immunities: HashSet<String>,
}

impl Group {
    fn effective_power(&self) -> i32 {
        self.units * self.attack
    }

    fn damage_to(&self, other: &Group) -> i32 {
        if other.immunities.contains(&self.attack_type) {
            return 0;
        }
        let power = self.effective_power();
        if other.weaknesses.contains(&self.attack_type) {
            power * 2
        } else {
            power
        }
    }
}

fn parse(input: &str) -> Vec<Group> {
    let mut groups = Vec::new();
    let mut current_team = Team::ImmuneSystem;
    let mut id_counter = 1;
    let re = Regex::new(r"(\d+) units each with (\d+) hit points (\([^)]*\) )?with an attack that does (\d+) (\w+) damage at initiative (\d+)").unwrap();

    for line in input.lines() {
        if line.contains("Immune System") {
            current_team = Team::ImmuneSystem;
            id_counter = 1;
        } else if line.contains("Infection") {
            current_team = Team::Infection;
            id_counter = 1;
        } else if let Some(caps) = re.captures(line) {
            let mut weaknesses = HashSet::new();
            let mut immunities = HashSet::new();
            if let Some(m) = caps.get(3) {
                let mods = m.as_str().trim().trim_matches(|p| p == '(' || p == ')');
                for part in mods.split(';') {
                    let part = part.trim();
                    if let Some(w) = part.strip_prefix("weak to ") {
                        w.split(", ").for_each(|s| {
                            weaknesses.insert(s.to_string());
                        });
                    }
                    if let Some(i) = part.strip_prefix("immune to ") {
                        i.split(", ").for_each(|s| {
                            immunities.insert(s.to_string());
                        });
                    }
                }
            }
            groups.push(Group {
                id: id_counter,
                team: current_team.clone(),
                units: caps[1].parse().unwrap(),
                hp: caps[2].parse().unwrap(),
                attack: caps[4].parse().unwrap(),
                attack_type: caps[5].to_string(),
                initiative: caps[6].parse().unwrap(),
                weaknesses,
                immunities,
            });
            id_counter += 1;
        }
    }
    groups
}

fn fight(groups: &mut Vec<Group>) -> (Option<Team>, i32) {
    loop {
        groups.sort_by(|a, b| {
            (b.effective_power(), b.initiative).cmp(&(a.effective_power(), a.initiative))
        });
        let mut targets = HashMap::new();
        let mut targeted = HashSet::new();

        for i in 0..groups.len() {
            let attacker = &groups[i];
            let best_target = (0..groups.len())
                .filter(|&j| groups[j].team != attacker.team && !targeted.contains(&j))
                .max_by(|&j1, &j2| {
                    let d1 = attacker.damage_to(&groups[j1]);
                    let d2 = attacker.damage_to(&groups[j2]);
                    (d1, groups[j1].effective_power(), groups[j1].initiative).cmp(&(
                        d2,
                        groups[j2].effective_power(),
                        groups[j2].initiative,
                    ))
                });

            if let Some(j) = best_target {
                if attacker.damage_to(&groups[j]) > 0 {
                    targets.insert(i, j);
                    targeted.insert(j);
                }
            }
        }

        let mut attackers: Vec<usize> = (0..groups.len()).collect();
        attackers.sort_by_key(|&i| std::cmp::Reverse(groups[i].initiative));

        let mut units_killed = 0;
        for i in attackers {
            if groups[i].units <= 0 {
                continue;
            }
            if let Some(j) = targets.get(&i) {
                let damage = groups[i].damage_to(&groups[*j]);
                let killed = (damage / groups[*j].hp).min(groups[*j].units);
                units_killed += killed;
                groups[*j].units -= killed;
            }
        }

        if units_killed == 0 {
            return (None, 0);
        }

        groups.retain(|g| g.units > 0);
        let immune_left = groups.iter().any(|g| g.team == Team::ImmuneSystem);
        let infection_left = groups.iter().any(|g| g.team == Team::Infection);

        if !immune_left || !infection_left {
            let winner = if immune_left {
                Some(Team::ImmuneSystem)
            } else {
                Some(Team::Infection)
            };
            return (winner, groups.iter().map(|g| g.units).sum());
        }
    }
}

impl Solution for Day24 {
    fn year(&self) -> u32 {
        2018
    }
    fn day(&self) -> u32 {
        24
    }

    fn part1(&self, input: &str) -> Result<String> {
        let mut groups = parse(input);
        let (_, score) = fight(&mut groups);
        Ok((score).to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let initial_groups = parse(input);
        for boost in 1.. {
            let mut groups = initial_groups.clone();
            for g in &mut groups {
                if g.team == Team::ImmuneSystem {
                    g.attack += boost;
                }
            }
            let (winner, score) = fight(&mut groups);
            if winner == Some(Team::ImmuneSystem) {
                return Ok((score).to_string());
            }
        }
        Ok(("No solution found").to_string())
    }
}
