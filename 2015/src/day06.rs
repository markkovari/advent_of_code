use aoc_rust_common::Solution;
use rayon::prelude::*;
use std::fmt::Display;

pub struct Day06;

type Grid = [[bool; 1000]; 1000];
type GridAmbient = [[i8; 1000]; 1000];

fn apply_on_grid(instruction: Instruction, grid: &mut Grid) {
    for x in instruction.start.0..=instruction.end.0 {
        for y in instruction.start.1..=instruction.end.1 {
            match instruction.action {
                Action::On => grid[x as usize][y as usize] = true,
                Action::Off => grid[x as usize][y as usize] = false,
                Action::Toggle => grid[x as usize][y as usize] = !grid[x as usize][y as usize],
            }
        }
    }
}

fn apply_on_ambient_grid(instruction: Instruction, grid: &mut GridAmbient) {
    for x in instruction.start.0..=instruction.end.0 {
        for y in instruction.start.1..=instruction.end.1 {
            grid[x as usize][y as usize] += match (instruction.action, grid[x as usize][y as usize])
            {
                (Action::On, _) => 1,
                (Action::Off, 0) => 0,
                (Action::Off, _) => -1,
                (_, _) => 2,
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Action {
    On,
    Off,
    Toggle,
}

impl TryFrom<&str> for Action {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.starts_with("turn on") {
            Ok(Action::On)
        } else if value.starts_with("turn off") {
            Ok(Action::Off)
        } else if value.starts_with("toggle") {
            Ok(Action::Toggle)
        } else {
            Err(String::from("Invalid action"))
        }
    }
}

struct Instruction {
    action: Action,
    start: Point,
    end: Point,
}

impl TryFrom<&str> for Instruction {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut split = value.split_whitespace().collect::<Vec<&str>>();
        let end: Point = Point::try_from(split.pop().ok_or("Missing end point")?)?;
        split.pop();
        let start: Point = Point::try_from(split.pop().ok_or("Missing start point")?)?;
        let action: Action = Action::try_from(split.join(" ").as_str())?;
        Ok(Instruction { action, start, end })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Point(i32, i32);

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point(x, y)
    }
}

impl TryFrom<&str> for Point {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut split = value.split(',');
        let x = split
            .next()
            .ok_or("Missing x")?
            .parse::<i32>()
            .map_err(|e| e.to_string())?;
        let y = split
            .next()
            .ok_or("Missing y")?
            .parse::<i32>()
            .map_err(|e| e.to_string())?;
        Ok(Point::new(x, y))
    }
}

impl Solution for Day06 {
    fn year(&self) -> u32 {
        2015
    }
    fn day(&self) -> u32 {
        6
    }

    fn part1(&self, input: &str) -> Box<dyn Display> {
        let instructions = input
            .lines()
            .map(|line| Instruction::try_from(line).unwrap())
            .collect::<Vec<Instruction>>();
        let mut grid: Grid = [[false; 1000]; 1000];
        for instruction in instructions {
            apply_on_grid(instruction, &mut grid);
        }
        Box::new(
            grid.par_iter()
                .flat_map(|row| row.par_iter())
                .filter(|&&cell| cell)
                .count() as i64,
        )
    }

    fn part2(&self, input: &str) -> Box<dyn Display> {
        let instructions = input
            .lines()
            .map(|line| Instruction::try_from(line).unwrap())
            .collect::<Vec<Instruction>>();
        let mut grid: GridAmbient = [[0; 1000]; 1000];
        for instruction in instructions {
            apply_on_ambient_grid(instruction, &mut grid);
        }
        Box::new(
            grid.par_iter()
                .flat_map(|row| row.par_iter())
                .map(|&cell| cell as i64)
                .sum::<i64>(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day06() {
        let day = Day06;
        assert_eq!(
            day.part1("turn on 0,0 through 999,999").to_string(),
            "1000000"
        );
        assert_eq!(day.part1("toggle 0,0 through 999,0").to_string(), "1000");
        assert_eq!(
            day.part1("turn off 499,499 through 500,500").to_string(),
            "0"
        );

        assert_eq!(day.part2("turn on 0,0 through 0,0").to_string(), "1");
        assert_eq!(
            day.part2("toggle 0,0 through 999,999").to_string(),
            "2000000"
        );
    }
}
