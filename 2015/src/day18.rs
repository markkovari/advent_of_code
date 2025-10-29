use std::{collections::HashMap, fmt::Debug};

use crate::Exercise;
use rayon::prelude::*;
// use iter_tools::{dependency::itertools::Product, Itertools};
// use regex::Regex;

struct EightteenthDay {
    exercise: Exercise,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum Cell {
    Alive,
    Dead,
}

impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Alive => write!(f, "#"),
            Cell::Dead => write!(f, "."),
        }
    }
}

type Grid = HashMap<(i64, i64), Cell>;

fn stringify_grid(grid: &Grid) -> String {
    let mut string = String::new();
    let mut max_x = 0;
    let mut max_y = 0;
    for pos in grid.keys() {
        if pos.0 > max_x {
            max_x = pos.0;
        }
        if pos.1 > max_y {
            max_y = pos.1;
        }
    }
    for y in 0..=max_y {
        for x in 0..=max_x {
            string.push(match grid.get(&(x, y)) {
                Some(Cell::Alive) => '#',
                Some(Cell::Dead) => '.',
                None => '.',
            });
        }
        string.push('\n');
    }
    string
}

fn read_grid(content: &str) -> Grid {
    let mut grid: Grid = HashMap::new();
    for (y, line) in content.lines().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            grid.insert(
                (x as i64, y as i64),
                if cell == '#' { Cell::Alive } else { Cell::Dead },
            );
        }
    }
    grid
}

fn get_neighbors(grid: &Grid, x: i64, y: i64) -> Vec<Cell> {
    let mut neighbors: Vec<Cell> = Vec::new();
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }
            neighbors.push(*grid.get(&(x + i, y + j)).unwrap_or(&Cell::Dead));
        }
    }
    neighbors
}

fn evolve(grid: &Grid) -> Grid {
    grid.par_iter()
        .map(|(pos, cell)| {
            let neighbors = get_neighbors(grid, pos.0, pos.1);
            let alive_neighbors = neighbors.iter().filter(|e| **e == Cell::Alive).count();
            let new_cell = match cell {
                Cell::Alive => {
                    if alive_neighbors == 2 || alive_neighbors == 3 {
                        Cell::Alive
                    } else {
                        Cell::Dead
                    }
                }
                Cell::Dead => {
                    if alive_neighbors == 3 {
                        Cell::Alive
                    } else {
                        Cell::Dead
                    }
                }
            };
            (*pos, new_cell)
        })
        .collect()
}

fn setup_corner(grid: &mut Grid) -> &Grid {
    let max_x = grid.keys().map(|e| e.0).max().unwrap();
    let max_y = grid.keys().map(|e| e.1).max().unwrap();
    grid.insert((0, 0), Cell::Alive);
    grid.insert((0, max_y), Cell::Alive);
    grid.insert((max_x, 0), Cell::Alive);
    grid.insert((max_x, max_y), Cell::Alive);
    grid
}

impl EightteenthDay {
    fn solve_first(&self, is_prod: bool, amount: usize) -> i64 {
        if is_prod {
            self.first(&self.exercise.content, amount)
        } else {
            self.first(&self.exercise.example, amount)
        }
    }

    fn solve_second(&self, is_prod: bool, amount: u64) -> i64 {
        if is_prod {
            self.second(&self.exercise.content, amount)
        } else {
            self.second(&self.exercise.example, amount)
        }
    }

    fn first(&self, content: &str, amount: usize) -> i64 {
        let mut grid = read_grid(content);
        for _ in 0..amount {
            grid = evolve(&grid);
        }
        grid.values().filter(|e| **e == Cell::Alive).count() as i64
    }

    fn second(&self, content: &str, amount: u64) -> i64 {
        let mut grid = read_grid(content);
        for _ in 0..amount {
            setup_corner(&mut grid);
            grid = evolve(&grid);
        }
        setup_corner(&mut grid);
        grid.values().filter(|e| **e == Cell::Alive).count() as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("inputs/18_test.txt");
    const PROD: &str = include_str!("inputs/18_prod.txt");

    #[test]
    fn first_test() {
        let mut first_exercise = EightteenthDay {
            exercise: Exercise {
                content: String::from(PROD),
                example: String::from(EXAMPLE),
            },
        };

        let expected_example = 4;
        let expected_prod = 1061;
        let result_example = first_exercise.solve_first(false, 4);
        let result_prod = first_exercise.solve_first(true, 100);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);

        let expected_example = 17;
        let expected_prod = 1006;
        let new_example = String::from(
            "##.#.#
...##.
#....#
..#...
#.#..#
####..",
        );

        let result_example = first_exercise.second(&new_example, 5);
        let result_prod = first_exercise.solve_second(true, 100);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);
    }
}
