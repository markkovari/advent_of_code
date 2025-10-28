use lazy_static::lazy_static;
use regex::Regex;
use std::{
    cmp,
    collections::{BTreeSet, HashMap, HashSet},
    fmt::Debug,
    ops::AddAssign,
    vec,
};

use iter_tools::Itertools;

use crate::{Excercise, Solvable};

struct TwentyFirstDay {
    exercise: Excercise,
}

#[derive(Debug, Eq, PartialEq)]
enum ItemType {
    Weapon,
    Armor,
    Ring,
}

#[rustfmt::skip]
fn all_items() -> Vec<Item> {
    vec![
        Item { t: ItemType::Weapon, cost: 8, damage: 4, armor: 0 },
        Item { t: ItemType::Weapon, cost: 10, damage: 5, armor: 0 },
        Item { t: ItemType::Weapon, cost: 25, damage: 6, armor: 0 },
        Item { t: ItemType::Weapon, cost: 40, damage: 7, armor: 0 },
        Item { t: ItemType::Weapon, cost: 74, damage: 8, armor: 0 },
        Item { t: ItemType::Armor, cost: 13, damage: 0, armor: 1 },
        Item { t: ItemType::Armor, cost: 31, damage: 0, armor: 2 },
        Item { t: ItemType::Armor, cost: 53, damage: 0, armor: 3 },
        Item { t: ItemType::Armor, cost: 75, damage: 0, armor: 4 },
        Item { t: ItemType::Armor, cost: 102, damage: 0, armor: 5 },
        Item { t: ItemType::Ring, cost: 25, damage:  1, armor: 0 },
        Item { t: ItemType::Ring, cost: 50, damage:  2, armor: 0 },
        Item { t: ItemType::Ring, cost: 100, damage:  3, armor: 0 },
        Item { t: ItemType::Ring, cost: 20, damage:  0, armor: 1 },
        Item { t: ItemType::Ring, cost: 40, damage:  0, armor: 2 },
        Item { t: ItemType::Ring, cost: 80, damage:  0, armor: 3 },
    ]
}

fn is_valid_item_combination(items: &[&Item]) -> bool {
    let items_of_type =
        |items: &[&Item], t: ItemType| items.iter().filter(|item| item.t == t).count();

    items_of_type(items, ItemType::Weapon) == 1
        && items_of_type(items, ItemType::Armor) <= 1
        && items_of_type(items, ItemType::Ring) <= 2
}

fn all_valid_item_combinations(items: &[Item]) -> impl Iterator<Item = Vec<&Item>> {
    (1..items.len())
        .flat_map(move |length| items.iter().combinations(length))
        .filter(|items| is_valid_item_combination(items))
}

fn total_cost_items(items: Vec<&Item>) -> i32 {
    items.iter().map(|item| item.cost).sum()
}

#[derive(Debug, Eq, PartialEq)]
struct Fighter {
    hit_points: i32,
    damage: i32,
    armor: i32,
}

#[derive(Debug, Eq, PartialEq)]
struct Item {
    t: ItemType,
    cost: i32,
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
                r"Hit Points: (?P<hps>\d+)\nDamage: (?P<damage>\d+)\nArmor: (?P<armor>\d+)"
            )
            .unwrap();
        }
        let captures = RE.captures(specs).unwrap();

        let hit_points = captures.name("hps").unwrap().as_str().parse().unwrap();
        let damage = captures.name("damage").unwrap().as_str().parse().unwrap();
        let armor = captures.name("armor").unwrap().as_str().parse().unwrap();

        Self::new(hit_points, damage, armor)
    }

    fn simulate_fight(&self, other: &Self, equipped_items: &[&Item]) -> bool {
        let total_damage = self.damage + equipped_items.iter().map(|&i| i.damage).sum::<i32>();
        let total_armor = self.armor + equipped_items.iter().map(|&i| i.armor).sum::<i32>();

        let damage_done = cmp::max(total_damage - other.armor, 1);
        let turns_till_kill = (other.hit_points as f32 / damage_done as f32).ceil() as i32;

        let damage_done = cmp::max(other.damage - total_armor, 1);
        let turns_till_dead = (self.hit_points as f32 / damage_done as f32).ceil() as i32;

        turns_till_kill <= turns_till_dead
    }
}

impl TwentyFirstDay {
    fn solve_first(&self, input: &str) -> usize {
        self.first(&input)
    }

    fn solve_second(&self, input: &str) -> usize {
        self.second(&input)
    }

    fn first(&self, input: &str) -> usize {
        let player = Fighter::new(100, 0, 0);
        let boss = Fighter::parse(input);
        let items = all_items();

        all_valid_item_combinations(&items)
            .filter(|items| player.simulate_fight(&boss, items))
            .map(total_cost_items)
            .min()
            .unwrap() as usize
    }

    fn second(&self, input: &str) -> usize {
        let player = Fighter::new(100, 0, 0);
        let boss = Fighter::parse(input);
        let items = all_items();

        all_valid_item_combinations(&items)
            .filter(|items| !player.simulate_fight(&boss, items))
            .map(total_cost_items)
            .max()
            .unwrap() as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("21_test.txt");
    const PROD: &str = include_str!("21_prod.txt");

    #[test]
    fn first_test() {
        let mut first_excersise = TwentyFirstDay {
            exercise: Excercise {
                content: String::from(PROD),
                example: String::from(EXAMPLE),
            },
        };

        let expected_example = 78;
        let expected_prod = 78;
        let result_example = first_excersise.first(PROD);
        let result_prod = first_excersise.first(PROD);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);

        let expected_example = 148;
        let expected_prod = 148;
        let result_example = first_excersise.second(PROD);
        let result_prod = first_excersise.second(PROD);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);
    }
}
