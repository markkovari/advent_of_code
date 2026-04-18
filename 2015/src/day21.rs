use aoc_rust_common::Solution;
use iter_tools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp;
use std::fmt::Display;

pub struct Day21;

#[derive(Debug, Eq, PartialEq)]
enum ItemType {
    Weapon,
    Armor,
    Ring,
}

#[derive(Debug, Eq, PartialEq)]
struct Item {
    t: ItemType,
    cost: i32,
    damage: i32,
    armor: i32,
}

#[rustfmt::skip]
fn all_items() -> Vec<Item> {
    vec![
        Item { t: ItemType::Weapon, cost: 8, damage: 4, armor: 0 }, Item { t: ItemType::Weapon, cost: 10, damage: 5, armor: 0 },
        Item { t: ItemType::Weapon, cost: 25, damage: 6, armor: 0 }, Item { t: ItemType::Weapon, cost: 40, damage: 7, armor: 0 },
        Item { t: ItemType::Weapon, cost: 74, damage: 8, armor: 0 }, Item { t: ItemType::Armor, cost: 13, damage: 0, armor: 1 },
        Item { t: ItemType::Armor, cost: 31, damage: 0, armor: 2 }, Item { t: ItemType::Armor, cost: 53, damage: 0, armor: 3 },
        Item { t: ItemType::Armor, cost: 75, damage: 0, armor: 4 }, Item { t: ItemType::Armor, cost: 102, damage: 0, armor: 5 },
        Item { t: ItemType::Ring, cost: 25, damage: 1, armor: 0 }, Item { t: ItemType::Ring, cost: 50, damage: 2, armor: 0 },
        Item { t: ItemType::Ring, cost: 100, damage: 3, armor: 0 }, Item { t: ItemType::Ring, cost: 20, damage: 0, armor: 1 },
        Item { t: ItemType::Ring, cost: 40, damage: 0, armor: 2 }, Item { t: ItemType::Ring, cost: 80, damage: 0, armor: 3 },
    ]
}

fn is_valid_item_combination(items: &[&Item]) -> bool {
    let weapons = items.iter().filter(|i| i.t == ItemType::Weapon).count();
    let armors = items.iter().filter(|i| i.t == ItemType::Armor).count();
    let rings = items.iter().filter(|i| i.t == ItemType::Ring).count();
    weapons == 1 && armors <= 1 && rings <= 2
}

fn all_valid_item_combinations(items: &[Item]) -> impl Iterator<Item = Vec<&Item>> {
    (1..=items.len())
        .flat_map(move |n| items.iter().combinations(n))
        .filter(|c| is_valid_item_combination(c))
}

struct Fighter {
    hit_points: i32,
    damage: i32,
    armor: i32,
}

impl Fighter {
    fn new(hit_points: i32, damage: i32, armor: i32) -> Self {
        Self {
            hit_points,
            damage,
            armor,
        }
    }

    fn parse(specs: &str) -> Fighter {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"Hit Points: (?P<hps>\d+)
Damage: (?P<damage>\d+)
Armor: (?P<armor>\d+)"
            )
            .unwrap();
        }
        let captures = RE.captures(specs).unwrap();
        Fighter::new(
            captures.name("hps").unwrap().as_str().parse().unwrap(),
            captures.name("damage").unwrap().as_str().parse().unwrap(),
            captures.name("armor").unwrap().as_str().parse().unwrap(),
        )
    }

    fn fight(&self, other: &Self, items: &[&Item]) -> bool {
        let total_damage = self.damage + items.iter().map(|i| i.damage).sum::<i32>();
        let total_armor = self.armor + items.iter().map(|i| i.armor).sum::<i32>();
        let damage_to_other = cmp::max(1, total_damage - other.armor);
        let damage_to_self = cmp::max(1, other.damage - total_armor);
        let turns_to_win = (other.hit_points as f32 / damage_to_other as f32).ceil() as i32;
        let turns_to_lose = (self.hit_points as f32 / damage_to_self as f32).ceil() as i32;
        turns_to_win <= turns_to_lose
    }
}

impl Solution for Day21 {
    fn year(&self) -> u32 {
        2015
    }
    fn day(&self) -> u32 {
        21
    }

    fn part1(&self, input: &str) -> Box<dyn Display> {
        let player = Fighter::new(100, 0, 0);
        let boss = Fighter::parse(input);
        let items = all_items();
        Box::new(
            all_valid_item_combinations(&items)
                .filter(|i| player.fight(&boss, i))
                .map(|i| i.iter().map(|item| item.cost).sum::<i32>())
                .min()
                .unwrap_or(0),
        )
    }

    fn part2(&self, input: &str) -> Box<dyn Display> {
        let player = Fighter::new(100, 0, 0);
        let boss = Fighter::parse(input);
        let items = all_items();
        Box::new(
            all_valid_item_combinations(&items)
                .filter(|i| !player.fight(&boss, i))
                .map(|i| i.iter().map(|item| item.cost).sum::<i32>())
                .max()
                .unwrap_or(0),
        )
    }
}
