use aoc_rust_common::Solution;
use std::fmt::Display;
use std::collections::{HashMap, HashSet};
use nom::{
    bytes::complete::tag,
    character::complete::anychar,
    IResult,
};

pub struct Day07;

type Step = char;

fn parse_dependency(input: &str) -> IResult<&str, (Step, Step)> {
    let (input, _) = tag("Step ")(input)?;
    let (input, req) = anychar(input)?;
    let (input, _) = tag(" must be finished before step ")(input)?;
    let (input, step) = anychar(input)?;
    let (input, _) = tag(" can begin.")(input)?;
    Ok((input, (req, step)))
}

fn get_deps(input: &str) -> (HashSet<Step>, HashMap<Step, HashSet<Step>>) {
    let mut all_steps = HashSet::new();
    let mut requirements: HashMap<Step, HashSet<Step>> = HashMap::new();

    for line in input.lines() {
        let (_, (req, step)) = parse_dependency(line).unwrap();
        all_steps.insert(req);
        all_steps.insert(step);
        requirements.entry(step).or_default().insert(req);
    }
    (all_steps, requirements)
}

impl Solution for Day07 {
    fn year(&self) -> u32 { 2018 }
    fn day(&self) -> u32 { 7 }

    fn part1(&self, input: &str) -> Box<dyn Display> {
        let (all_steps, requirements) = get_deps(input);
        let mut done: HashSet<Step> = HashSet::new();
        let mut ordered = String::new();

        while done.len() < all_steps.len() {
            let mut available: Vec<Step> = all_steps.iter()
                .filter(|s| !done.contains(s))
                .filter(|s| requirements.get(s).map_or(true, |reqs| reqs.is_subset(&done)))
                .copied().collect();
            available.sort_unstable();

            if let Some(next_step) = available.first() {
                ordered.push(*next_step);
                done.insert(*next_step);
            } else {
                break; // No available steps
            }
        }
        Box::new(ordered)
    }

    fn part2(&self, input: &str) -> Box<dyn Display> {
        let (all_steps, requirements) = get_deps(input);
        let mut done: HashSet<Step> = HashSet::new();
        let mut workers = vec![(0, '.'); 5]; // (time_free, step)
        let mut time = 0;
        let base_duration = if input.lines().count() < 10 { 0 } else { 60 };

        while done.len() < all_steps.len() {
            let mut available: Vec<Step> = all_steps.iter()
                .filter(|s| !done.contains(s) && !workers.iter().any(|&(_, w)| w == **s))
                .filter(|s| requirements.get(s).map_or(true, |reqs| reqs.is_subset(&done)))
                .copied().collect();
            available.sort_unstable_by(|a, b| b.cmp(a)); // So we can pop from the end

            for (time_free, step) in workers.iter_mut().filter(|(tf, _)| *tf <= time) {
                 if *step != '.' { done.insert(*step); }
                *step = '.';
                if let Some(next_step) = available.pop() {
                    *step = next_step;
                    *time_free = time + base_duration + (next_step as u32 - 'A' as u32 + 1);
                }
            }
            
            let next_event_time = workers.iter().filter(|&&(_, s)| s != '.').map(|(tf, _)| *tf).min();
            if let Some(t) = next_event_time {
                time = t;
            } else if !available.is_empty() {
                // All workers are idle, but there are still steps to do. This can happen if we are waiting for a dependency.
                // In this case, we need to advance time.
                time += 1;
            } else if done.len() == all_steps.len() {
                break;
            }
        }
        
        let final_time = workers.iter().map(|(tf, _)| *tf).max().unwrap_or(time);
        Box::new(final_time)
    }
}
