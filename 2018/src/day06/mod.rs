use aoc_rust_common::Solution;
use std::fmt::Display;
use std::collections::{HashMap, HashSet};
use serde_scan::scan;

pub struct Day06;

impl Solution for Day06 {
    fn year(&self) -> u32 { 2018 }
    fn day(&self) -> u32 { 6 }

    fn part1(&self, input: &str) -> Box<dyn Display> {
        let coords: Vec<(i32, i32)> = input.lines().map(|l| scan!("{}, {}" <- l).unwrap()).collect();
        let (min_x, max_x, min_y, max_y) = coords.iter().fold(
            (i32::MAX, i32::MIN, i32::MAX, i32::MIN),
            |(min_x, max_x, min_y, max_y), &(x, y)| {
                (min_x.min(x), max_x.max(x), min_y.min(y), max_y.max(y))
            },
        );

        let mut areas = HashMap::new();
        let mut infinites = HashSet::new();

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let distances: Vec<_> = coords.iter().map(|&(px, py)| (px - x).abs() + (py - y).abs()).collect();
                let min_dist = *distances.iter().min().unwrap();
                
                if distances.iter().filter(|&&d| d == min_dist).count() == 1 {
                    let owner_idx = distances.iter().position(|&d| d == min_dist).unwrap();
                    *areas.entry(owner_idx).or_insert(0) += 1;
                    if x == min_x || x == max_x || y == min_y || y == max_y {
                        infinites.insert(owner_idx);
                    }
                }
            }
        }
        
        let max_area = areas.iter()
            .filter(|(k, _)| !infinites.contains(k))
            .map(|(_, v)| v)
            .max().unwrap_or(&0);
            
        Box::new(*max_area)
    }

    fn part2(&self, input: &str) -> Box<dyn Display> {
        let coords: Vec<(i32, i32)> = input.lines().map(|l| scan!("{}, {}" <- l).unwrap()).collect();
        let (min_x, max_x, min_y, max_y) = coords.iter().fold(
            (i32::MAX, i32::MIN, i32::MAX, i32::MIN),
            |(min_x, max_x, min_y, max_y), &(x, y)| {
                (min_x.min(x), max_x.max(x), min_y.min(y), max_y.max(y))
            },
        );
        let limit = if coords.len() < 10 { 32 } else { 10000 };
        let safe_region_size = (min_x..=max_x).flat_map(|x| {
            let coords = coords.clone();
            (min_y..=max_y).filter(move |&y| {
                coords.iter().map(|&(px, py)| (px - x).abs() + (py - y).abs()).sum::<i32>() < limit
            })
        }).count();
        
        Box::new(safe_region_size)
    }
}
