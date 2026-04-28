use anyhow::Result;
use aoc_rust_common::Solution;

pub struct Day08;

impl Solution for Day08 {
    fn year(&self) -> u32 {
        2018
    }
    fn day(&self) -> u32 {
        8
    }

    fn part1(&self, input: &str) -> Result<String> {
        // Use compile-time constants for layer size calculation
        const WIDTH: u32 = 25;
        const HEIGHT: u32 = 6;
        const LAYER_SIZE: u32 = aoc_rust_common::layer_size(WIDTH, HEIGHT);

        let layers: Vec<&[u8]> = input.trim().as_bytes().chunks(LAYER_SIZE as usize).collect();

        let mut min_zeros = usize::MAX;
        let mut result = 0;

        for layer in layers {
            let zeros = layer.iter().filter(|&&c| c == b'0').count();
            if zeros < min_zeros {
                min_zeros = zeros;
                let ones = layer.iter().filter(|&&c| c == b'1').count();
                let twos = layer.chars().filter(|&c| c == '2').count();
                result = ones * twos;
            }
        }

        Ok(result.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        const WIDTH: u32 = 25;
        const HEIGHT: u32 = 6;
        const LAYER_SIZE: u32 = aoc_rust_common::layer_size(WIDTH, HEIGHT);

        let mut final_image = vec![b'2'; LAYER_SIZE as usize];

        for layer in input.trim().as_bytes().chunks(LAYER_SIZE as usize) {
            for i in 0..LAYER_SIZE as usize {
                if final_image[i] == b'2' {
                    final_image[i] = layer[i];
                }
            }
        }

        let mut output = String::new();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let pixel = final_image[(y * WIDTH + x) as usize];
                output.push(if pixel == b'1' { '#' } else { ' ' });
            }
            output.push('
');
        }

        Ok(output.trim_end().to_string())
    }
}

aoc_rust_common::aoc_test!(Day08);
