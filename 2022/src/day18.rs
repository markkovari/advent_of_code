use anyhow::Result;
use aoc_rust_common::Solution;
use itertools::Itertools;
use std::collections::HashSet;

pub struct Day18;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Default)]
struct Coord {
    x: i16,
    y: i16,
    z: i16,
}

impl Coord {
    fn neighbours(&self) -> impl Iterator<Item = Coord> + '_ {
        [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ]
        .iter()
        .map(|&(dx, dy, dz)| Coord {
            x: self.x + dx,
            y: self.y + dy,
            z: self.z + dz,
        })
    }

    fn in_bounds(&self, bounds: &[Coord; 2]) -> bool {
        let [mins, maxs] = bounds;
        self.x >= mins.x - 1
            && self.x <= maxs.x + 1
            && self.y >= mins.y - 1
            && self.y <= maxs.y + 1
            && self.z >= mins.z - 1
            && self.z <= maxs.z + 1
    }
}

fn parse(input: &str) -> HashSet<Coord> {
    input
        .lines()
        .map(|line| {
            let mut nums = line.split(',').map(|s| s.parse().unwrap());
            Coord {
                x: nums.next().unwrap(),
                y: nums.next().unwrap(),
                z: nums.next().unwrap(),
            }
        })
        .collect()
}

fn bounds(cubes: &HashSet<Coord>) -> [Coord; 2] {
    cubes.iter().fold(
        [
            Coord {
                x: i16::MAX,
                y: i16::MAX,
                z: i16::MAX,
            },
            Coord {
                x: i16::MIN,
                y: i16::MIN,
                z: i16::MIN,
            },
        ],
        |[mut mins, mut maxs], cube| {
            mins.x = mins.x.min(cube.x);
            mins.y = mins.y.min(cube.y);
            mins.z = mins.z.min(cube.z);
            maxs.x = maxs.x.max(cube.x);
            maxs.y = maxs.y.max(cube.y);
            maxs.z = maxs.z.max(cube.z);
            [mins, maxs]
        },
    )
}

impl Solution for Day18 {
    fn year(&self) -> u32 {
        2022
    }
    fn day(&self) -> u32 {
        18
    }

    fn part1(&self, input: &str) -> Result<String> {
        let cubes = parse(input);
        let count = cubes
            .iter()
            .flat_map(|c| c.neighbours())
            .filter(|n| !cubes.contains(n))
            .count();
        Ok((count).to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let cubes = parse(input);
        let bounds = bounds(&cubes);
        let mut exposed = HashSet::new();
        let mut stack = vec![Coord {
            x: bounds[0].x - 1,
            y: bounds[0].y - 1,
            z: bounds[0].z - 1,
        }];
        exposed.insert(stack[0]);

        while let Some(coord) = stack.pop() {
            for neighbour in coord.neighbours() {
                if !cubes.contains(&neighbour)
                    && neighbour.in_bounds(&bounds)
                    && exposed.insert(neighbour)
                {
                    stack.push(neighbour);
                }
            }
        }

        let count = cubes
            .iter()
            .flat_map(|c| c.neighbours())
            .filter(|n| exposed.contains(n))
            .count();
        Ok((count).to_string())
    }
}
