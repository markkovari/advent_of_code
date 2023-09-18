use std::cmp::Reverse;
use std::error::Error;
use std::str::FromStr;

use lazy_static::*;
use regex::Regex;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Clone, Eq, Debug, PartialEq, Copy)]
enum Team {
    Immune,
    Infect,
}

#[derive(Clone, Eq, Debug, PartialEq, Copy)]
struct Unit {
    team: Team,
    group: Group,
}

#[derive(Clone, Eq, Debug, PartialEq, Copy)]
struct Group {
    units: usize,
    hp: usize,
    dmg: usize,
    mults: [u8; 5],
    dmg_typ: DamageType,
    init: usize,
}

impl Group {
    /// damage self deals to other
    fn damage_to(&self, other: &Group) -> usize {
        self.units * self.dmg * other.mults[self.dmg_typ as usize] as usize
    }
}
#[derive(Clone, Copy, Eq, Debug, PartialEq, Hash)]
#[repr(u8)]
enum DamageType {
    Slashing = 0,
    Cold = 1,
    Bludgeoning = 2,
    Radiation = 3,
    Fire = 4,
}

impl FromStr for DamageType {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<DamageType> {
        Ok(match s {
            "slashing" => DamageType::Slashing,
            "cold" => DamageType::Cold,
            "bludgeoning" => DamageType::Bludgeoning,
            "radiation" => DamageType::Radiation,
            "fire" => DamageType::Fire,
            _ => return Err(format!("invalid type: {:?}", s).into()),
        })
    }
}

fn battle(mut units: Vec<Unit>) -> (Option<Team>, usize) {
    loop {
        units.sort_by_key(|v| Reverse((v.units * v.dmg, v.init)));
        let mut targets: Vec<Option<usize>> = vec![None; units.len()];
        for (j, u) in units.iter().enumerate() {
            let mut best = 0;
            for (i, v) in units.iter().enumerate() {
                if u.team == v.team || targets.contains(&Some(i)) || v.units == 0 {
                    continue;
                }
                if u.damage_to(&v) > best {
                    best = u.damage_to(&v);
                    targets[j] = Some(i);
                };
            }
        }
        let mut attackers = (0..units.len()).collect::<Vec<_>>();
        attackers.sort_by_key(|&idx| Reverse(units[idx].init));
        let mut any_die = false;
        for atk_idx in attackers {
            if units[atk_idx].units == 0 {
                continue;
            }
            if let Some(j) = targets[atk_idx] {
                let atk = units[atk_idx];
                let mut def = units[j];
                let dmg = atk.damage_to(&def);
                def.units = def.units.saturating_sub(dmg / def.hp);
                any_die = any_die || dmg > def.hp;
                units[j] = def;
            }
        }

        if !any_die {
            return (None, 0);
        }

        let alive = units.iter().fold((0, 0), |mut teams, group| {
            if group.team == Team::Immune {
                teams.0 += group.units;
            } else {
                teams.1 += group.units;
            }
            teams
        });
        if alive == (0, 0) {
            return (None, 0);
        } else if alive.0 == 0 {
            return (Some(Team::Infect), alive.1);
        } else if alive.1 == 0 {
            return (Some(Team::Immune), alive.0);
        }
    }
}

static INPUT: &str = "data/day24";

fn solve(content: String) -> Result<(usize, usize)> {
    let mut team = Team::Immune;
    let mut units = Vec::new();
    for line in content.lines() {
        if line.starts_with("Immune System:") {
            team = Team::Immune;
        } else if line.starts_with("Infection:") {
            team = Team::Infect;
        } else if !line.trim().is_empty() {
            let group = line.parse()?;
            units.push(Unit { team, group });
        }
    }
    let (_, p1) = battle(units.clone());
    let p2 = (1..)
        .filter_map(|b| {
            let mut units = units.clone();
            units
                .iter_mut()
                .filter(|u| u.team == Team::Immune)
                .for_each(|u| u.dmg += b);
            match battle(units) {
                (Some(Team::Immune), rem) => Some(rem),
                _ => None,
            }
        })
        .next()
        .unwrap();
    Ok((p1, p2))
}

impl std::ops::Deref for Unit {
    type Target = Group;
    fn deref(&self) -> &Group {
        &self.group
    }
}

impl std::ops::DerefMut for Unit {
    fn deref_mut(&mut self) -> &mut Group {
        &mut self.group
    }
}

impl FromStr for Group {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref UNHP: Regex =
                Regex::new(r"^(\d+) units each with (\d+) hit points").unwrap();
            static ref DMIN: Regex =
                Regex::new(r"with an attack that does (\d+) (\w+) damage at initiative (\d+)$")
                    .unwrap();
        }
        let wk = s
            .trim_matches(|c| !(c == ')' || c == '('))
            .trim_matches(|c| c == ')' || c == '(');
        let caps = UNHP
            .captures(s)
            .ok_or(format!("no UNHP match for input: {:?}", s))?;
        let units = caps
            .get(1)
            .ok_or(format!("no units in input: {:?}", s))?
            .as_str()
            .parse()?;
        let hp = caps
            .get(2)
            .ok_or(format!("no hp in input: {:?}", s))?
            .as_str()
            .parse()?;
        let caps = DMIN
            .captures(s)
            .ok_or(format!("no DMIN match for input: {:?}", s))?;
        let dmg = caps
            .get(1)
            .ok_or(format!("no dmg in input: {:?}", s))?
            .as_str()
            .parse()?;
        let dmg_typ = caps
            .get(2)
            .ok_or(format!("no dmg type in input: {:?}", s))?
            .as_str()
            .parse()?;
        let init = caps
            .get(3)
            .ok_or(format!("no initative in input: {:?}", s))?
            .as_str()
            .parse()?;
        let mut mults = [1; 5];

        for w in wk.split(';') {
            let w = w.trim();
            if w.starts_with("weak to ") {
                let w = w.trim_start_matches("weak to ");
                for d in w.split(", ") {
                    mults[d.parse::<DamageType>()? as usize] = 2;
                }
            } else if w.starts_with("immune to ") {
                let w = w.trim_start_matches("immune to ");
                for d in w.split(", ") {
                    mults[d.parse::<DamageType>()? as usize] = 0;
                }
            }
        }
        Ok(Self {
            units,
            hp,
            dmg,
            dmg_typ,
            mults,
            init,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_part1() {
        let text = include_str!("./prod.data").to_owned();
        assert_eq!(solve(text).unwrap(), (18532, 6523));
    }
}
