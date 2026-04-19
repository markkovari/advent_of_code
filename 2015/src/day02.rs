use anyhow::Result;
use aoc_rust_common::Solution;
use rayon::prelude::*;

pub struct Day02;

fn calculate_wrapper_needed_materials(line: &str) -> Result<(i64, i64)> {
    let dimensions = line
        .split('x')
        .map(|x| {
            x.parse::<i64>()
                .map_err(|e| anyhow::anyhow!("failed to parse dimension: {}", e))
        })
        .collect::<Result<Vec<i64>>>()?;
    if dimensions.len() < 3 {
        return Err(anyhow::anyhow!("invalid input line: {}", line));
    }
    let l = dimensions[0];
    let w = dimensions[1];
    let h = dimensions[2];

    let areas = [l * w, w * h, h * l];
    let smallest = areas
        .iter()
        .min()
        .ok_or_else(|| anyhow::anyhow!("no areas"))?;

    let perimeters = [2 * (l + w), 2 * (w + h), 2 * (h + l)];
    let smallest_perimeter = perimeters
        .iter()
        .min()
        .ok_or_else(|| anyhow::anyhow!("no perimeters"))?;

    Ok((
        2 * l * w + 2 * w * h + 2 * h * l + smallest,
        smallest_perimeter + l * w * h,
    ))
}

impl Solution for Day02 {
    fn year(&self) -> u32 {
        2015
    }
    fn day(&self) -> u32 {
        2
    }

    fn part1(&self, input: &str) -> Result<String> {
        let results: Result<Vec<(i64, i64)>> = input
            .par_lines()
            .map(calculate_wrapper_needed_materials)
            .collect();

        Ok(results?
            .into_iter()
            .map(|(wrapping, _)| wrapping)
            .sum::<i64>()
            .to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let results: Result<Vec<(i64, i64)>> = input
            .par_lines()
            .map(calculate_wrapper_needed_materials)
            .collect();

        Ok(results?
            .into_iter()
            .map(|(_, ribbon)| ribbon)
            .sum::<i64>()
            .to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day02() {
        let day = Day02;
        assert_eq!(day.part1("2x3x4").unwrap(), "58");
        assert_eq!(day.part1("1x1x10").unwrap(), "43");

        assert_eq!(day.part2("2x3x4").unwrap(), "34");
        assert_eq!(day.part2("1x1x10").unwrap(), "14");
    }
}
