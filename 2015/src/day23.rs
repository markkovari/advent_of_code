use anyhow::Result;
use aoc_rust_common::Solution;
use lazy_static::lazy_static;
use regex::Regex;

pub struct Day23;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum Register {
    A,
    B,
}

impl TryFrom<&str> for Register {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "a" => Ok(Register::A),
            "b" => Ok(Register::B),
            _ => Err(format!("unknown register {}", value)),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Instruction {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jump(isize),
    JumpIfEven(Register, isize),
    JumpIfOne(Register, isize),
}

impl Instruction {
    fn parse(input: &str) -> Instruction {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(?P<op>\w{3}) ?(?P<reg>[ab])?,? ?(?P<offset>[+-]\d+)?$").unwrap();
        }
        let captures = RE.captures(input).unwrap();
        let op = captures.name("op").unwrap().as_str();
        let get_reg = || captures.name("reg").unwrap().as_str().try_into().unwrap();
        let get_off = || captures.name("offset").unwrap().as_str().parse().unwrap();
        match op {
            "hlf" => Instruction::Half(get_reg()),
            "tpl" => Instruction::Triple(get_reg()),
            "inc" => Instruction::Increment(get_reg()),
            "jmp" => Instruction::Jump(get_off()),
            "jie" => Instruction::JumpIfEven(get_reg(), get_off()),
            "jio" => Instruction::JumpIfOne(get_reg(), get_off()),
            _ => panic!("unknown op"),
        }
    }
}

struct Computer {
    a: u32,
    b: u32,
    pc: isize,
}
impl Computer {
    fn new(a_start: u32) -> Self {
        Self {
            a: a_start,
            b: 0,
            pc: 0,
        }
    }
    fn get(&self, r: Register) -> u32 {
        match r {
            Register::A => self.a,
            Register::B => self.b,
        }
    }
    fn modify<F: Fn(u32) -> u32>(&mut self, r: Register, f: F) {
        match r {
            Register::A => self.a = f(self.a),
            Register::B => self.b = f(self.b),
        }
    }
    fn run(&mut self, prog: &[Instruction]) {
        while let Some(ir) = prog.get(self.pc as usize) {
            match ir {
                Instruction::Half(r) => self.modify(*r, |v| v / 2),
                Instruction::Triple(r) => self.modify(*r, |v| v * 3),
                Instruction::Increment(r) => self.modify(*r, |v| v + 1),
                Instruction::Jump(off) => {
                    self.pc += off - 1;
                }
                Instruction::JumpIfEven(r, off) => {
                    if self.get(*r).is_multiple_of(2) {
                        self.pc += off - 1;
                    }
                }
                Instruction::JumpIfOne(r, off) => {
                    if self.get(*r) == 1 {
                        self.pc += off - 1;
                    }
                }
            }
            self.pc += 1;
        }
    }
}

impl Solution for Day23 {
    fn year(&self) -> u32 {
        2015
    }
    fn day(&self) -> u32 {
        23
    }

    fn part1(&self, input: &str) -> Result<String> {
        let instructions = input.lines().map(Instruction::parse).collect::<Vec<_>>();
        let mut computer = Computer::new(0);
        computer.run(&instructions);
        Ok((computer.get(Register::B)).to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let instructions = input.lines().map(Instruction::parse).collect::<Vec<_>>();
        let mut computer = Computer::new(1);
        computer.run(&instructions);
        Ok((computer.get(Register::B)).to_string())
    }
}
