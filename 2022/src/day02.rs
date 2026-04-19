use anyhow::Result;
use aoc_rust_common::Solution;

pub struct Day02;

impl Solution for Day02 {
    fn year(&self) -> u32 {
        2022
    }
    fn day(&self) -> u32 {
        2
    }

    fn part1(&self, input: &str) -> Result<String> {
        let score: i32 = input
            .lines()
            .map(|line| {
                let mut parts = line.split_whitespace();
                let opp = parts
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("missing opponent move"))?;
                let me = parts
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("missing my move"))?;
                let score = match me {
                    "X" => 1,
                    "Y" => 2,
                    "Z" => 3,
                    _ => 0,
                };
                let outcome = match (opp, me) {
                    ("A", "X") | ("B", "Y") | ("C", "Z") => 3,
                    ("A", "Y") | ("B", "Z") | ("C", "X") => 6,
                    _ => 0,
                };
                Ok(score + outcome)
            })
            .collect::<Result<Vec<i32>>>()?
            .into_iter()
            .sum();
        Ok((score).to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let score: i32 = input
            .lines()
            .map(|line| {
                let mut parts = line.split_whitespace();
                let opp = parts
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("missing opponent move"))?;
                let outcome_str = parts
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("missing outcome"))?;

                let (me, outcome_score) = match outcome_str {
                    "X" => match opp {
                        "A" => ("Z", 0),
                        "B" => ("X", 0),
                        "C" => ("Y", 0),
                        _ => ("", 0),
                    },
                    "Y" => match opp {
                        "A" => ("X", 3),
                        "B" => ("Y", 3),
                        "C" => ("Z", 3),
                        _ => ("", 3),
                    },
                    "Z" => match opp {
                        "A" => ("Y", 6),
                        "B" => ("Z", 6),
                        "C" => ("X", 6),
                        _ => ("", 6),
                    },
                    _ => ("", 0),
                };
                let choice_score = match me {
                    "X" => 1,
                    "Y" => 2,
                    "Z" => 3,
                    _ => 0,
                };
                Ok(choice_score + outcome_score)
            })
            .collect::<Result<Vec<i32>>>()?
            .into_iter()
            .sum();
        Ok((score).to_string())
    }
}
