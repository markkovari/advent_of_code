use anyhow::Result;
use aoc_rust_common::Solution;
use serde_json::Value;

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

    fn part1(&self, input: &str) -> Result<String> {
        let value: Value = serde_json::from_str(input).unwrap();
        Ok((count_value(&value) as i64).to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let value: Value = serde_json::from_str(input).unwrap();
        Ok((skip_red(&value) as i64).to_string())
    }
}

aoc_rust_common::aoc_test!(Day12, "6", "6");
