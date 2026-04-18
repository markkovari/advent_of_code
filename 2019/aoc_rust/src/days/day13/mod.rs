use crate::utils::Computer;
use aoc_rust_common::Solution;
use std::collections::HashMap;
use std::fmt::Display;

pub struct Day13;

impl Solution for Day13 {
    fn year(&self) -> u32 {
        2019
    }
    fn day(&self) -> u32 {
        13
    }

    fn part1(&self, input: &str) -> Box<dyn Display> {
        let mut computer = Computer::from_string(input);
        computer.run();
        let mut map = HashMap::new();
        while let Ok(x) = computer.o.try_recv() {
            let y = computer.o.try_recv().unwrap();
            let b = computer.o.try_recv().unwrap();
            map.insert((x, y), b);
        }
        Box::new(map.values().filter(|&&b| b == 2).count())
    }

    fn part2(&self, input: &str) -> Box<dyn Display> {
        let mut computer = Computer::from_string(input);
        computer.p[0] = 2;
        let (mut pad, mut ball, mut score) = (0, 0, 0);

        while {
            let done = !computer.run();
            while let Ok(x) = computer.o.try_recv() {
                let y = computer.o.try_recv().unwrap();
                let b = computer.o.try_recv().unwrap();
                match (x, y, b) {
                    (-1, 0, _) => score = b,
                    (_, _, 3) => pad = x,
                    (_, _, 4) => ball = x,
                    _ => {}
                }
            }
            computer.i.send((ball - pad).signum()).unwrap_or_default();
            done
        } {}
        Box::new(score)
    }
}
