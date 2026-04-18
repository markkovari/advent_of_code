use aoc_rust_common::Solution;
use std::fmt::Display;
use std::str::FromStr;

pub struct Day04;

#[derive(Debug, Clone)]
struct Table {
    rows: Vec<Vec<(u8, bool)>>,
}

impl FromStr for Table {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|val| (val.parse::<u8>().unwrap(), false))
                    .collect::<Vec<_>>()
            })
            .collect();
        Ok(Table { rows })
    }
}

impl Table {
    fn mark(&mut self, number: u8) {
        for row in self.rows.iter_mut() {
            for cell in row.iter_mut() {
                if cell.0 == number {
                    cell.1 = true;
                }
            }
        }
    }

    fn check_win(&self) -> bool {
        self.rows.iter().any(|row| row.iter().all(|c| c.1))
            || (0..self.rows[0].len()).any(|col| self.rows.iter().all(|row| row[col].1))
    }

    fn score(&self, last_number: u8) -> u64 {
        let sum: u64 = self
            .rows
            .iter()
            .flatten()
            .filter(|c| !c.1)
            .map(|c| c.0 as u64)
            .sum();
        sum * last_number as u64
    }
}

impl Solution for Day04 {
    fn year(&self) -> u32 {
        2021
    }
    fn day(&self) -> u32 {
        4
    }

    fn part1(&self, input: &str) -> Box<dyn Display> {
        let mut sections = input.split("\n\n");
        let numbers: Vec<u8> = sections
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        let mut tables: Vec<Table> = sections.map(|s| s.parse().unwrap()).collect();

        for num in numbers {
            for table in &mut tables {
                table.mark(num);
                if table.check_win() {
                    return Box::new(table.score(num));
                }
            }
        }
        Box::new(0)
    }

    fn part2(&self, input: &str) -> Box<dyn Display> {
        let mut sections = input.split("\n\n");
        let numbers: Vec<u8> = sections
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        let mut tables: Vec<Table> = sections.map(|s| s.parse().unwrap()).collect();
        let mut won = vec![false; tables.len()];

        for num in numbers {
            for (i, table) in tables.iter_mut().enumerate() {
                if won[i] {
                    continue;
                }
                table.mark(num);
                if table.check_win() {
                    won[i] = true;
                    if won.iter().all(|&w| w) {
                        return Box::new(table.score(num));
                    }
                }
            }
        }
        Box::new(0)
    }
}
