use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1(content: String) -> usize {
    2
}

fn part2() -> usize {
    2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_part1() {
        let text = include_str!("./example.data").to_owned();
        assert_eq!(part1(text), 0);
        let text = include_str!("./prod.data").to_owned();
        assert_eq!(part1(text), 646);
    }
    #[test]
    #[ignore]
    fn test_part2() {
        // let text = include_str!("./prod.data").to_owned();
        let result = part2();
        assert_eq!(result, 681);
    }
}
