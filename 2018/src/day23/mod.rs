use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Read, Write};
use std::ops::Range;
use std::result;
use std::str::{self, FromStr};

use lazy_static::lazy_static;
use rand::distributions::uniform::SampleRange;
use rand::seq::SliceRandom;
use rand::Rng;
use regex::Regex;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = result::Result<T, Box<dyn Error>>;

fn solve(content: String) -> Result<(u64, i64)> {
    let bots: Bots = content.parse()?;

    let largest = bots.largest_radius();
    let in_range = bots.in_range_of_bot(&largest);

    let best = search(&bots);
    let dist = Coordinate::origin().distance(&best);
    Ok((in_range, dist))
}

fn search(bots: &Bots) -> Coordinate {
    const INIT_TEMPERATURE: f64 = 1_000.0;
    const COOLING_FACTOR: f64 = 0.9999;
    const ITERS: usize = 1_000;

    fn prob(iter: usize, in_range_old: u64, in_range_new: u64) -> f64 {
        let temp = COOLING_FACTOR.powi(iter as i32) * INIT_TEMPERATURE;
        ((in_range_new as f64 - in_range_old as f64) / temp).exp()
    }

    let mut rng = rand::thread_rng();
    let mut origins = vec![];
    for bot in bots.bots.iter() {
        for i in 0..10_000 {
            if i % 100 == 0 {
                println!("generating origins: {}/{}", i, 10_000);
            }
            origins.push(bot.random_surface_coordinate(&mut rng));
        }
    }
    origins.shuffle(&mut rng);

    let mut best_in_range = bots.in_range(&origins[0]);
    let mut best: HashSet<Coordinate> = HashSet::new();
    best.insert(origins[0]);

    for (i, &o) in origins.iter().enumerate() {
        let mut cur_pos = o;
        let mut cur_in_range = bots.in_range(&cur_pos);

        for i in 0..ITERS {
            let new_pos = cur_pos.random_neighbor(&mut rng);
            let new_in_range = bots.in_range(&new_pos);
            let p = prob(i, cur_in_range, new_in_range);
            if p >= 1.0 || rng.gen_bool(p) {
                cur_pos = new_pos;
                cur_in_range = new_in_range;
            }
            if new_in_range == best_in_range {
                best.insert(new_pos);
            } else if new_in_range > best_in_range {
                best.clear();
                best.insert(new_pos);
                best_in_range = new_in_range;
            }
        }

        // print out progress
        if i % 100 == 0 {
            let zzz = best
                .iter()
                .cloned()
                .min_by_key(|c| Coordinate::origin().distance(&c))
                .unwrap();
            println!(
                "origin ({}/{}): {:?} => {:?} (in range: {}, dist: {})",
                i,
                origins.len(),
                o,
                zzz,
                best_in_range,
                Coordinate::origin().distance(&zzz),
            );
        }
    }
    best.iter()
        .cloned()
        .min_by_key(|c| Coordinate::origin().distance(&c))
        .unwrap()
}

#[derive(Clone, Debug)]
struct Bots {
    bots: Vec<Bot>,
}

impl Bots {
    fn largest_radius(&self) -> &Bot {
        self.bots.iter().max_by_key(|b| b.radius).unwrap()
    }

    fn in_range_of_bot(&self, bot: &Bot) -> u64 {
        self.bots.iter().filter(|b| bot.in_range_of_bot(b)).count() as u64
    }

    fn in_range(&self, c: &Coordinate) -> u64 {
        self.bots.iter().filter(|b| b.in_range(c)).count() as u64
    }

    fn total_dist(&self, c: &Coordinate) -> i64 {
        self.bots.iter().map(|b| b.pos.distance(c) as i64).sum()
    }
}

impl FromStr for Bots {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Bots> {
        let mut bots: Vec<Bot> = vec![];
        for line in s.lines() {
            let bot = line
                .parse()
                .or_else(|err| err!("failed to parse '{:?}': {}", line, err))?;
            bots.push(bot);
        }
        if bots.is_empty() {
            return err!("found no bots in input");
        }
        Ok(Bots { bots })
    }
}

#[derive(Clone, Debug)]
struct Bot {
    pos: Coordinate,
    radius: i64,
}

impl Bot {
    fn in_range_of_bot(&self, other: &Bot) -> bool {
        self.pos.distance(&other.pos) <= self.radius
    }

    fn in_range(&self, c: &Coordinate) -> bool {
        self.pos.distance(c) <= self.radius
    }

    fn random_surface_coordinate<R: Rng>(&self, mut rng: R) -> Coordinate {
        loop {
            let (x, y, z): (f64, f64, f64) = rng.gen();
            if x == 0.0 && y == 0.0 && z == 0.0 {
                continue;
            }
            let normal = 1.0 / (x * x + y * y + z * z).sqrt();
            let (x, y, z) = (x * normal, y * normal, z * normal);
            let radius = self.radius as f64;
            return Coordinate {
                x: (x * radius) as i32,
                y: (y * radius) as i32,
                z: (z * radius) as i32,
            };
        }
    }
}

impl FromStr for Bot {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Bot> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?x)
                pos=<(?P<x>-?[0-9]+),(?P<y>-?[0-9]+),(?P<z>-?[0-9]+)>,
                \s
                r=(?P<radius>[0-9]+)
            "
            )
            .unwrap();
        }

        let caps = match RE.captures(s) {
            None => return err!("unrecognized position/radius"),
            Some(caps) => caps,
        };
        let pos = Coordinate {
            x: caps["x"].parse()?,
            y: caps["y"].parse()?,
            z: caps["z"].parse()?,
        };
        let radius = caps["radius"].parse()?;
        Ok(Bot { pos, radius })
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
struct Coordinate {
    x: i32,
    y: i32,
    z: i32,
}

impl Coordinate {
    fn origin() -> Coordinate {
        Coordinate { x: 0, y: 0, z: 0 }
    }

    fn distance(&self, other: &Coordinate) -> i64 {
        (self.x as i64 - other.x as i64).abs()
            + (self.y as i64 - other.y as i64).abs()
            + (self.z as i64 - other.z as i64).abs()
    }

    fn random<R: Rng>(mut rng: R) -> Coordinate {
        Coordinate {
            x: rng.gen(),
            y: rng.gen(),
            z: rng.gen(),
        }
    }

    fn random_neighbor<R: Rng>(&self, mut rng: R) -> Coordinate {
        let dx = rng.gen_range(-10_000..=10_000);
        let dy = rng.gen_range(-10_000..=10_000);
        let dz = rng.gen_range(-10_000..=10_000);
        Coordinate {
            x: self.x + dx,
            y: self.y + dy,
            z: self.z + dz,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_part1() {
        let text = include_str!("./prod.data").to_owned();
        let (in_range, dist) = solve(text).unwrap();
        assert_eq!(in_range, 164);
        assert_eq!(dist, 101_529_360);
    }
}
