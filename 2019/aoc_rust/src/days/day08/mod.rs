use anyhow::Result;
use aoc_rust_common::Solution;

pub struct Day08;

impl Solution for Day08 {
    fn year(&self) -> u32 {
        2019
    }
    fn day(&self) -> u32 {
        8
    }

    fn part1(&self, input: &str) -> Result<String> {
        let width = 25;
        let height = 6;
        let layer_size = width * height;
        let layers: Vec<&str> = input
            .trim()
            .as_bytes()
            .chunks(layer_size)
            .map(|c| std::str::from_utf8(c).unwrap())
            .collect();

        let mut min_zeros = layer_size + 1;
        let mut result = 0;

        for layer in layers {
            let zeros = layer.chars().filter(|&c| c == '0').count();
            if zeros < min_zeros {
                min_zeros = zeros;
                let ones = layer.chars().filter(|&c| c == '1').count();
                let twos = layer.chars().filter(|&c| c == '2').count();
                result = ones * twos;
            }
        }

        Ok(result.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let width = 25;
        let height = 6;
        let layer_size = width * height;
        let layers: Vec<&[u8]> = input.trim().as_bytes().chunks(layer_size).collect();

        let mut final_image = vec![b'2'; layer_size];

        for layer in layers {
            for i in 0..layer_size {
                if final_image[i] == b'2' {
                    final_image[i] = layer[i];
                }
            }
        }

        let mut output = String::new();
        for y in 0..height {
            for x in 0..width {
                let pixel = final_image[y * width + x];
                output.push(if pixel == b'1' { '#' } else { ' ' });
            }
            output.push('\n');
        }

        Ok(output.trim_end().to_string())
    }
}

aoc_rust_common::aoc_test!(Day08);
