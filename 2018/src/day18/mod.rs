use core::fmt;
use std::error::Error;

use std::{result, str::FromStr};

use itertools::Itertools;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = result::Result<T, Box<dyn Error>>;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coordinate {
    x: i64,
    y: i64,
}

#[derive(Clone, Debug)]
struct Area {
    acres: Vec<Vec<Acre>>,
    acres2: Vec<Vec<Acre>>,
}

impl Area {
    fn resource_value(&self) -> usize {
        let (mut wooded, mut lumber) = (0, 0);
        for row in &self.acres {
            for acre in row {
                match acre {
                    Acre::Open => {}
                    Acre::Trees => wooded += 1,
                    Acre::Lumberyard => lumber += 1,
                }
            }
        }
        wooded * lumber
    }
    fn step(&mut self) {
        let mut new = std::mem::take(&mut self.acres2);
        for y in 0..self.height() {
            for x in 0..self.width() {
                self.step_cell(x, y, &mut new);
            }
        }
        self.acres2 = std::mem::take(&mut self.acres);
        self.acres = new;
    }

    fn step_cell(&self, x: usize, y: usize, new: &mut Vec<Vec<Acre>>) {
        use self::Acre::*;

        new[y][x] = self.acres[y][x];
        match self.acres[y][x] {
            Open => {
                let count = self.neighbors(
                    x,
                    y,
                    0,
                    |count, n| {
                        if n == Trees {
                            count + 1
                        } else {
                            count
                        }
                    },
                );
                if count >= 3 {
                    new[y][x] = Trees;
                }
            }
            Trees => {
                let count =
                    self.neighbors(
                        x,
                        y,
                        0,
                        |count, n| {
                            if n == Lumberyard {
                                count + 1
                            } else {
                                count
                            }
                        },
                    );
                if count >= 3 {
                    new[y][x] = Lumberyard;
                }
            }
            Lumberyard => {
                let (has_lumber, has_trees) =
                    self.neighbors(x, y, (false, false), |(lumber, trees), n| {
                        (lumber || n == Lumberyard, trees || n == Trees)
                    });
                if !has_lumber || !has_trees {
                    new[y][x] = Open;
                }
            }
        }
    }

    fn neighbors<T>(&self, ox: usize, oy: usize, init: T, mut f: impl FnMut(T, Acre) -> T) -> T {
        let mut ret = init;
        for y in oy.saturating_sub(1)..=oy.saturating_add(1) {
            for x in ox.saturating_sub(1)..=ox.saturating_add(1) {
                if x == ox && y == oy {
                    continue;
                }
                if x >= self.width() || y >= self.height() {
                    continue;
                }
                ret = f(ret, self.acres[y][x]);
            }
        }
        ret
    }

    fn width(&self) -> usize {
        self.acres[0].len()
    }

    fn height(&self) -> usize {
        self.acres.len()
    }
}

impl FromStr for Area {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Area> {
        if !s.is_ascii() {
            return err!("area must be in ASCII");
        }

        let ylen = s.lines().count();
        if ylen == 0 {
            return err!("area cannot be empty");
        }

        let xlen = s.lines().next().unwrap().len();
        let mut area = Area {
            acres: vec![vec![Acre::Open; xlen]; ylen],
            acres2: vec![vec![Acre::Open; xlen]; ylen],
        };
        for (y, line) in s.lines().enumerate() {
            if line.len() != xlen {
                return err!(
                    "all rows expected to have length {}, but found {}",
                    xlen,
                    line.len()
                );
            }
            for x in 0..line.len() {
                area.acres[y][x] = line[x..x + 1].parse()?;
            }
        }
        Ok(area)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Acre {
    Open,
    Trees,
    Lumberyard,
}

impl FromStr for Acre {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Acre> {
        match s.chars().next() {
            None => err!("cannot parse acre from empty string"),
            Some('.') => Ok(Acre::Open),
            Some('|') => Ok(Acre::Trees),
            Some('#') => Ok(Acre::Lumberyard),
            Some(c) => err!("invalid acre: '{}'", c),
        }
    }
}

impl fmt::Display for Area {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.acres {
            for col in row {
                write!(f, "{}", col)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl fmt::Display for Acre {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Acre::Open => write!(f, "."),
            Acre::Trees => write!(f, "|"),
            Acre::Lumberyard => write!(f, "#"),
        }
    }
}

fn part1(content: String) -> usize {
    let minutes = 10;
    let mut area: Area = content.parse().unwrap();
    for _ in 0..minutes {
        area.step();
    }
    area.resource_value()
}

fn part2(content: String) -> usize {
    let minutes = 1028;
    let mut area: Area = content.parse().unwrap();
    for _ in 0..minutes {
        area.step();
    }
    area.resource_value()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_part1() {
        let text = include_str!("./example.data").to_owned();
        assert_eq!(part1(text), 1147);
        let text = include_str!("./prod.data").to_owned();
        assert_eq!(part1(text), 535522);
    }
    #[test]
    #[ignore]
    fn test_part2() {
        let text = include_str!("./example.data").to_owned();
        assert_eq!(part2(text), 0);
        let text = include_str!("./prod.data").to_owned();
        assert_eq!(part2(text), 210160);
    }
}
