use aoc_rust_common::Solution;
use std::fmt::Display;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::i64 as nom_i64,
    multi::separated_list0,
    sequence::delimited,
    IResult,
    Parser,
};

pub struct Day13;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Packet {
    Int(i64),
    List(Vec<Packet>),
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    alt((
        nom_i64.map(Packet::Int),
        delimited(
            tag("["),
            separated_list0(tag(","), parse_packet),
            tag("]"),
        ).map(Packet::List),
    ))(input)
}

impl Solution for Day13 {
    fn year(&self) -> u32 { 2022 }
    fn day(&self) -> u32 { 13 }

    fn part1(&self, input: &str) -> Box<dyn Display> {
        let mut count = 0;
        for (i, pair) in input.split("\n\n").enumerate() {
            let mut lines = pair.lines();
            let left = parse_packet(lines.next().unwrap()).unwrap().1;
            let right = parse_packet(lines.next().unwrap()).unwrap().1;
            if left < right { count += i + 1; }
        }
        Box::new(count)
    }

    fn part2(&self, input: &str) -> Box<dyn Display> {
        let mut packets: Vec<Packet> = input
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| parse_packet(l).unwrap().1)
            .collect();

        let div1 = Packet::List(vec![Packet::List(vec![Packet::Int(2)])]);
        let div2 = Packet::List(vec![Packet::List(vec![Packet::Int(6)])]);
        packets.push(div1.clone());
        packets.push(div2.clone());
        packets.sort();

        let pos1 = packets.iter().position(|e| e == &div1).unwrap() + 1;
        let pos2 = packets.iter().position(|e| e == &div2).unwrap() + 1;
        Box::new(pos1 * pos2)
    }
}
