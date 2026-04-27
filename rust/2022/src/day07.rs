use anyhow::Result;
use aoc_rust_common::Solution;

pub struct Day07;

#[derive(Debug)]
enum Command {
    CdIn(String),
    CdOut,
    Ls,
    Dir(String),
    Size(i64),
}

fn parse(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|line| {
            if line.starts_with("$ cd ..") {
                Command::CdOut
            } else if line.starts_with("$ cd ") {
                Command::CdIn(line[5..].to_string())
            } else if line.starts_with("$ ls") {
                Command::Ls
            } else if line.starts_with("dir ") {
                Command::Dir(line[4..].to_string())
            } else {
                let parts: Vec<&str> = line.split_whitespace().collect();
                Command::Size(parts[0].parse().unwrap())
            }
        })
        .collect()
}

fn get_dir_sizes(commands: &[Command]) -> Vec<i64> {
    let mut dir_sizes = Vec::new();
    let mut stack = Vec::new();
    let mut current_size = 0;

    let mut i = 0;
    while i < commands.len() {
        match &commands[i] {
            Command::CdIn(_) => {
                stack.push(current_size);
                current_size = 0;
            }
            Command::CdOut => {
                let parent_size = stack.pop().unwrap();
                dir_sizes.push(current_size);
                current_size += parent_size;
            }
            Command::Size(size) => current_size += size,
            _ => {}
        }
        i += 1;
    }
    dir_sizes.push(current_size);
    dir_sizes
}

impl Solution for Day07 {
    fn year(&self) -> u32 {
        2022
    }
    fn day(&self) -> u32 {
        7
    }

    fn part1(&self, input: &str) -> Result<String> {
        let commands = parse(input);
        let sizes = get_dir_sizes(&commands);
        Ok((sizes.iter().filter(|&&s| s <= 100_000).sum::<i64>()).to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let commands = parse(input);
        let sizes = get_dir_sizes(&commands);
        let total_used = *sizes.iter().max().unwrap();
        let to_delete = total_used - (70_000_000 - 30_000_000);
        Ok((*sizes.iter().filter(|&&s| s >= to_delete).min().unwrap()).to_string())
    }
}

aoc_rust_common::aoc_test!(Day07);
