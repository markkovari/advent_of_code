use aoc_rust_common::Solution;
use std::fmt::Display;
use std::collections::VecDeque;

pub struct Day19;

#[derive(Clone, Copy, Debug)]
struct State {
    inventory: [u16; 4],
    bots: [u16; 4],
    elapsed: u16,
}

fn max_geodes(blueprint: &[[u16; 4]; 4], max_time: u16) -> u16 {
    let mut max_robots = [u16::MAX; 4];
    for i in 0..3 {
        max_robots[i] = blueprint.iter().map(|cost| cost[i]).max().unwrap();
    }
    let mut max_geodes = 0;

    let mut q = VecDeque::new();
    q.push_back(State {
        inventory: [0, 0, 0, 0],
        bots: [1, 0, 0, 0],
        elapsed: 0,
    });

    while let Some(State { inventory, bots, elapsed }) = q.pop_front() {
        for i in 0..blueprint.len() {
            if bots[i] == max_robots[i] { continue; }

            let costs = &blueprint[i];
            let wait_time = (0..3)
                .map(|idx| {
                    match costs[idx] {
                        cost if cost <= inventory[idx] => 0,
                        _ if bots[idx] == 0 => max_time + 1,
                        cost => (cost - inventory[idx] + bots[idx] - 1) / bots[idx],
                    }
                })
                .max()
                .unwrap();

            let new_elapsed = elapsed + wait_time + 1;
            if new_elapsed >= max_time { continue; }

            let mut new_inventory = [0; 4];
            for idx in 0..bots.len() {
                new_inventory[idx] = inventory[idx] + bots[idx] * (wait_time + 1) - costs[idx];
            }

            let mut new_bots = bots;
            new_bots[i] += 1;

            let remaining_time = max_time - new_elapsed;
            if ((remaining_time - 1) * remaining_time) / 2 + new_inventory[3] + remaining_time * new_bots[3] < max_geodes {
                continue;
            }

            q.push_back(State { inventory: new_inventory, bots: new_bots, elapsed: new_elapsed })
        }
        max_geodes = max_geodes.max(inventory[3] + bots[3] * (max_time - elapsed));
    }
    max_geodes
}

fn parse(input: &str) -> Vec<[[u16; 4]; 4]> {
    input.lines().map(|line| {
        let mut iter = line.split_ascii_whitespace();
        let ore_bot_costs = [iter.nth(6).unwrap().parse().unwrap(), 0, 0, 0];
        let clay_bot_costs = [iter.nth(5).unwrap().parse().unwrap(), 0, 0, 0];
        let obsidian_bot_costs = [iter.nth(5).unwrap().parse().unwrap(), iter.nth(2).unwrap().parse().unwrap(), 0, 0];
        let geode_bot_costs = [iter.nth(5).unwrap().parse().unwrap(), 0, iter.nth(2).unwrap().parse().unwrap(), 0];
        [ore_bot_costs, clay_bot_costs, obsidian_bot_costs, geode_bot_costs]
    }).collect()
}

impl Solution for Day19 {
    fn year(&self) -> u32 { 2022 }
    fn day(&self) -> u32 { 19 }

    fn part1(&self, input: &str) -> Box<dyn Display> {
        let blueprints = parse(input);
        Box::new(blueprints.iter().enumerate().map(|(idx, blueprint)| (idx + 1) as u16 * max_geodes(blueprint, 24)).sum::<u16>() as i64)
    }

    fn part2(&self, input: &str) -> Box<dyn Display> {
        let blueprints = parse(input);
        Box::new(blueprints.iter().take(3).map(|blueprint| max_geodes(blueprint, 32) as u64).product::<u64>() as i64)
    }
}
