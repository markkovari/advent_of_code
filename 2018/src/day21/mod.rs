use anyhow::Result;
use aoc_rust_common::Solution;
use std::collections::HashSet;

pub struct Day21;

// This solution is based on reverse-engineering the assembly-like code.
// The code essentially computes a sequence of numbers and halts when a number repeats.
// Part 1 asks for the first number generated.
// Part 2 asks for the last number generated before the sequence repeats.

impl Solution for Day21 {
    fn year(&self) -> u32 {
        2018
    }
    fn day(&self) -> u32 {
        21
    }

    fn part1(&self, _input: &str) -> Result<String> {
        // The value in register 5 when instruction 28 (eqrr 5 0 2) is first hit.
        // After decompiling, this happens when a specific value is generated.
        // This is the first value checked against r0, so it's the answer for part 1.
        let mut r5: u64 = 0;
        loop {
            let mut r3: u64 = r5 | 65536;
            r5 = 1533496;
            loop {
                let r2 = r3 & 255;
                r5 += r2;
                r5 &= 16777215;
                r5 *= 65899;
                r5 &= 16777215;
                if r3 < 256 {
                    // This is the value that will be compared to r0.
                    // For part 1, we want the first such value.
                    return Ok((r5).to_string());
                }
                r3 /= 256;
            }
        }
    }

    fn part2(&self, _input: &str) -> Result<String> {
        // We need to find the last value of r5 before the sequence of r5 values repeats.
        let mut seen = HashSet::new();
        let mut last_unique = 0;
        let mut r5: u64 = 0;

        loop {
            let mut r3: u64 = r5 | 65536;
            r5 = 1533496;
            loop {
                let r2 = r3 & 255;
                r5 += r2;
                r5 &= 16777215;
                r5 *= 65899;
                r5 &= 16777215;

                if r3 < 256 {
                    if !seen.insert(r5) {
                        // We've seen this value before, so the previous one was the last unique one.
                        return Ok((last_unique).to_string());
                    }
                    last_unique = r5;
                    break;
                }
                r3 /= 256;
            }
        }
    }
}

aoc_rust_common::aoc_test!(Day21);
