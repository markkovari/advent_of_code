use anyhow::Result;
use aoc_rust_common::Solution;
use sscanf::sscanf;
use std::collections::HashMap;

pub struct Day16;

#[derive(Debug, Clone)]
struct Valve<'a> {
    name: &'a str,
    flow: u16,
    tunnels: Vec<&'a str>,
}

impl Solution for Day16 {
    fn year(&self) -> u32 {
        2022
    }
    fn day(&self) -> u32 {
        16
    }

    fn part1(&self, input: &str) -> Result<String> {
        let valves = parse(input);
        let dists = compute_dists(&valves);
        let non_zero = valves
            .iter()
            .filter(|v| v.flow > 0)
            .map(|v| v.name)
            .collect::<Vec<_>>();
        let start = "AA";

        let mut memo = HashMap::new();
        Ok(max_pressure(start, 30, 0, &non_zero, &dists, &valves, &mut memo).to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let valves = parse(input);
        let dists = compute_dists(&valves);
        let non_zero = valves
            .iter()
            .filter(|v| v.flow > 0)
            .map(|v| v.name)
            .collect::<Vec<_>>();
        let start = "AA";

        let mut memo = HashMap::new();
        Ok(max_pressure_two(
            start, start, 26, 26, 0, &non_zero, &dists, &valves, &mut memo,
        )
        .to_string())
    }
}

fn parse(input: &str) -> Vec<Valve<'_>> {
    input
        .lines()
        .map(|line| {
            let (name, flow, _, tunnels) = sscanf!(
                line,
                "Valve {str} has flow rate={u16}; {str:/tunnels? leads? to valves?/} {str}"
            )
            .unwrap();
            Valve {
                name,
                flow,
                tunnels: tunnels.split(", ").collect(),
            }
        })
        .collect()
}

fn compute_dists<'a>(valves: &'a [Valve<'a>]) -> HashMap<(&'a str, &'a str), u16> {
    let mut dists = HashMap::new();
    for v in valves {
        for t in &v.tunnels {
            dists.insert((v.name, *t), 1);
        }
        dists.insert((v.name, v.name), 0);
    }
    for k in valves.iter().map(|v| v.name) {
        for i in valves.iter().map(|v| v.name) {
            for j in valves.iter().map(|v| v.name) {
                let d_ij = *dists.get(&(i, j)).unwrap_or(&1000);
                let d_ik = *dists.get(&(i, k)).unwrap_or(&1000);
                let d_kj = *dists.get(&(k, j)).unwrap_or(&1000);
                if d_ik + d_kj < d_ij {
                    dists.insert((i, j), d_ik + d_kj);
                }
            }
        }
    }
    dists
}

fn max_pressure<'a>(
    curr: &'a str,
    time: u16,
    opened: u32,
    non_zero: &[&'a str],
    dists: &HashMap<(&'a str, &'a str), u16>,
    valves: &[Valve<'a>],
    memo: &mut HashMap<(&'a str, u16, u32), u16>,
) -> u16 {
    if time == 0 {
        return 0;
    }
    let key = (curr, time, opened);
    if let Some(&res) = memo.get(&key) {
        return res;
    }

    let mut res = 0;
    for (i, &next) in non_zero.iter().enumerate() {
        if (opened & (1 << i)) == 0 {
            if let Some(&dist) = dists.get(&(curr, next)) {
                if time > dist + 1 {
                    let flow = valves.iter().find(|v| v.name == next).unwrap().flow;
                    res = res.max(
                        flow * (time - dist - 1)
                            + max_pressure(
                                next,
                                time - dist - 1,
                                opened | (1 << i),
                                non_zero,
                                dists,
                                valves,
                                memo,
                            ),
                    );
                }
            }
        }
    }
    memo.insert(key, res);
    res
}

fn max_pressure_two<'a>(
    curr1: &'a str,
    curr2: &'a str,
    time1: u16,
    time2: u16,
    opened: u32,
    non_zero: &[&'a str],
    dists: &HashMap<(&'a str, &'a str), u16>,
    valves: &[Valve<'a>],
    memo: &mut HashMap<(&'a str, &'a str, u16, u16, u32), u16>,
) -> u16 {
    let key = (curr1, curr2, time1, time2, opened);
    if let Some(&res) = memo.get(&key) {
        return res;
    }

    let mut res = 0;
    for (i, &next) in non_zero.iter().enumerate() {
        if (opened & (1 << i)) == 0 {
            if let Some(&dist1) = dists.get(&(curr1, next)) {
                if time1 > dist1 + 1 {
                    let flow = valves.iter().find(|v| v.name == next).unwrap().flow;
                    res = res.max(
                        flow * (time1 - dist1 - 1)
                            + max_pressure_two(
                                next,
                                curr2,
                                time1 - dist1 - 1,
                                time2,
                                opened | (1 << i),
                                non_zero,
                                dists,
                                valves,
                                memo,
                            ),
                    );
                }
            }
            if let Some(&dist2) = dists.get(&(curr2, next)) {
                if time2 > dist2 + 1 {
                    let flow = valves.iter().find(|v| v.name == next).unwrap().flow;
                    res = res.max(
                        flow * (time2 - dist2 - 1)
                            + max_pressure_two(
                                curr1,
                                next,
                                time1,
                                time2 - dist2 - 1,
                                opened | (1 << i),
                                non_zero,
                                dists,
                                valves,
                                memo,
                            ),
                    );
                }
            }
        }
    }
    memo.insert(key, res);
    res
}

aoc_rust_common::aoc_test!(Day16);
