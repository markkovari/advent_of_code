use aoc_rust_common::Solution;
use std::fmt::Display;
use std::str::FromStr;

pub struct Day01;

#[derive(Debug, Clone, Copy)]
enum Rotation { Left(i32), Right(i32) }

struct Dial { position: i32 }

impl Default for Dial {
    fn default() -> Self { Dial { position: 50 } }
}

impl Dial {
    pub fn rotate(&mut self, rotation: Rotation) -> u16 {
        let old_position = self.position;
        let angle = match rotation {
            Rotation::Left(x) => -x,
            Rotation::Right(x) => x,
        };
        self.position = (self.position + angle).rem_euclid(100);

        let count = if angle >= 0 {
            (old_position + angle).div_euclid(100) - old_position.div_euclid(100)
        } else {
            (old_position - 1).div_euclid(100) - (old_position + angle - 1).div_euclid(100)
        };
        count as u16
    }

    pub fn is_zero(&self) -> bool { self.position == 0 }
}

impl FromStr for Rotation {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir_char, amount) = s.split_at(1);
        let amount = amount.parse::<i32>().map_err(|_| ())?;
        match dir_char {
            "R" => Ok(Rotation::Right(amount)),
            "L" => Ok(Rotation::Left(amount)),
            _ => Err(()),
        }
    }
}

impl Solution for Day01 {
    fn year(&self) -> u32 { 2025 }
    fn day(&self) -> u32 { 1 }

    fn part1(&self, input: &str) -> Box<dyn Display> {
        let mut dial = Dial::default();
        let mut result = 0;
        for line in input.lines() {
            if let Ok(rot) = line.trim().parse::<Rotation>() {
                dial.rotate(rot);
                if dial.is_zero() { result += 1; }
            }
        }
        Box::new(result)
    }

    fn part2(&self, input: &str) -> Box<dyn Display> {
        let mut dial = Dial::default();
        let mut result = 0;
        for line in input.lines() {
            if let Ok(rot) = line.trim().parse::<Rotation>() {
                result += dial.rotate(rot);
            }
        }
        Box::new(result)
    }
}
