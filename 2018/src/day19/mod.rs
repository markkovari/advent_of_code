use anyhow::Result;
use aoc_rust_common::Solution;
use std::str::FromStr;

pub struct Day19;

type Word = u32;
type Regs = [Word; 6];
type Instruction = (Insn, [usize; 3]);

#[derive(Clone, Copy, Debug)]
enum Insn {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

impl FromStr for Insn {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "addr" => Insn::Addr,
            "addi" => Insn::Addi,
            "mulr" => Insn::Mulr,
            "muli" => Insn::Muli,
            "banr" => Insn::Banr,
            "bani" => Insn::Bani,
            "borr" => Insn::Borr,
            "bori" => Insn::Bori,
            "setr" => Insn::Setr,
            "seti" => Insn::Seti,
            "gtir" => Insn::Gtir,
            "gtri" => Insn::Gtri,
            "gtrr" => Insn::Gtrr,
            "eqir" => Insn::Eqir,
            "eqri" => Insn::Eqri,
            "eqrr" => Insn::Eqrr,
            _ => return Err(()),
        })
    }
}

fn execute((insn, args): &Instruction, regs: &mut Regs) {
    regs[args[2]] = match insn {
        Insn::Addr => regs[args[0]] + regs[args[1]],
        Insn::Addi => regs[args[0]] + args[1] as Word,
        Insn::Mulr => regs[args[0]] * regs[args[1]],
        Insn::Muli => regs[args[0]] * args[1] as Word,
        Insn::Banr => regs[args[0]] & regs[args[1]],
        Insn::Bani => regs[args[0]] & args[1] as Word,
        Insn::Borr => regs[args[0]] | regs[args[1]],
        Insn::Bori => regs[args[0]] | args[1] as Word,
        Insn::Setr => regs[args[0]],
        Insn::Seti => args[0] as Word,
        Insn::Gtir => (args[0] as Word > regs[args[1]]) as Word,
        Insn::Gtri => (regs[args[0]] > args[1] as Word) as Word,
        Insn::Gtrr => (regs[args[0]] > regs[args[1]]) as Word,
        Insn::Eqir => (args[0] as Word == regs[args[1]]) as Word,
        Insn::Eqri => (regs[args[0]] == args[1] as Word) as Word,
        Insn::Eqrr => (regs[args[0]] == regs[args[1]]) as Word,
    };
}

fn solve(ipr: usize, opcodes: &[Instruction], r0: Word) -> Word {
    let mut regs = [r0, 0, 0, 0, 0, 0];
    while regs[ipr] != 1 {
        execute(&opcodes[regs[ipr] as usize], &mut regs);
        regs[ipr] += 1;
    }
    let seed = *regs.iter().max().unwrap();
    (1..=seed).filter(|i| seed % i == 0).sum()
}

impl Solution for Day19 {
    fn year(&self) -> u32 {
        2018
    }
    fn day(&self) -> u32 {
        19
    }

    fn part1(&self, input: &str) -> Result<String> {
        let mut lines = input.lines();
        let ipr: usize = lines.next().unwrap()[4..].parse().unwrap();
        let instrs: Vec<Instruction> = lines
            .map(|l| {
                let words: Vec<_> = l.split(' ').collect();
                let ops = [
                    words[1].parse().unwrap(),
                    words[2].parse().unwrap(),
                    words[3].parse().unwrap(),
                ];
                (words[0].parse().unwrap(), ops)
            })
            .collect();
        Ok((solve(ipr, &instrs, 0)).to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let mut lines = input.lines();
        let ipr: usize = lines.next().unwrap()[4..].parse().unwrap();
        let instrs: Vec<Instruction> = lines
            .map(|l| {
                let words: Vec<_> = l.split(' ').collect();
                let ops = [
                    words[1].parse().unwrap(),
                    words[2].parse().unwrap(),
                    words[3].parse().unwrap(),
                ];
                (words[0].parse().unwrap(), ops)
            })
            .collect();
        Ok((solve(ipr, &instrs, 1)).to_string())
    }
}
