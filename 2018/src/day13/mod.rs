use aoc_rust_common::Solution;
use std::fmt::Display;
use std::collections::HashSet;

pub struct Day13;

#[derive(Clone, Copy)]
enum Track { Empty, Vertical, Horizontal, Intersection, CurveForward, CurveBackward }

impl From<char> for Track {
    fn from(c: char) -> Self {
        match c {
            '|' | '^' | 'v' => Track::Vertical,
            '-' | '<' | '>' => Track::Horizontal,
            '+' => Track::Intersection,
            '/' => Track::CurveForward,
            '\\' => Track::CurveBackward,
            _ => Track::Empty,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
struct Coordinate { y: usize, x: usize }

#[derive(Clone, Copy, Eq, PartialEq)]
enum Direction { Up, Down, Left, Right }

#[derive(Clone, Copy)]
struct Cart {
    pos: Coordinate,
    dir: Direction,
    intersections: usize,
}

impl Cart {
    fn turn(&mut self) {
        self.dir = match self.dir {
            Direction::Up => match self.intersections % 3 { 0 => Direction::Left, 1 => Direction::Up, _ => Direction::Right },
            Direction::Down => match self.intersections % 3 { 0 => Direction::Right, 1 => Direction::Down, _ => Direction::Left },
            Direction::Left => match self.intersections % 3 { 0 => Direction::Down, 1 => Direction::Left, _ => Direction::Up },
            Direction::Right => match self.intersections % 3 { 0 => Direction::Up, 1 => Direction::Right, _ => Direction::Down },
        };
        self.intersections += 1;
    }
    
    fn apply_curve(&mut self, track: Track) {
        self.dir = match track {
            Track::CurveForward => match self.dir {
                Direction::Up => Direction::Right, Direction::Down => Direction::Left,
                Direction::Left => Direction::Down, Direction::Right => Direction::Up,
            },
            Track::CurveBackward => match self.dir {
                Direction::Up => Direction::Left, Direction::Down => Direction::Right,
                Direction::Left => Direction::Up, Direction::Right => Direction::Down,
            },
            _ => self.dir
        };
    }
}

fn parse_input(input: &str) -> (Vec<Vec<Track>>, Vec<Cart>) {
    let mut carts = Vec::new();
    let grid: Vec<Vec<Track>> = input.lines().enumerate().map(|(y, line)| {
        line.chars().enumerate().map(|(x, c)| {
            if let Some(dir) = match c {
                '^' => Some(Direction::Up), 'v' => Some(Direction::Down),
                '<' => Some(Direction::Left), '>' => Some(Direction::Right),
                _ => None,
            } {
                carts.push(Cart { pos: Coordinate { y, x }, dir, intersections: 0 });
            }
            Track::from(c)
        }).collect()
    }).collect();
    (grid, carts)
}

impl Solution for Day13 {
    fn year(&self) -> u32 { 2018 }
    fn day(&self) -> u32 { 13 }

    fn part1(&self, input: &str) -> Box<dyn Display> {
        let (grid, mut carts) = parse_input(input);
        loop {
            carts.sort_by_key(|c| c.pos);
            let mut positions = carts.iter().map(|c| c.pos).collect::<HashSet<_>>();
            for i in 0..carts.len() {
                let cart = carts[i];
                positions.remove(&cart.pos);
                
                let mut next_pos = cart.pos;
                match cart.dir {
                    Direction::Up => next_pos.y -= 1, Direction::Down => next_pos.y += 1,
                    Direction::Left => next_pos.x -= 1, Direction::Right => next_pos.x += 1,
                }

                if !positions.insert(next_pos) {
                    return Box::new(format!("{},{}", next_pos.x, next_pos.y));
                }
                
                let mut next_cart = Cart { pos: next_pos, ..cart };
                match grid[next_pos.y][next_pos.x] {
                    Track::Intersection => next_cart.turn(),
                    Track::CurveForward | Track::CurveBackward => next_cart.apply_curve(grid[next_pos.y][next_pos.x]),
                    _ => {},
                }
                carts[i] = next_cart;
            }
        }
    }

    fn part2(&self, input: &str) -> Box<dyn Display> {
        let (grid, mut carts) = parse_input(input);
        while carts.len() > 1 {
            carts.sort_by_key(|c| c.pos);
            let mut positions = carts.iter().map(|c| c.pos).collect::<HashSet<_>>();
            let mut next_carts = Vec::new();
            let mut crashed_this_tick = HashSet::new();

            for cart in carts {
                if crashed_this_tick.contains(&cart.pos) { continue; }

                positions.remove(&cart.pos);
                let mut next_pos = cart.pos;
                match cart.dir {
                    Direction::Up => next_pos.y -= 1,
                    Direction::Down => next_pos.y += 1,
                    Direction::Left => next_pos.x -= 1,
                    Direction::Right => next_pos.x += 1,
                }

                if !positions.insert(next_pos) {
                    crashed_this_tick.insert(next_pos);
                    next_carts.retain(|c: &Cart| c.pos != next_pos);
                } else {
                    let mut next_cart = Cart { pos: next_pos, ..cart };
                     match grid[next_pos.y][next_pos.x] {
                        Track::Intersection => next_cart.turn(),
                        Track::CurveForward | Track::CurveBackward => next_cart.apply_curve(grid[next_pos.y][next_pos.x]),
                        _ => {},
                    }
                    next_carts.push(next_cart);
                }
            }
            carts = next_carts;
        }
        if let Some(last_cart) = carts.first() {
            Box::new(format!("{},{}", last_cart.pos.x, last_cart.pos.y))
        } else {
            Box::new("No cart left")
        }
    }
}
