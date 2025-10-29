use lazy_static::lazy_static;
use regex::Regex;
use std::{cmp, vec};



use crate::Exercise;

struct TwentySecondDay {
    exercise: Exercise,
}

struct PlayerSpecs {
    hit_points: i32,
    mana: i32,
}

impl PlayerSpecs {
    fn new(hit_points: i32, mana: i32) -> Self {
        Self { hit_points, mana }
    }
}

struct BossSpecs {
    hit_points: i32,
    damage: i32,
}

impl BossSpecs {
    fn parse(input: &str) -> Self {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"Hit Points: (?P<hps>\d+)\nDamage: (?P<damage>\d+)").unwrap();
        }
        let captures = RE.captures(input).unwrap();

        let hit_points = captures.name("hps").unwrap().as_str().parse().unwrap();
        let damage = captures.name("damage").unwrap().as_str().parse().unwrap();

        Self { hit_points, damage }
    }
}

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
struct Game {
    player: PlayerState,
    boss: BossState,
    shield_effect: TurnLimitedEffect,
    poison_effect: TurnLimitedEffect,
    recharge_effect: TurnLimitedEffect,
    hard_mode: bool,
}

#[derive(Clone)]
struct PlayerState {
    hit_points: i32,
    mana: i32,
    mana_spent: i32,
}

#[derive(Clone)]
struct BossState {
    hit_points: i32,
    damage: i32,
}

#[derive(Copy, Clone)]
struct TurnLimitedEffect(u8);

impl TurnLimitedEffect {
    fn new() -> Self {
        Self(0)
    }

    fn is_active(&self) -> bool {
        self.0 > 0
    }

    fn update(&mut self) {
        if self.0 > 0 {
            self.0 -= 1;
        }
    }

    fn reload(&mut self, turns: u8) {
        self.0 = turns;
    }
}

enum FightResult {
    Inconclusive,
    IllegalMove,
    OutOfMana,
    Dead,
    Win,
}

impl Game {
    fn init(player_spec: &PlayerSpecs, boss_spec: &BossSpecs, hard_mode: bool) -> Self {
        Self {
            shield_effect: TurnLimitedEffect::new(),
            poison_effect: TurnLimitedEffect::new(),
            recharge_effect: TurnLimitedEffect::new(),
            player: PlayerState {
                hit_points: player_spec.hit_points,
                mana: player_spec.mana,
                mana_spent: 0,
            },
            boss: BossState {
                hit_points: boss_spec.hit_points,
                damage: boss_spec.damage,
            },
            hard_mode,
        }
    }

    fn check_for_dead(&self) -> Option<FightResult> {
        if self.player.hit_points <= 0 {
            Some(FightResult::Dead)
        } else if self.boss.hit_points <= 0 {
            Some(FightResult::Win)
        } else {
            None
        }
    }

    fn update(&mut self) -> Option<FightResult> {
        if self.poison_effect.is_active() {
            self.boss.hit_points -= 3;
        }
        if self.recharge_effect.is_active() {
            self.player.mana += 101;
        }

        self.shield_effect.update();
        self.poison_effect.update();
        self.recharge_effect.update();

        self.check_for_dead()
    }

    fn play_turn(&mut self, spell: &Spell) -> FightResult {
        if let Some(result) = self.update() {
            return result;
        }

        if self.hard_mode {
            self.player.hit_points -= 1;
        }

        if let Some(result) = self.check_for_dead() {
            return result;
        }

        let cost = spell.cost();
        if cost > self.player.mana {
            return FightResult::OutOfMana;
        }
        self.player.mana -= cost;
        self.player.mana_spent += cost;

        match spell {
            Spell::MagicMissile => {
                self.boss.hit_points -= 4;
            }
            Spell::Drain => {
                self.player.hit_points += 2;
                self.boss.hit_points -= 2;
            }
            Spell::Shield => {
                if self.shield_effect.is_active() {
                    return FightResult::IllegalMove;
                }
                self.shield_effect.reload(6);
            }
            Spell::Poison => {
                if self.poison_effect.is_active() {
                    return FightResult::IllegalMove;
                }
                self.poison_effect.reload(6);
            }
            Spell::Recharge => {
                if self.recharge_effect.is_active() {
                    return FightResult::IllegalMove;
                }
                self.recharge_effect.reload(5);
            }
        }

        if let Some(result) = self.update() {
            return result;
        }

        let armor = if self.shield_effect.is_active() { 7 } else { 0 };
        self.player.hit_points -= cmp::max(self.boss.damage - armor, 1);

        if let Some(result) = self.check_for_dead() {
            return result;
        }

        FightResult::Inconclusive
    }
}

fn find_least_mana_to_win(game: Game) -> i32 {
    let mut inconclusive_games = vec![game];
    let mut least_mana = i32::MAX;

    loop {
        let mut new_inconclusive_games = Vec::new();

        let mut games_won = 0;
        let mut games_lost = 0;
        let mut games_inconclusive = 0;
        let mut games_too_expensive = 0;

        for game in inconclusive_games {
            for spell in Spell::all() {
                let mut game = game.clone();
                let result = game.play_turn(&spell);

                match result {
                    FightResult::Inconclusive => {
                        if game.player.mana_spent > least_mana {
                            games_too_expensive += 1;
                        } else {
                            new_inconclusive_games.push(game);
                            games_inconclusive += 1;
                        }
                    }
                    FightResult::Win => {
                        least_mana = cmp::min(game.player.mana_spent, least_mana);
                        games_won += 1;
                    }
                    FightResult::IllegalMove | FightResult::OutOfMana | FightResult::Dead => {
                        games_lost += 1;
                    }
                }
            }
        }

        inconclusive_games = new_inconclusive_games;

        if inconclusive_games.is_empty() {
            break;
        }

        println!(
            "wins {:5} | losses {:5} | inconclusive {:5} | too expensive {:5} | current min: {}",
            games_won, games_lost, games_inconclusive, games_too_expensive, least_mana
        );
    }

    least_mana
}

impl TwentySecondDay {
    fn solve_first(&self, input: &str) -> usize {
        self.first(input)
    }

    fn solve_second(&self, input: &str) -> usize {
        self.second(input)
    }

    fn first(&self, input: &str) -> usize {
        let player = PlayerSpecs::new(50, 500);
        let boss = BossSpecs::parse(input);

        find_least_mana_to_win(Game::init(&player, &boss, false)) as usize
    }

    fn second(&self, input: &str) -> usize {
        let player = PlayerSpecs::new(50, 500);
        let boss = BossSpecs::parse(input);

        find_least_mana_to_win(Game::init(&player, &boss, true)) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("inputs/22_test.txt");
    const PROD: &str = include_str!("inputs/22_prod.txt");

    #[test]
    fn first_test() {
        let mut first_exercise = TwentySecondDay {
            exercise: Exercise {
                content: String::from(PROD),
                example: String::from(EXAMPLE),
            },
        };

        let expected_example = 953;
        let expected_prod = 953;
        let result_example = first_exercise.first(PROD);
        let result_prod = first_exercise.first(PROD);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);

        let expected_example = 1289;
        let expected_prod = 1289;
        let result_example = first_exercise.second(PROD);
        let result_prod = first_exercise.second(PROD);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);
    }
}
