use crate::utils::intcode::{growing_memory, VM};
use anyhow::Result;
use aoc_rust_common::{Direction, Point, Solution};
use std::collections::HashMap;

pub struct Day11;

impl Solution for Day11 {
    fn year(&self) -> u32 {
        2019
    }
    fn day(&self) -> u32 {
        11
    }

    fn part1(&self, input: &str) -> Result<String> {
        let panels = paint_hull(input, 0)?;
        Ok(panels.len().to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let panels = paint_hull(input, 1)?;
        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;
        let mut min_y = i32::MAX;
        let mut max_y = i32::MIN;
        for p in panels.keys() {
            min_x = min_x.min(p.x);
            max_x = max_x.max(p.x);
            min_y = min_y.min(p.y);
            max_y = max_y.max(p.y);
        }

        let mut output = String::new();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let color = panels.get(&Point::new(x, y)).cloned().unwrap_or(0);
                output.push(if color == 1 { '#' } else { ' ' });
            }
            output.push('\n');
        }
        Ok(output.trim_end().to_string())
    }
}

fn paint_hull(input: &str, start_color: i64) -> Result<HashMap<Point, i64>> {
    let memory: Vec<i64> = input
        .trim()
        .split(',')
        .filter_map(|s| s.parse().ok())
        .collect();
    let mut vm = VM::new(growing_memory(memory));
    let mut panels = HashMap::new();
    let mut pos = Point::new(0, 0);
    let mut dir = Direction::North;

    panels.insert(pos, start_color);

    while vm.state != crate::utils::intcode::State::Halted {
        let color = panels.get(&pos).cloned().unwrap_or(0);
        vm.registers.pending_in = Some(color);
        
        // Run until output (new color)
        while vm.state != crate::utils::intcode::State::Halted && vm.registers.pending_out.is_none() {
            vm.run_all_async(|_| Ok(()))?;
        }
        
        if let Some(new_color) = vm.registers.pending_out.take() {
            panels.insert(pos, new_color);
            
            // Run until next output (turn)
            while vm.state != crate::utils::intcode::State::Halted && vm.registers.pending_out.is_none() {
                vm.run_all_async(|_| Ok(()))?;
            }
            
            if let Some(turn) = vm.registers.pending_out.take() {
                dir = match turn {
                    0 => rotate_left(dir),
                    1 => rotate_right(dir),
                    _ => panic!("Invalid turn"),
                };
                pos += dir.delta();
            }
        }
    }

    Ok(panels)
}

fn rotate_left(dir: Direction) -> Direction {
    match dir {
        Direction::North => Direction::West,
        Direction::West => Direction::South,
        Direction::South => Direction::East,
        Direction::East => Direction::North,
        _ => unreachable!(),
    }
}

fn rotate_right(dir: Direction) -> Direction {
    match dir {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
        _ => unreachable!(),
    }
}

aoc_rust_common::aoc_test!(Day11);
