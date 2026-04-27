use anyhow::Result;
use aoc_rust_common::Point;
use aoc_rust_common::Solution;
use std::collections::{HashMap, HashSet};

pub struct Day10;

impl Solution for Day10 {
    fn year(&self) -> u32 {
        2019
    }
    fn day(&self) -> u32 {
        10
    }

    fn part1(&self, input: &str) -> Result<String> {
        let asteroids = parse(input);
        let (_, count) = find_best_station(&asteroids);
        Ok(count.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let mut asteroids = parse(input);
        let (station, _) = find_best_station(&asteroids);
        asteroids.remove(&station);

        let mut vaporized = Vec::new();
        while !asteroids.is_empty() {
            let mut visible: Vec<_> = get_visible_asteroids(&station, &asteroids);
            // Sort by angle (clockwise from up)
            visible.sort_by(|a, b| {
                let angle_a = get_angle(&station, a);
                let angle_b = get_angle(&station, b);
                angle_a.partial_cmp(&angle_b).unwrap()
            });

            for asteroid in visible {
                vaporized.push(asteroid);
                asteroids.remove(&asteroid);
            }
        }

        if vaporized.len() >= 200 {
            let p = vaporized[199];
            Ok((p.x * 100 + p.y).to_string())
        } else {
            anyhow::bail!("Fewer than 200 asteroids vaporized")
        }
    }
}

fn parse(input: &str) -> HashSet<Point> {
    let mut asteroids = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                asteroids.insert(Point::new(x as i32, y as i32));
            }
        }
    }
    asteroids
}

fn find_best_station(asteroids: &HashSet<Point>) -> (Point, usize) {
    asteroids
        .iter()
        .map(|&station| {
            let count = get_visible_asteroids(&station, asteroids).len();
            (station, count)
        })
        .max_by_key(|&(_, count)| count)
        .unwrap()
}

fn get_visible_asteroids(station: &Point, asteroids: &HashSet<Point>) -> Vec<Point> {
    let mut angles: HashMap<String, Point> = HashMap::new();
    for &asteroid in asteroids {
        if asteroid == *station {
            continue;
        }
        let dx = asteroid.x - station.x;
        let dy = asteroid.y - station.y;
        let g = gcd(dx.abs() as i64, dy.abs() as i64) as i32;
        let reduced = format!("{}/{}", dx / g, dy / g);
        let entry = angles.entry(reduced).or_insert(asteroid);
        if station.manhattan_distance(&asteroid) < station.manhattan_distance(entry) {
            *entry = asteroid;
        }
    }
    angles.into_values().collect()
}

fn get_angle(station: &Point, asteroid: &Point) -> f64 {
    let dx = asteroid.x - station.x;
    let dy = asteroid.y - station.y;
    let mut angle = (dx as f64).atan2(-dy as f64);
    if angle < 0.0 {
        angle += 2.0 * std::f64::consts::PI;
    }
    angle
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

aoc_rust_common::aoc_test!(Day10);
