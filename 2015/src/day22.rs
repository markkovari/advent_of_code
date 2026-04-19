use anyhow::Result;
use aoc_rust_common::Solution;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp;

pub struct Day22;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    fn cost(&self) -> i32 {
        match self {
            Spell::MagicMissile => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229,
        }
    }
    fn all() -> Vec<Self> {
        vec![
            Spell::MagicMissile,
            Spell::Drain,
            Spell::Shield,
            Spell::Poison,
            Spell::Recharge,
        ]
    }
}

#[derive(Clone)]
struct GameState {
    player_hp: i32,
    boss_hp: i32,
    mana: i32,
    mana_spent: i32,
    shield_timer: i32,
    poison_timer: i32,
    recharge_timer: i32,
    hard_mode: bool,
}

fn find_min_mana(start_state: GameState) -> i32 {
    let mut min_mana = i32::MAX;
    let mut games = vec![start_state];

    while let Some(mut state) = games.pop() {
        if state.mana_spent >= min_mana {
            continue;
        }

        // Player's turn
        if state.hard_mode {
            state.player_hp -= 1;
            if state.player_hp <= 0 {
                continue;
            }
        }

        // Effects
        if state.shield_timer > 0 {
            state.shield_timer -= 1;
        }
        if state.poison_timer > 0 {
            state.boss_hp -= 3;
            state.poison_timer -= 1;
        }
        if state.recharge_timer > 0 {
            state.mana += 101;
            state.recharge_timer -= 1;
        }
        if state.boss_hp <= 0 {
            min_mana = cmp::min(min_mana, state.mana_spent);
            continue;
        }

        for spell in Spell::all() {
            let cost = spell.cost();
            if state.mana < cost {
                continue;
            }
            if (spell == Spell::Shield && state.shield_timer > 0)
                || (spell == Spell::Poison && state.poison_timer > 0)
                || (spell == Spell::Recharge && state.recharge_timer > 0)
            {
                continue;
            }

            let mut next_state = state.clone();
            next_state.mana -= cost;
            next_state.mana_spent += cost;

            match spell {
                Spell::MagicMissile => next_state.boss_hp -= 4,
                Spell::Drain => {
                    next_state.boss_hp -= 2;
                    next_state.player_hp += 2;
                }
                Spell::Shield => next_state.shield_timer = 6,
                Spell::Poison => next_state.poison_timer = 6,
                Spell::Recharge => next_state.recharge_timer = 5,
            }

            if next_state.boss_hp <= 0 {
                min_mana = cmp::min(min_mana, next_state.mana_spent);
                continue;
            }

            // Boss's turn
            // Effects
            let player_armor = if next_state.shield_timer > 0 { 7 } else { 0 };
            if next_state.poison_timer > 0 {
                next_state.boss_hp -= 3;
                next_state.poison_timer -= 1;
            }
            if next_state.recharge_timer > 0 {
                next_state.mana += 101;
                next_state.recharge_timer -= 1;
            }
            if next_state.shield_timer > 0 {
                next_state.shield_timer -= 1;
            }

            if next_state.boss_hp <= 0 {
                min_mana = cmp::min(min_mana, next_state.mana_spent);
                continue;
            }

            lazy_static! {
                static ref RE: Regex = Regex::new(
                    r"Hit Points: \d+
Damage: (\d+)"
                )
                .unwrap();
            }
            let boss_damage = RE.captures("").map_or(8, |c| c[1].parse().unwrap_or(8));
            next_state.player_hp -= cmp::max(1, boss_damage - player_armor);

            if next_state.player_hp > 0 {
                games.push(next_state);
            }
        }
    }
    min_mana
}

impl Solution for Day22 {
    fn year(&self) -> u32 {
        2015
    }
    fn day(&self) -> u32 {
        22
    }

    fn part1(&self, input: &str) -> Result<String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"Hit Points: (\d+)").unwrap();
        }
        let boss_hp = RE
            .captures(input)
            .map_or(58, |c| c[1].parse().unwrap_or(58));
        let start_state = GameState {
            player_hp: 50,
            boss_hp,
            mana: 500,
            mana_spent: 0,
            shield_timer: 0,
            poison_timer: 0,
            recharge_timer: 0,
            hard_mode: false,
        };
        Ok((find_min_mana(start_state)).to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"Hit Points: (\d+)").unwrap();
        }
        let boss_hp = RE
            .captures(input)
            .map_or(58, |c| c[1].parse().unwrap_or(58));
        let start_state = GameState {
            player_hp: 50,
            boss_hp,
            mana: 500,
            mana_spent: 0,
            shield_timer: 0,
            poison_timer: 0,
            recharge_timer: 0,
            hard_mode: true,
        };
        Ok((find_min_mana(start_state)).to_string())
    }
}

aoc_rust_common::aoc_test!(Day22);
