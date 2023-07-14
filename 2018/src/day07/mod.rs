use std::collections::{HashMap, HashSet};

use serde_scan::scan;

#[derive(Clone, Copy, Debug)]
struct Dependency {
    step: Step,
    required: Step,
}

type RequiredFor = HashMap<Step, HashSet<Step>>;

type Step = char;

fn read_deps(content: String) -> Vec<Dependency> {
    let mut rules = Vec::new();
    for l in content.lines() {
        let rule: (char, char) =
            scan!("Step {} must be finished before step {} can begin." <- l).unwrap();
        rules.push(Dependency {
            step: rule.1,
            required: rule.0,
        });
    }
    rules
}

fn part1(deps: Vec<Dependency>) -> Option<String> {
    let mut required_for: RequiredFor = HashMap::new();
    for dep in deps {
        required_for
            .entry(dep.step)
            .or_default()
            .insert(dep.required);
        required_for.entry(dep.required).or_default();
    }

    let mut taken: HashSet<Step> = HashSet::new();
    let mut order: Vec<Step> = vec![];
    let mut next: Vec<Step> = vec![];
    loop {
        find_next_steps(&required_for, &taken, &taken, &mut next);
        let next_step = match next.pop() {
            None => break,
            Some(next_step) => next_step,
        };
        taken.insert(next_step);
        order.push(next_step);
    }

    let answer: String = order.iter().cloned().collect();
    Some(answer)
}

fn part2(deps: Vec<Dependency>) -> (String, usize) {
    let mut required_for: RequiredFor = HashMap::new();
    for dep in deps {
        required_for
            .entry(dep.step)
            .or_default()
            .insert(dep.required);
        required_for.entry(dep.required).or_default();
    }
    let mut workers = Workers::new(5);
    let mut assigned: HashSet<Step> = HashSet::new();
    let mut done: HashSet<Step> = HashSet::new();
    let mut order: Vec<Step> = vec![];
    let mut next: Vec<Step> = vec![];

    let mut seconds = 0;
    loop {
        workers.run_one_step(&mut order, &mut done);

        find_next_steps(&required_for, &assigned, &done, &mut next);
        if next.is_empty() && workers.all_idle() {
            break;
        }
        for worker in workers.available() {
            let next_step = match next.pop() {
                None => break,
                Some(next_step) => next_step,
            };
            assigned.insert(next_step);
            workers.work_on(worker, next_step);
        }
        seconds += 1;
        println!("S: {:?} W: {:?}", seconds, workers);
    }
    let answer: String = order.iter().cloned().collect();
    (answer, seconds)
}

#[derive(Debug)]
struct Workers {
    status: Vec<Status>,
}

type WorkerID = usize;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Status {
    Idle,
    Working { step: Step, remaining: u32 },
}

impl Workers {
    fn new(count: usize) -> Workers {
        Workers {
            status: vec![Status::Idle; count],
        }
    }

    fn available(&self) -> Vec<WorkerID> {
        let mut available = vec![];
        for (worker, &status) in self.status.iter().enumerate() {
            if status == Status::Idle {
                available.push(worker);
            }
        }
        available
    }

    fn all_idle(&self) -> bool {
        self.status.iter().all(|s| *s == Status::Idle)
    }

    fn work_on(&mut self, worker: WorkerID, step: Step) {
        let status = &mut self.status[worker];
        assert!(
            *status == Status::Idle,
            "worker {} is not available",
            worker
        );

        let remaining = (step as u32) - b'A' as u32 + 1 + 60;
        *status = Status::Working { step, remaining }
    }

    fn run_one_step(&mut self, order: &mut Vec<Step>, done: &mut HashSet<Step>) {
        for worker in 0..self.status.len() {
            let mut is_done = false;
            match self.status[worker] {
                Status::Idle => {}
                Status::Working {
                    step,
                    ref mut remaining,
                } => {
                    *remaining -= 1;
                    if *remaining == 0 {
                        is_done = true;
                        order.push(step);
                        done.insert(step);
                    }
                }
            }
            if is_done {
                self.status[worker] = Status::Idle;
            }
        }
    }
}

fn find_next_steps(
    required_for: &RequiredFor,
    taken: &HashSet<Step>,
    done: &HashSet<Step>,
    next_stack: &mut Vec<Step>,
) {
    for (&step, dependencies) in required_for {
        if taken.contains(&step) {
            continue;
        }
        if dependencies.iter().all(|s| done.contains(s)) {
            next_stack.push(step);
        }
    }
    next_stack.sort();
    next_stack.dedup();
    next_stack.reverse();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_first() {
        let example = include_str!("./example.data");
        let deps = read_deps(example.to_owned());
        let result = part1(deps);
        assert_eq!("CABDFE", result.unwrap());
        let prod = include_str!("./prod.data");
        let deps = read_deps(prod.to_owned());
        let result = part1(deps);
        assert_eq!("BFKEGNOVATIHXYZRMCJDLSUPWQ", result.unwrap());
    }

    #[test]
    #[ignore]
    fn test_second() {
        let example = include_str!("./example.data");
        let deps = read_deps(example.to_owned());
        let (result_string, seconds) = part2(deps);
        assert_eq!(253, seconds);
        assert_eq!("CAFBDE", result_string);
        let example = include_str!("./prod.data");
        let deps = read_deps(example.to_owned());
        let (result_string, seconds) = part2(deps);
        assert_eq!(1020, seconds);
        assert_eq!("BFKVEGAOTNYIHXZRMCJLDSUPWQ", result_string);
    }
}
