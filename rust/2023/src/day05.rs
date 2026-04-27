use anyhow::Result;
use aoc_rust_common::Solution;

pub struct Day05;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Range {
    start: i64,
    end: i64,
}

#[derive(Debug)]
struct Mapping {
    dest_start: i64,
    src_start: i64,
    length: i64,
}

impl Mapping {
    fn src_end(&self) -> i64 {
        self.src_start + self.length - 1
    }
}

struct ResourceMap {
    mappings: Vec<Mapping>,
}

impl ResourceMap {
    fn map_range(&self, range: Range) -> Vec<Range> {
        let mut result = Vec::new();
        let mut to_process = vec![range];

        for mapping in &self.mappings {
            let mut next_to_process = Vec::new();
            let src_start = mapping.src_start;
            let src_end = mapping.src_end();
            let shift = mapping.dest_start - mapping.src_start;

            for r in to_process {
                // Check overlap
                let overlap_start = r.start.max(src_start);
                let overlap_end = r.end.min(src_end);

                if overlap_start <= overlap_end {
                    // Overlapping part
                    result.push(Range {
                        start: overlap_start + shift,
                        end: overlap_end + shift,
                    });

                    // Part before overlap
                    if r.start < overlap_start {
                        next_to_process.push(Range {
                            start: r.start,
                            end: overlap_start - 1,
                        });
                    }

                    // Part after overlap
                    if r.end > overlap_end {
                        next_to_process.push(Range {
                            start: overlap_end + 1,
                            end: r.end,
                        });
                    }
                } else {
                    // No overlap
                    next_to_process.push(r);
                }
            }
            to_process = next_to_process;
        }
        result.extend(to_process);
        result
    }
}

fn parse_input(input: &str) -> (Vec<i64>, Vec<ResourceMap>) {
    let mut sections = input.split("\n\n");
    let seeds_line = sections.next().expect("seeds line missing");
    let seeds: Vec<i64> = seeds_line
        .strip_prefix("seeds: ")
        .expect("invalid seeds line")
        .split_whitespace()
        .map(|s| s.parse().expect("failed to parse seed"))
        .collect();

    let mut maps = Vec::new();
    for section in sections {
        let mut lines = section.lines();
        lines.next(); // skip header
        let mut mappings = Vec::new();
        for line in lines {
            let nums: Vec<i64> = line
                .split_whitespace()
                .map(|s| s.parse().expect("failed to parse mapping"))
                .collect();
            if nums.len() == 3 {
                mappings.push(Mapping {
                    dest_start: nums[0],
                    src_start: nums[1],
                    length: nums[2],
                });
            }
        }
        maps.push(ResourceMap { mappings });
    }
    (seeds, maps)
}

impl Solution for Day05 {
    fn year(&self) -> u32 {
        2023
    }
    fn day(&self) -> u32 {
        5
    }

    fn part1(&self, input: &str) -> Result<String> {
        let (seeds, maps) = parse_input(input);
        let mut current_ranges: Vec<Range> =
            seeds.iter().map(|&s| Range { start: s, end: s }).collect();

        for map in maps {
            let mut next_ranges = Vec::new();
            for r in current_ranges {
                next_ranges.extend(map.map_range(r));
            }
            current_ranges = next_ranges;
        }

        let min_start = current_ranges
            .iter()
            .map(|r| r.start)
            .min()
            .expect("no ranges");
        Ok(min_start.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let (seeds_raw, maps) = parse_input(input);
        let mut current_ranges = Vec::new();
        for chunk in seeds_raw.chunks(2) {
            current_ranges.push(Range {
                start: chunk[0],
                end: chunk[0] + chunk[1] - 1,
            });
        }

        for map in maps {
            let mut next_ranges = Vec::new();
            for r in current_ranges {
                next_ranges.extend(map.map_range(r));
            }
            current_ranges = next_ranges;
        }

        let min_start = current_ranges
            .iter()
            .map(|r| r.start)
            .min()
            .expect("no ranges");
        Ok(min_start.to_string())
    }
}

aoc_rust_common::aoc_test!(Day05, "1181555926", "37806486");
