use std::str::FromStr;

type Word = u32;
type Regs = [Word; 6];
type Instruction = (Insn, [usize; 3]);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
enum Insn {
    addr,
    addi,
    mulr,
    muli,
    banr,
    bani,
    borr,
    bori,
    setr,
    seti,
    gtir,
    gtri,
    gtrr,
    eqir,
    eqri,
    eqrr,
}

impl FromStr for Insn {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "addr" => Ok(Insn::addr),
            "addi" => Ok(Insn::addi),
            "mulr" => Ok(Insn::mulr),
            "muli" => Ok(Insn::muli),
            "banr" => Ok(Insn::banr),
            "bani" => Ok(Insn::bani),
            "borr" => Ok(Insn::borr),
            "bori" => Ok(Insn::bori),
            "setr" => Ok(Insn::setr),
            "seti" => Ok(Insn::seti),
            "gtir" => Ok(Insn::gtir),
            "gtri" => Ok(Insn::gtri),
            "gtrr" => Ok(Insn::gtrr),
            "eqir" => Ok(Insn::eqir),
            "eqri" => Ok(Insn::eqri),
            "eqrr" => Ok(Insn::eqrr),
            _ => Err(s.to_owned()),
        }
    }
}

fn input_generator(input: &str) -> (usize, Vec<Instruction>) {
    let mut lines = input.lines();
    let ipr = lines.next().unwrap()[4..].parse::<usize>().unwrap();
    let instrs = lines
        .map(|l| {
            let words = l.split(' ').collect::<Vec<_>>();
            let insn = words[0].parse::<Insn>().unwrap();
            let ops = [
                words[1].parse::<usize>().unwrap(),
                words[2].parse::<usize>().unwrap(),
                words[3].parse::<usize>().unwrap(),
            ];
            (insn, ops)
        })
        .collect::<Vec<_>>();
    (ipr, instrs)
}

fn execute((insn, args): &Instruction, regs: &mut Regs) {
    regs[args[2]] = match insn {
        Insn::addr => regs[args[0]] + regs[args[1]],
        Insn::addi => regs[args[0]] + args[1] as Word,
        Insn::mulr => regs[args[0]] * regs[args[1]],
        Insn::muli => regs[args[0]] * args[1] as Word,
        Insn::banr => regs[args[0]] & regs[args[1]],
        Insn::bani => regs[args[0]] & args[1] as Word,
        Insn::borr => regs[args[0]] | regs[args[1]],
        Insn::bori => regs[args[0]] | args[1] as Word,
        Insn::setr => regs[args[0]],
        Insn::seti => args[0] as Word,
        Insn::gtir => (args[0] as Word > regs[args[1]]) as Word,
        Insn::gtri => (regs[args[0]] > args[1] as Word) as Word,
        Insn::gtrr => (regs[args[0]] > regs[args[1]]) as Word,
        Insn::eqir => (args[0] as Word == regs[args[1]]) as Word,
        Insn::eqri => (regs[args[0]] == args[1] as Word) as Word,
        Insn::eqrr => (regs[args[0]] == regs[args[1]]) as Word,
    };
}

fn solution(content: String) -> (Word, Word) {
    let input = input_generator(&content);
    (part1(&input), part2(&input))
}

fn part1((ipr, opcodes): &(usize, Vec<Instruction>)) -> Word {
    solve(*ipr, opcodes, 0)
}

fn part2((ipr, opcodes): &(usize, Vec<Instruction>)) -> Word {
    solve(*ipr, opcodes, 1)
}

fn solve(ipr: usize, opcodes: &[Instruction], r0: Word) -> Word {
    let mut regs = [r0, 0, 0, 0, 0, 0];
    while regs[ipr] != 1 {
        execute(&opcodes[regs[ipr] as usize], &mut regs);
        regs[ipr] += 1;
    }
    let seed = *regs.iter().max().unwrap();
    let mut total = 0;
    for i in 1..=seed {
        if seed % i == 0 {
            total += i;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_part1() {
        let text = include_str!("./example.data").to_owned();
        assert_eq!(solution(text), (6, 1));
    }

    #[test]
    #[ignore]
    fn test_part2() {
        let text = include_str!("./prod.data").to_owned();
        assert_eq!(solution(text), (1488, 17427456));
    }
}
