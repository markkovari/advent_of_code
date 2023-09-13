use std::collections::{HashMap, HashSet};

use std::io::BufRead;

use itertools::Itertools;
use text_io::scan;

use self::OpCode::*;

type Reg = usize;

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

impl OpCode {
    fn apply(self, registers: &mut [Reg], a: usize, b: usize, c: usize) {
        registers[c] = match self {
            AddR => registers[a] + registers[b],
            AddI => registers[a] + b,
            MulR => registers[a] * registers[b],
            MulI => registers[a] * b,
            BanR => registers[a] & registers[b],
            BanI => registers[a] & b,
            BorR => registers[a] | registers[b],
            BorI => registers[a] | b,
            SetR => registers[a],
            SetI => a,
            GtIR => (a > registers[b]).into(),
            GtRI => (registers[a] > b).into(),
            GtRR => (registers[a] > registers[b]).into(),
            EqIR => (a == registers[b]).into(),
            EqRI => (registers[a] == b).into(),
            EqRR => (registers[a] == registers[b]).into(),
        }
    }
}

fn part1(content: String) -> usize {
    let result = content
        .lines()
        .by_ref()
        .scan(0, |state, line| {
            if line.is_empty() {
                *state += 1;
            } else {
                *state = 0;
            }
            if *state >= 2 {
                None
            } else {
                Some(line)
            }
        })
        .tuples()
        .filter(|(l1, l2, l3, _)| {
            let (r0, r1, r2, r3): (Reg, Reg, Reg, Reg);
            let (s0, s1, s2, s3): (Reg, Reg, Reg, Reg);
            let (op, a, b, c): (usize, usize, usize, usize);
            scan!(l1.bytes() => "Before: [{}, {}, {}, {}]", r0, r1, r2, r3);
            scan!(l2.bytes() => "{} {} {} {}", op, a, b, c);
            scan!(l3.bytes() => "After:  [{}, {}, {}, {}]", s0, s1, s2, s3);

            ALL_OP_CODES
                .iter()
                .filter(|op_code| {
                    let mut registers = [r0, r1, r2, r3];
                    op_code.apply(&mut registers, a, b, c);
                    registers == [s0, s1, s2, s3]
                })
                .count()
                >= 3
        })
        .count();
    return result;
}

fn part2() -> usize {
    let mut line_iter = include_str!("prod.data").lines();

    let mut op_codes = vec![ALL_OP_CODES.iter().cloned().collect::<HashSet<OpCode>>(); 16];
    let mut known_codes = HashMap::new();

    for (l1, l2, l3, _) in line_iter
        .by_ref()
        .scan(0, |state, line| {
            if line.is_empty() {
                *state += 1;
            } else {
                *state = 0;
            }
            if *state >= 2 {
                None
            } else {
                Some(line)
            }
        })
        .tuples()
    {
        let (r0, r1, r2, r3): (Reg, Reg, Reg, Reg);
        let (s0, s1, s2, s3): (Reg, Reg, Reg, Reg);
        let (op, a, b, c): (usize, usize, usize, usize);
        scan!(l1.bytes() => "Before: [{}, {}, {}, {}]", r0, r1, r2, r3);
        scan!(l2.bytes() => "{} {} {} {}", op, a, b, c);
        scan!(l3.bytes() => "After:  [{}, {}, {}, {}]", s0, s1, s2, s3);

        let possible: HashSet<OpCode> = ALL_OP_CODES
            .iter()
            .cloned()
            .filter(|op_code| {
                let mut registers = [r0, r1, r2, r3];
                op_code.apply(&mut registers, a, b, c);
                registers == [s0, s1, s2, s3]
            })
            .collect();

        op_codes[op].retain(|op_code| possible.contains(&op_code));
    }

    while known_codes.len() < ALL_OP_CODES.len() {
        for (code, ops) in op_codes.iter().enumerate() {
            if ops.len() == 1 && !known_codes.contains_key(&code) {
                known_codes.insert(code, ops.iter().cloned().next().unwrap());
            }
        }
        for (&code, &op) in known_codes.iter() {
            for (code2, ops) in op_codes.iter_mut().enumerate() {
                if code2 != code {
                    ops.remove(&op);
                }
            }
        }
    }
    println!("{:?}", known_codes);

    let mut registers = [0, 0, 0, 0];
    for line in line_iter {
        if line.is_empty() {
            continue;
        }
        let (op, a, b, c): (usize, usize, usize, usize);
        scan!(line.bytes() => "{} {} {} {}", op, a, b, c);
        known_codes[&op].apply(&mut registers, a, b, c);
    }
    return registers[0];
}

const ALL_OP_CODES: &[OpCode] = &[
    AddR, AddI, MulR, MulI, BanR, BanI, BorR, BorI, SetR, SetI, GtIR, GtRI, GtRR, EqIR, EqRI, EqRR,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_part1() {
        let text = include_str!("./example.data").to_owned();
        assert_eq!(part1(text), 0);
        let text = include_str!("./prod.data").to_owned();
        assert_eq!(part1(text), 646);
    }
    #[test]
    #[ignore]
    fn test_part2() {
        // let text = include_str!("./prod.data").to_owned();
        let result = part2();
        assert_eq!(result, 681);
    }
}
