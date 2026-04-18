use aoc_rust_common::Solution;
use onig::Regex;
use std::fmt::Display;

pub struct Day08;

pub fn raw_and_unescaped_len(s: &str) -> (usize, usize) {
    if !s.starts_with('"') || !s.ends_with('"') {
        panic!("invalid format (not quoted)");
    }
    let raw_len = s.len();
    let re = Regex::new(r#"\\(\\|"|x[0-9a-f]{2})"#).unwrap();
    let ss = &s[1..s.len() - 1];
    let (esc_count, esc_size) =
        re.find_iter(ss)
            .fold((0, 0), |(esc_count, esc_size), (start_pos, end_pos)| {
                (esc_count + 1, esc_size + (end_pos - start_pos))
            });
    (raw_len, raw_len - 2 - esc_size + esc_count)
}

pub fn raw_and_reescaped_len(s: &str) -> (usize, usize) {
    let raw_len = s.len();
    let re = Regex::new(r#"[\\"]"#).unwrap();
    let esc_count = re.find_iter(s).count();
    (raw_len, raw_len + 2 + esc_count)
}

impl Solution for Day08 {
    fn year(&self) -> u32 {
        2015
    }
    fn day(&self) -> u32 {
        8
    }

    fn part1(&self, input: &str) -> Box<dyn Display> {
        Box::new(input.lines().fold(0, |extra_chars, line| {
            let (raw_len, unescaped_len) = raw_and_unescaped_len(line);
            extra_chars + (raw_len - unescaped_len)
        }) as i64)
    }

    fn part2(&self, input: &str) -> Box<dyn Display> {
        Box::new(input.lines().fold(0, |extra_chars, line| {
            let (raw_len, reescaped_len) = raw_and_reescaped_len(line);
            extra_chars + (reescaped_len - raw_len)
        }) as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day08() {
        let day = Day08;
        let example = r#"""
"abc"
"aaa\"aaa"
"\x27""#;
        assert_eq!(day.part1(example).to_string(), "12");
        assert_eq!(day.part2(example).to_string(), "19");
    }
}
