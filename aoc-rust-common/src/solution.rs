use std::fmt::Display;

pub trait Solution {
    fn year(&self) -> u32;
    fn day(&self) -> u32;
    
    fn part1(&self, input: &str) -> Box<dyn Display>;
    fn part2(&self, input: &str) -> Box<dyn Display>;
}
