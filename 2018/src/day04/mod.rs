use std::collections::HashMap;

use chrono::offset::TimeZone;
use chrono::offset::Utc;
use chrono::DateTime;
use chrono::Timelike;
use regex::Regex;

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

#[derive(Default)]
pub struct Day04 {
    events: Vec<Event>,
}

fn read_events(input: String) -> Vec<Event> {
    let scanner = Regex::new(r"^\[([^]]+)] (Guard #(\d+)|falls asleep|wakes up)").unwrap();

    let mut events = Vec::new();
    for line in input.lines() {
        let captures = scanner.captures(&line).unwrap();
        let timestamp = Utc
            .datetime_from_str(&captures[1], "%Y-%m-%d %H:%M")
            .unwrap();

        let event = match &captures[2] {
            "falls asleep" => EventType::Sleep,
            "wakes up" => EventType::Wake,
            _ => EventType::Shift(captures[3].parse().unwrap()),
        };

        events.push(Event {
            time: timestamp,
            event,
        });
    }
    events.sort_unstable();
    events
}
fn format_results(sleepers: &HashMap<usize, [u32; 60]>, scores: &HashMap<usize, u32>) -> usize {
    let (best_sleeper, _) = scores.iter().max_by(|&(_, a), &(_, b)| a.cmp(b)).unwrap();

    let best_minute = sleepers[best_sleeper]
        .iter()
        .enumerate()
        .max_by(|&(_, a), &(_, b)| a.cmp(b))
        .unwrap()
        .0;

    best_sleeper * (best_minute as usize)
}

fn get_sleeps(events: Vec<Event>) -> HashMap<usize, [u32; 60]> {
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
fn part1(input: String) -> (HashMap<usize, [u32; 60]>, HashMap<usize, u32>) {
    let events = read_events(input);
    let sleepers = get_sleeps(events);
    let scores: HashMap<usize, u32> = sleepers.iter().map(|(k, v)| (*k, v.iter().sum())).collect();

    (sleepers, scores)
}

fn part2(input: String) -> (HashMap<usize, [u32; 60]>, HashMap<usize, u32>) {
    let events = read_events(input);
    let sleepers = get_sleeps(events);
    let scores: HashMap<usize, u32> = sleepers
        .iter()
        .map(|(k, v)| (*k, *v.iter().max().unwrap()))
        .collect();

    (sleepers, scores)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        let example = include_str!("./example.data");
        let (sleepers, scores) = part1(example.to_owned());
        let result = format_results(&sleepers, &scores);
        assert_eq!(240, result);
        let prod = include_str!("./prod.data");
        let (sleepers, scores) = part1(prod.to_owned());
        let result = format_results(&sleepers, &scores);
        assert_eq!(76357, result);
    }
    #[test]
    fn test_second() {
        let example = include_str!("./example.data");
        let (sleepers, scores) = part2(example.to_owned());
        let result = format_results(&sleepers, &scores);
        assert_eq!(4455, result);
        let prod = include_str!("./prod.data");
        let (sleepers, scores) = part2(prod.to_owned());
        let result = format_results(&sleepers, &scores);
        assert_eq!(41668, result);
    }
}
