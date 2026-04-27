use anyhow::Result;
use aoc_rust_common::Solution;
use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Moon {
    pos: [i32; 3],
    vel: [i32; 3],
}

impl Moon {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            pos: [x, y, z],
            vel: [0, 0, 0],
        }
    }

    fn energy(&self) -> i32 {
        let pot: i32 = self.pos.iter().map(|&x| x.abs()).sum();
        let kin: i32 = self.vel.iter().map(|&x| x.abs()).sum();
        pot * kin
    }
}

pub struct Day12;

fn parse(input: &str) -> Vec<Moon> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let parts: Vec<i32> = l
                .split(|c: char| !c.is_numeric() && c != '-')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect();
            Moon::new(parts[0], parts[1], parts[2])
        })
        .collect()
}

fn step(moons: &mut [Moon]) {
    for i in 0..moons.len() {
        for j in 0..moons.len() {
            if i == j {
                continue;
            }
            for d in 0..3 {
                match moons[i].pos[d].cmp(&moons[j].pos[d]) {
                    Ordering::Less => moons[i].vel[d] += 1,
                    Ordering::Greater => moons[i].vel[d] -= 1,
                    Ordering::Equal => {}
                }
            }
        }
    }
    for moon in moons.iter_mut() {
        for d in 0..3 {
            moon.pos[d] += moon.vel[d];
        }
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    (a / gcd(a, b)) * b
}

impl Solution for Day12 {
    fn year(&self) -> u32 {
        2019
    }
    fn day(&self) -> u32 {
        12
    }

    fn part1(&self, input: &str) -> Result<String> {
        let mut moons = parse(input);
        for _ in 0..1000 {
            step(&mut moons);
        }
        let total_energy: i32 = moons.iter().map(|m| m.energy()).sum();
        Ok(total_energy.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let initial_moons = parse(input);
        let mut periods = [0i64; 3];

        for d in 0..3 {
            let mut moons = initial_moons.clone();
            let mut count = 0i64;
            loop {
                count += 1;
                // Simplified step for single dimension
                for i in 0..moons.len() {
                    for j in 0..moons.len() {
                        match moons[i].pos[d].cmp(&moons[j].pos[d]) {
                            Ordering::Less => moons[i].vel[d] += 1,
                            Ordering::Greater => moons[i].vel[d] -= 1,
                            Ordering::Equal => {}
                        }
                    }
                }
                for moon in moons.iter_mut() {
                    moon.pos[d] += moon.vel[d];
                }

                if moons.iter().enumerate().all(|(i, m)| {
                    m.pos[d] == initial_moons[i].pos[d] && m.vel[d] == initial_moons[i].vel[d]
                }) {
                    periods[d] = count;
                    break;
                }
            }
        }

        let ans = lcm(lcm(periods[0], periods[1]), periods[2]);
        Ok(ans.to_string())
    }
}

aoc_rust_common::aoc_test!(Day12);
