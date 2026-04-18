use anyhow::Result;

pub trait Solution {
    fn year(&self) -> u32;
    fn day(&self) -> u32;

    fn part1(&self, input: &str) -> Result<String>;
    fn part2(&self, input: &str) -> Result<String>;
}
