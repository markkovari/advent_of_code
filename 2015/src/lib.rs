pub mod day18;
pub mod day08;
pub mod day11;
pub mod day15;
pub mod day05;
pub mod day01;
pub mod day14;
pub mod day04;
pub mod day19;
pub mod day09;
pub mod day02;
pub mod day17;
pub mod day07;
pub mod day16;
pub mod day06;
pub mod day10;
pub mod day13;
pub mod day03;
pub mod day12;
pub mod day20;
pub mod day25;
pub mod day21;
pub mod day24;
pub mod day22;
pub mod day23;
pub mod day26;

struct Excercise {
    content: String,
    example: String,
}

trait Readable {
    fn get_example(&self) -> String;
    fn get_prod(&self) -> String;
}

impl Readable for Excercise {
    fn get_example(&self) -> String {
        self.example.clone()
    }
    fn get_prod(&self) -> String {
        self.content.clone()
    }
}

trait Solvable {
    fn first(&self, content: String) -> i32;
    fn solve_first(&self, is_prod: bool) -> i32;
    fn second(&self, content: String) -> i32;
    fn solve_second(&self, is_prod: bool) -> i32;
}
