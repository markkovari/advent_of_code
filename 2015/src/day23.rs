use lazy_static::lazy_static;
use regex::Regex;
use std::{
    cmp,
    collections::{BTreeSet, HashMap, HashSet},
    fmt::Debug,
    ops::AddAssign,
    vec,
};

use iter_tools::Itertools;

use crate::{Excercise, Solvable};

struct TwentyThirdDay {
    exercise: Excercise,
}

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
            v => Err(format!("unknown register {}", v)),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Offset(isize);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Instruction {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jump(Offset),
    JumpIfEven(Register, Offset),
    JumpIfOne(Register, Offset),
}

impl Instruction {
    fn parse(input: &str) -> Instruction {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(?P<op>\w{3}) ?(?P<reg>[ab])?,? ?(?P<offset>[+-]\d+)?$").unwrap();
        }
        let captures = RE.captures(input).unwrap();

        let op = captures.name("op").unwrap().as_str();
        let get_register = || captures.name("reg").unwrap().as_str().try_into().unwrap();
        let get_offset = || Offset(captures.name("offset").unwrap().as_str().parse().unwrap());

        match op {
            "hlf" => Instruction::Half(get_register()),
            "tpl" => Instruction::Triple(get_register()),
            "inc" => Instruction::Increment(get_register()),
            "jmp" => Instruction::Jump(get_offset()),
            "jie" => Instruction::JumpIfEven(get_register(), get_offset()),
            "jio" => Instruction::JumpIfOne(get_register(), get_offset()),
            v => panic!("uknown op {}", v),
        }
    }
}

struct Computer {
    a: u32,
    b: u32,
    pc: isize,
}

impl Computer {
    fn new() -> Self {
        Self { a: 0, b: 0, pc: 0 }
    }

    fn get(&self, r: Register) -> u32 {
        match r {
            Register::A => self.a,
            Register::B => self.b,
        }
    }

    fn set(&mut self, r: Register, value: u32) {
        match r {
            Register::A => self.a = value,
            Register::B => self.b = value,
        }
    }

    fn modify(&mut self, r: Register, f: fn(u32) -> u32) {
        match r {
            Register::A => self.a = f(self.a),
            Register::B => self.b = f(self.b),
        }
    }

    fn execute(&mut self, program: &[Instruction]) {
        loop {
            let ir = match program.get(self.pc as usize) {
                Some(ir) => ir,
                None => return,
            };

            match ir {
                Instruction::Half(r) => {
                    self.modify(*r, |v| v / 2);
                }
                Instruction::Triple(r) => {
                    self.modify(*r, |v| v * 3);
                }
                Instruction::Increment(r) => {
                    self.modify(*r, |v| v + 1);
                }
                Instruction::Jump(offset) => {
                    self.pc += offset.0 - 1;
                }
                Instruction::JumpIfEven(r, offset) => {
                    if self.get(*r) % 2 == 0 {
                        self.pc += offset.0 - 1;
                    }
                }
                Instruction::JumpIfOne(r, offset) => {
                    if self.get(*r) == 1 {
                        self.pc += offset.0 - 1;
                    }
                }
            }

            self.pc += 1;
        }
    }
}

impl TwentyThirdDay {
    fn solve_first(&self, input: &str) -> usize {
        self.first(&input)
    }

    fn solve_second(&self, input: &str) -> usize {
        self.second(&input)
    }

    fn first(&self, input: &str) -> usize {
        let instructions = input
            .lines()
            .map(|l| Instruction::parse(l))
            .collect::<Vec<_>>();

        let mut computer = Computer::new();

        computer.execute(&instructions);

        computer.get(Register::B) as usize
    }

    fn second(&self, input: &str) -> usize {
        let instructions = input
            .lines()
            .map(|l| Instruction::parse(l))
            .collect::<Vec<_>>();

        let mut computer = Computer::new();
        computer.set(Register::A, 1);

        computer.execute(&instructions);

        computer.get(Register::B) as usize
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::E;

    use super::*;
    const EXAMPLE: &str = include_str!("23_test.txt");
    const PROD: &str = include_str!("23_prod.txt");

    #[test]
    fn first_test() {
        let mut first_excersise = TwentyThirdDay {
            exercise: Excercise {
                content: String::from(PROD),
                example: String::from(EXAMPLE),
            },
        };

        let expected_example = 170;
        let expected_prod = 170;
        let result_example = first_excersise.first(PROD);
        let result_prod = first_excersise.first(PROD);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);

        let expected_example = 247;
        let expected_prod = 247;
        let result_example = first_excersise.second(PROD);
        let result_prod = first_excersise.second(PROD);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);
    }
}
