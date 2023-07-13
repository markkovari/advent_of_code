use std::collections::{HashMap, HashSet};

use serde_scan::scan;

fn part1(content: String) -> Option<usize> {
    let mut map = HashMap::new();

    for l in content.lines() {
        let loc: (i32, i32) = scan!("{}, {}" <- l).unwrap();
        map.insert(loc, 0);
    }

    let mut infinites = HashSet::<(i32, i32)>::new();
    let edges = [-400, 1000];

    for x in -400..=1000 {
        for y in -400..=1000 {
            let mut min_dist = 10_000_000;
            let mut closest = None;
            for &(a, b) in map.keys() {
                let dist = (a - x).abs() + (b - y).abs();
                if dist < min_dist {
                    min_dist = dist;
                    closest = Some((a, b));
                } else if dist == min_dist {
                    closest = None;
                }
            }

            if closest.is_some() {
                let closest = closest.unwrap();
                *map.entry(closest).or_insert(0) += 1;
                if edges.contains(&x) || edges.contains(&y) {
                    infinites.insert(closest);
                }
            }
        }
    }

    let max = map
        .iter()
        .filter(|x| !infinites.contains(&x.0))
        .max_by_key(|&(a, b)| b);

    max.unwrap().1.to_owned().into()
}

fn part2(content: String) -> usize {
    let mut map = HashMap::new();

    for l in content.lines() {
        let loc: (i32, i32) = scan!("{}, {}" <- l).unwrap();
        map.insert(loc, 0);
    }

    let mut infinites = HashSet::<(i32, i32)>::new();
    let edges = [-400, 1000];

    for x in -400..=1000 {
        for y in -400..=1000 {
            let mut min_dist = 10_000_000;
            let mut closest = None;
            for &(a, b) in map.keys() {
                let dist = (a - x).abs() + (b - y).abs();
                if dist < min_dist {
                    min_dist = dist;
                    closest = Some((a, b));
                } else if dist == min_dist {
                    closest = None;
                }
            }

            if closest.is_some() {
                let closest = closest.unwrap();
                *map.entry(closest).or_insert(0) += 1;
                if edges.contains(&x) || edges.contains(&y) {
                    infinites.insert(closest);
                }
            }
        }
    }

    let limit = 10_000;
    let heuristic = 10_000 / map.len();
    let mut count = 0;

    for x in -100..=600 {
        for y in -100..=600 {
            let mut sum = 0;
            for &(a, b) in map.keys() {
                sum += (a - x).abs() + (b - y).abs();
            }

            if sum < limit {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_first() {
        let example = include_str!("./example.data");
        let result = part1(example.to_owned());
        assert_eq!(17, result.unwrap());
        let prod = include_str!("./prod.data");
        let result = part1(prod.to_owned());
        assert_eq!(3933, result.unwrap());
    }

    #[test]
    #[ignore]

    fn test_second() {
        let example = include_str!("./example.data");
        let result = part2(example.to_owned());
        assert_eq!(491401, result);
        let example = include_str!("./prod.data");
        let result = part2(example.to_owned());
        assert_eq!(41145, result);
    }
}
