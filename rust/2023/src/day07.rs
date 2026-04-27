use anyhow::Result;
use aoc_rust_common::Solution;
use std::collections::HashMap;

pub struct Day07;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Rank {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    cards: Vec<u8>, // stored as values
    bid: i64,
}

fn get_rank(cards: &[u8]) -> Rank {
    let mut counts = HashMap::new();
    for &c in cards {
        *counts.entry(c).or_insert(0) += 1;
    }
    let mut v: Vec<i32> = counts.values().cloned().collect();
    v.sort_unstable();
    match v.as_slice() {
        [5] => Rank::FiveOfAKind,
        [1, 4] => Rank::FourOfAKind,
        [2, 3] => Rank::FullHouse,
        [1, 1, 3] => Rank::ThreeOfAKind,
        [1, 2, 2] => Rank::TwoPairs,
        [1, 1, 1, 2] => Rank::OnePair,
        _ => Rank::HighCard,
    }
}

fn get_joker_rank(cards: &[u8]) -> Rank {
    let mut counts = HashMap::new();
    let mut jokers = 0;
    for &c in cards {
        if c == 1 {
            // We represent Joker as 1 for Part 2 comparison
            jokers += 1;
        } else {
            *counts.entry(c).or_insert(0) += 1;
        }
    }

    if jokers == 5 {
        return Rank::FiveOfAKind;
    }

    let mut v: Vec<i32> = counts.values().cloned().collect();
    v.sort_unstable();
    // Add jokers to the highest count
    if let Some(last) = v.last_mut() {
        *last += jokers;
    } else {
        v.push(jokers);
    }

    match v.as_slice() {
        [5] => Rank::FiveOfAKind,
        [1, 4] => Rank::FourOfAKind,
        [2, 3] => Rank::FullHouse,
        [1, 1, 3] => Rank::ThreeOfAKind,
        [1, 2, 2] => Rank::TwoPairs,
        [1, 1, 1, 2] => Rank::OnePair,
        _ => Rank::HighCard,
    }
}

fn card_value(c: char, part2: bool) -> u8 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => {
            if part2 {
                1
            } else {
                11
            }
        }
        'T' => 10,
        _ => c.to_digit(10).unwrap() as u8,
    }
}

impl Solution for Day07 {
    fn year(&self) -> u32 {
        2023
    }
    fn day(&self) -> u32 {
        7
    }

    fn part1(&self, input: &str) -> Result<String> {
        let mut hands: Vec<Hand> = input
            .lines()
            .map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                let cards: Vec<u8> = parts[0].chars().map(|c| card_value(c, false)).collect();
                let bid = parts[1].parse().unwrap();
                Hand { cards, bid }
            })
            .collect();

        hands.sort_by(|a, b| {
            let rank_a = get_rank(&a.cards);
            let rank_b = get_rank(&b.cards);
            if rank_a != rank_b {
                rank_a.cmp(&rank_b)
            } else {
                a.cards.cmp(&b.cards)
            }
        });

        let total: i64 = hands
            .iter()
            .enumerate()
            .map(|(i, h)| h.bid * (i as i64 + 1))
            .sum();
        Ok(total.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let mut hands: Vec<Hand> = input
            .lines()
            .map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                let cards: Vec<u8> = parts[0].chars().map(|c| card_value(c, true)).collect();
                let bid = parts[1].parse().unwrap();
                Hand { cards, bid }
            })
            .collect();

        hands.sort_by(|a, b| {
            let rank_a = get_joker_rank(&a.cards);
            let rank_b = get_joker_rank(&b.cards);
            if rank_a != rank_b {
                rank_a.cmp(&rank_b)
            } else {
                a.cards.cmp(&b.cards)
            }
        });

        let total: i64 = hands
            .iter()
            .enumerate()
            .map(|(i, h)| h.bid * (i as i64 + 1))
            .sum();
        Ok(total.to_string())
    }
}

aoc_rust_common::aoc_test!(Day07, "250602641", "251037509");
