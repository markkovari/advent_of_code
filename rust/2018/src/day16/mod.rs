use anyhow::Result;
use aoc_common::Solution;
use std::collections::{HashMap, HashSet};
use text_io::scan;

pub struct Day16;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum OpCode {
    AddR,
    AddI,
    MulR,
    MulI,
    BanR,
    BanI,
    BorR,
    BorI,
    SetR,
    SetI,
    GtIR,
    GtRI,
    GtRR,
    EqIR,
    EqRI,
    EqRR,
}

const ALL_OP_CODES: &[OpCode] = &[
    OpCode::AddR,
    OpCode::AddI,
    OpCode::MulR,
    OpCode::MulI,
    OpCode::BanR,
    OpCode::BanI,
    OpCode::BorR,
    OpCode::BorI,
    OpCode::SetR,
    OpCode::SetI,
    OpCode::GtIR,
    OpCode::GtRI,
    OpCode::GtRR,
    OpCode::EqIR,
    OpCode::EqRI,
    OpCode::EqRR,
];

impl OpCode {
    fn apply(self, registers: &mut [usize], a: usize, b: usize, c: usize) {
        registers[c] = match self {
            OpCode::AddR => registers[a] + registers[b],
            OpCode::AddI => registers[a] + b,
            OpCode::MulR => registers[a] * registers[b],
            OpCode::MulI => registers[a] * b,
            OpCode::BanR => registers[a] & registers[b],
            OpCode::BanI => registers[a] & b,
            OpCode::BorR => registers[a] | registers[b],
            OpCode::BorI => registers[a] | b,
            OpCode::SetR => registers[a],
            OpCode::SetI => a,
            OpCode::GtIR => (a > registers[b]).into(),
            OpCode::GtRI => (registers[a] > b).into(),
            OpCode::GtRR => (registers[a] > registers[b]).into(),
            OpCode::EqIR => (a == registers[b]).into(),
            OpCode::EqRI => (registers[a] == b).into(),
            OpCode::EqRR => (registers[a] == registers[b]).into(),
        };
    }
}

struct Sample {
    before: [usize; 4],
    instruction: [usize; 4],
    after: [usize; 4],
}

fn parse_input(input: &str) -> (Vec<Sample>, Vec<[usize; 4]>) {
    let mut samples = Vec::new();
    let mut program = Vec::new();
    let mut lines = input.lines();

    loop {
        let l1 = match lines.next() {
            Some(l) if !l.is_empty() => l,
            _ => break,
        };
        let l2 = lines.next().unwrap();
        let l3 = lines.next().unwrap();
        lines.next(); // Consume empty line

        let (r0, r1, r2, r3): (usize, usize, usize, usize);
        scan!(l1.bytes() => "Before: [{}, {}, {}, {}]", r0, r1, r2, r3);
        let before = [r0, r1, r2, r3];

        let (op, a, b, c): (usize, usize, usize, usize);
        scan!(l2.bytes() => "{} {} {} {}", op, a, b, c);
        let instruction = [op, a, b, c];

        let (s0, s1, s2, s3): (usize, usize, usize, usize);
        scan!(l3.bytes() => "After:  [{}, {}, {}, {}]", s0, s1, s2, s3);
        let after = [s0, s1, s2, s3];

        samples.push(Sample {
            before,
            instruction,
            after,
        });
    }

    // After the samples, there are two blank lines, then the test program
    lines.next();
    for line in lines {
        let (op, a, b, c): (usize, usize, usize, usize);
        scan!(line.bytes() => "{} {} {} {}", op, a, b, c);
        program.push([op, a, b, c]);
    }

    (samples, program)
}

impl Solution for Day16 {
    fn year(&self) -> u32 {
        2018
    }
    fn day(&self) -> u32 {
        16
    }

    fn part1(&self, input: &str) -> Result<String> {
        let (samples, _) = parse_input(input);
        let result = samples
            .iter()
            .filter(|sample| {
                let [_, a, b, c] = sample.instruction;
                ALL_OP_CODES
                    .iter()
                    .filter(|opcode| {
                        let mut registers = sample.before;
                        opcode.apply(&mut registers, a, b, c);
                        registers == sample.after
                    })
                    .count()
                    >= 3
            })
            .count();
        Ok((result).to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let (samples, program) = parse_input(input);
        let mut op_map: HashMap<usize, OpCode> = HashMap::new();
        let mut possible_ops: HashMap<usize, HashSet<OpCode>> = (0..16)
            .map(|i| (i, ALL_OP_CODES.iter().cloned().collect()))
            .collect();

        for sample in &samples {
            let [op_num, a, b, c] = sample.instruction;
            let possibilities = possible_ops.get_mut(&op_num).unwrap();
            possibilities.retain(|opcode| {
                let mut registers = sample.before;
                opcode.apply(&mut registers, a, b, c);
                registers == sample.after
            });
        }

        while op_map.len() < 16 {
            for (op_num, possibilities) in &possible_ops {
                if possibilities.len() == 1 {
                    let opcode = possibilities.iter().next().unwrap().clone();
                    op_map.insert(*op_num, opcode);
                }
            }
            for opcode in op_map.values() {
                for possibilities in possible_ops.values_mut() {
                    possibilities.remove(opcode);
                }
            }
        }

        let mut registers = [0, 0, 0, 0];
        for instruction in &program {
            let [op_num, a, b, c] = *instruction;
            op_map[&op_num].apply(&mut registers, a, b, c);
        }
        Ok((registers[0]).to_string())
    }
}

aoc_common::aoc_test!(Day16);
