use std::error::Error;
use std::i32;
use std::io::{self, Read, Write};
use std::result;
use std::str::{self, FromStr};

use nom::Err;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = result::Result<T, Box<dyn Error>>;

fn solve(content: String) -> Result<usize> {
    let mut points: Vec<Point> = vec![];
    for line in content.lines() {
        let point = line
            .parse()
            .or_else(|err| err!("failed to parse '{:?}': {}", line, err))?;
        points.push(point);
    }

    Ok(part1(&points).or_else(|err| err!("part 1: {}", err))?)
}

fn part1(points: &[Point]) -> Result<usize> {
    let mut consts = Constellations::shatter_all(points);
    while consts.step() {}
    Ok(consts.groups.len())
}

#[derive(Clone, Debug)]
struct Constellations {
    groups: Vec<Constellation>,
}

impl Constellations {
    fn shatter_all(points: &[Point]) -> Constellations {
        let mut groups = vec![];
        for &p in points {
            groups.push(Constellation { points: vec![p] });
        }
        Constellations { groups }
    }

    fn step(&mut self) -> bool {
        for i in 0..self.groups.len() {
            for j in i + 1..self.groups.len() {
                if self.groups[i].is_connected(&self.groups[j]) {
                    self.merge(i, j);
                    return true;
                }
            }
        }
        false
    }

    fn merge(&mut self, i1: usize, i2: usize) {
        let g2 = self.groups.swap_remove(i2);
        self.groups[i1].join(&g2);
    }
}

#[derive(Clone, Debug)]
struct Constellation {
    points: Vec<Point>,
}

impl Constellation {
    fn join(&mut self, other: &Constellation) {
        self.points.extend(other.points.iter().cloned());
    }

    fn is_connected(&self, other: &Constellation) -> bool {
        for p in other.points.iter() {
            if self.is_point_connected(p) {
                return true;
            }
        }
        false
    }

    fn is_point_connected(&self, point: &Point) -> bool {
        for p in self.points.iter() {
            if point.distance(p) <= 3 {
                return true;
            }
        }
        false
    }
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
    t: i32,
}

impl Point {
    fn distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs()
            + (self.y - other.y).abs()
            + (self.z - other.z).abs()
            + (self.t - other.t).abs()
    }
}

impl FromStr for Point {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Point> {
        let parts: Vec<&str> = s.trim().split(",").collect();
        if parts.len() != 4 {
            return err!("unrecognized point '{:?}'", s);
        }
        Ok(Point {
            x: parts[0].parse()?,
            y: parts[1].parse()?,
            z: parts[2].parse()?,
            t: parts[3].parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // #[ignore]
    fn test_part1() {
        let text = include_str!("./prod.data").to_owned();
        assert_eq!(solve(text).unwrap(), (327));
    }
}
