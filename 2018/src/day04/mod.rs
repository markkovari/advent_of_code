use aoc_rust_common::Solution;
use chrono::{DateTime, NaiveDateTime, Timelike, Utc};
use regex::Regex;
use std::collections::HashMap;
use std::fmt::Display;

pub struct Day04;

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Copy, Clone)]
enum EventType {
    Wake,
    Sleep,
    Shift(usize),
}
#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Copy, Clone)]
struct Event {
    time: DateTime<Utc>,
    event: EventType,
}

fn get_sleeps(input: &str) -> HashMap<usize, [u32; 60]> {
    let scanner = Regex::new(r"^\[([^]]+)] (Guard #(\d+)|falls asleep|wakes up)").unwrap();
    let mut events: Vec<Event> = input
        .lines()
        .map(|line| {
            let captures = scanner.captures(line).unwrap();
            let timestamp = NaiveDateTime::parse_from_str(&captures[1], "%Y-%m-%d %H:%M")
                .unwrap()
                .and_local_timezone(Utc)
                .unwrap();
            let event = match &captures[2] {
                "falls asleep" => EventType::Sleep,
                "wakes up" => EventType::Wake,
                _ => EventType::Shift(captures[3].parse().unwrap()),
            };
            Event {
                time: timestamp,
                event,
            }
        })
        .collect();
    events.sort_unstable();

    let mut sleeps = HashMap::new();
    let mut guard: Option<usize> = None;
    let mut sleep_start: Option<DateTime<Utc>> = None;
    for event in events.iter() {
        match &event.event {
            EventType::Shift(val) => {
                guard = Some(*val);
                sleep_start = None;
            }
            EventType::Sleep => {
                sleep_start = Some(event.time);
            }
            EventType::Wake => {
                let minutes = sleeps.entry(guard.unwrap()).or_insert([0u32; 60]);
                for m in sleep_start.unwrap().minute()..event.time.minute() {
                    minutes[m as usize] += 1;
                }
            }
        }
    }
    sleeps
}

impl Solution for Day04 {
    fn year(&self) -> u32 {
        2018
    }
    fn day(&self) -> u32 {
        4
    }

    fn part1(&self, input: &str) -> Box<dyn Display> {
        let sleepers = get_sleeps(input);
        let scores: HashMap<usize, u32> =
            sleepers.iter().map(|(k, v)| (*k, v.iter().sum())).collect();
        let (best_sleeper, _) = scores.iter().max_by_key(|&(_, v)| v).unwrap();
        let best_minute = sleepers[best_sleeper]
            .iter()
            .enumerate()
            .max_by_key(|&(_, count)| count)
            .unwrap()
            .0;
        Box::new(best_sleeper * best_minute)
    }

    fn part2(&self, input: &str) -> Box<dyn Display> {
        let sleepers = get_sleeps(input);
        let (guard, (minute, _)) = sleepers
            .iter()
            .map(|(guard, minutes)| {
                (
                    guard,
                    minutes
                        .iter()
                        .enumerate()
                        .max_by_key(|&(_, count)| count)
                        .unwrap(),
                )
            })
            .max_by_key(|&(_, (_, count))| count)
            .unwrap();
        Box::new(guard * minute)
    }
}
