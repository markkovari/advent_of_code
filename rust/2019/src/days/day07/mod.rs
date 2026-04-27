use crate::utils::intcode::{growing_memory, VM};
use anyhow::Result;
use aoc_rust_common::Solution;
use itertools::Itertools;

pub struct Day07;

impl Solution for Day07 {
    fn year(&self) -> u32 {
        2019
    }
    fn day(&self) -> u32 {
        7
    }

    fn part1(&self, input: &str) -> Result<String> {
        let memory: Vec<i64> = input
            .trim()
            .split(',')
            .filter_map(|s| s.parse().ok())
            .collect();
        let mut max_signal = 0;
        for phases in (0..5).permutations(5) {
            let mut signal = 0;
            for &phase in &phases {
                let mut vm = VM::new(growing_memory(memory.clone()));
                let mut inputs = vec![phase, signal].into_iter();
                let mut output = 0;
                vm.run_all(|| Ok(inputs.next().unwrap()), |v| {
                    output = v;
                    Ok(())
                })?;
                signal = output;
            }
            max_signal = max_signal.max(signal);
        }
        Ok(max_signal.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let memory: Vec<i64> = input
            .trim()
            .split(',')
            .filter_map(|s| s.parse().ok())
            .collect();
        let mut max_signal = 0;
        for phases in (5..10).permutations(5) {
            let mut vms: Vec<_> = (0..5)
                .map(|i| {
                    let mut vm = VM::new(growing_memory(memory.clone()));
                    vm.registers.pending_in = Some(phases[i]);
                    vm
                })
                .collect();
            
            let mut signal = 0;
            let mut halted = false;
            while !halted {
                for i in 0..5 {
                    vms[i].registers.pending_in = Some(signal);
                    // Run until it either halts or yields after writing
                    while vms[i].state != crate::utils::intcode::State::Halted {
                        if vms[i].run_all_async(|_| Ok(()))? {
                            // Returned true -> Halted
                            break;
                        }
                        if let Some(out) = vms[i].registers.pending_out.take() {
                            signal = out;
                            break;
                        }
                    }
                    if i == 4 && vms[4].state == crate::utils::intcode::State::Halted {
                        halted = true;
                    }
                }
            }
            max_signal = max_signal.max(signal);
        }
        Ok(max_signal.to_string())
    }
}

aoc_rust_common::aoc_test!(Day07);
