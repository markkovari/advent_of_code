use anyhow::Result;
use aoc_rust_common::Solution;
use std::collections::{BTreeMap, HashSet, VecDeque};

pub struct Day15;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Wall,
    Open,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum UnitKind {
    Elf,
    Goblin,
}

#[derive(Clone, Debug)]
struct Unit {
    kind: UnitKind,
    hp: usize,
    attack: usize,
    pos: Coordinate,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
struct Coordinate {
    y: usize,
    x: usize,
}

fn parse_input(input: &str, elf_attack: usize) -> (BTreeMap<Coordinate, Cell>, Vec<Unit>) {
    let mut grid = BTreeMap::new();
    let mut units = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            let pos = Coordinate { y, x };
            match char {
                '#' => {
                    grid.insert(pos, Cell::Wall);
                }
                '.' => {
                    grid.insert(pos, Cell::Open);
                }
                'G' => {
                    grid.insert(pos, Cell::Open);
                    units.push(Unit {
                        kind: UnitKind::Goblin,
                        hp: 200,
                        attack: 3,
                        pos,
                    });
                }
                'E' => {
                    grid.insert(pos, Cell::Open);
                    units.push(Unit {
                        kind: UnitKind::Elf,
                        hp: 200,
                        attack: elf_attack,
                        pos,
                    });
                }
                _ => {}
            }
        }
    }
    (grid, units)
}

fn simulate(grid: &BTreeMap<Coordinate, Cell>, units: &mut Vec<Unit>) -> Option<usize> {
    let mut rounds = 0;
    loop {
        units.sort_by_key(|u| u.pos);

        for i in 0..units.len() {
            if units[i].hp == 0 {
                continue;
            }

            let enemies_present = units.iter().any(|u| u.kind != units[i].kind && u.hp > 0);
            if !enemies_present {
                return Some(
                    rounds
                        * units
                            .iter()
                            .filter(|u| u.hp > 0)
                            .map(|u| u.hp)
                            .sum::<usize>(),
                );
            }

            let current_pos = units[i].pos;
            let current_kind = units[i].kind;

            let targets: Vec<_> = units
                .iter()
                .filter(|u| u.kind != current_kind && u.hp > 0)
                .cloned()
                .collect();
            let in_range: Vec<_> = targets
                .iter()
                .flat_map(|t| neighbors(t.pos, grid))
                .collect();

            let adjacent_to_enemy = neighbors(current_pos, grid)
                .iter()
                .any(|n| targets.iter().any(|t| t.pos == *n));

            if !adjacent_to_enemy {
                if let Some(next_pos) = find_next_step(current_pos, &in_range, grid, units) {
                    units[i].pos = next_pos;
                }
            }

            let updated_pos = units[i].pos;
            let attack_power = units[i].attack;

            let attackable_targets: Vec<_> = neighbors(updated_pos, grid);
            let best_target = units
                .iter_mut()
                .filter(|u| {
                    u.hp > 0 && u.kind != current_kind && attackable_targets.contains(&u.pos)
                })
                .min_by_key(|t| (t.hp, t.pos));

            if let Some(target) = best_target {
                if target.hp <= attack_power {
                    target.hp = 0;
                } else {
                    target.hp -= attack_power;
                }
            }
        }
        units.retain(|u| u.hp > 0);
        rounds += 1;
    }
}

fn find_next_step(
    start: Coordinate,
    targets: &[Coordinate],
    grid: &BTreeMap<Coordinate, Cell>,
    units: &[Unit],
) -> Option<Coordinate> {
    let mut queue = VecDeque::new();
    queue.push_back((start, Vec::new()));
    let mut visited = HashSet::new();
    visited.insert(start);

    let unit_positions: HashSet<_> = units.iter().filter(|u| u.hp > 0).map(|u| u.pos).collect();

    let mut found_paths = Vec::new();

    while let Some((pos, path)) = queue.pop_front() {
        if targets.contains(&pos) {
            found_paths.push(path);
            continue;
        }

        for &neighbor in &neighbors(pos, grid) {
            if !visited.contains(&neighbor) && !unit_positions.contains(&neighbor) {
                let mut new_path = path.clone();
                new_path.push(neighbor);
                visited.insert(neighbor);
                queue.push_back((neighbor, new_path));
            }
        }
    }

    if found_paths.is_empty() {
        return None;
    }

    found_paths.sort_by(|a, b| {
        if a.len() != b.len() {
            a.len().cmp(&b.len())
        } else {
            a.last().unwrap().cmp(b.last().unwrap())
        }
    });

    found_paths.first().and_then(|p| p.first()).copied()
}

fn neighbors(pos: Coordinate, grid: &BTreeMap<Coordinate, Cell>) -> Vec<Coordinate> {
    let mut result = Vec::new();
    let moves = [(-1, 0), (0, -1), (0, 1), (1, 0)]; // Reading order: up, left, right, down
    for (dy, dx) in moves.iter() {
        let new_pos = Coordinate {
            y: (pos.y as isize + dy) as usize,
            x: (pos.x as isize + dx) as usize,
        };
        if grid.get(&new_pos) == Some(&Cell::Open) {
            result.push(new_pos);
        }
    }
    result
}

impl Solution for Day15 {
    fn year(&self) -> u32 {
        2018
    }
    fn day(&self) -> u32 {
        15
    }

    fn part1(&self, input: &str) -> Result<String> {
        let (grid, mut units) = parse_input(input, 3);
        Ok((simulate(&grid, &mut units).unwrap_or(0)).to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        for elf_attack in 4.. {
            let (grid, mut units) = parse_input(input, elf_attack);
            let initial_elves = units.iter().filter(|u| u.kind == UnitKind::Elf).count();
            if let Some(outcome) = simulate(&grid, &mut units) {
                let final_elves = units.iter().filter(|u| u.kind == UnitKind::Elf).count();
                if initial_elves == final_elves {
                    return Ok((outcome).to_string());
                }
            }
        }
        Ok(("No solution found").to_string())
    }
}

aoc_rust_common::aoc_test!(Day15);
