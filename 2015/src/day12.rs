use aoc_rust_common::Solution;
use serde_json::Value;
use std::fmt::Display;

pub struct Day12;

fn count_value(value: &Value) -> i32 {
    match value {
        Value::Number(e) => e.as_i64().unwrap_or(0) as i32,
        Value::Array(array) => array.iter().map(count_value).sum(),
        Value::Object(object) => object.values().map(count_value).sum(),
        _ => 0,
    }
}

fn skip_red(value: &Value) -> i32 {
    match value {
        Value::Number(e) => e.as_i64().unwrap_or(0) as i32,
        Value::Array(array) => array.iter().map(skip_red).sum(),
        Value::Object(object) => {
            if object.values().any(|v| v.as_str() == Some("red")) {
                0
            } else {
                object.values().map(skip_red).sum()
            }
        }
        _ => 0,
    }
}

impl Solution for Day12 {
    fn year(&self) -> u32 {
        2015
    }
    fn day(&self) -> u32 {
        12
    }

    fn part1(&self, input: &str) -> Box<dyn Display> {
        let value: Value = serde_json::from_str(input).unwrap();
        Box::new(count_value(&value) as i64)
    }

    fn part2(&self, input: &str) -> Box<dyn Display> {
        let value: Value = serde_json::from_str(input).unwrap();
        Box::new(skip_red(&value) as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day12() {
        let day = Day12;
        assert_eq!(day.part1("[1,2,3]").to_string(), "6");
        assert_eq!(day.part1(r#"{"a":2,"b":4}"#).to_string(), "6");
        assert_eq!(day.part2("[1,2,3]").to_string(), "6");
        assert_eq!(day.part2(r#"[1,{"c":"red","a":2},3]"#).to_string(), "4");
    }
}
