use crate::{Exercise, Solvable};
use rayon::prelude::*;

struct SixthDay {
    exercise: Exercise,
}

type Grid = [[bool; 1000]; 1000];
type GridAmbient = [[i8; 1000]; 1000];

fn apply_on_grid(instruction: Instruction, grid: &mut Grid) {
    for x in instruction.start.0..=instruction.end.0 {
        for y in instruction.start.1..=instruction.end.1 {
            match instruction.action {
                Action::On => grid[x as usize][y as usize] = true,
                Action::Off => grid[x as usize][y as usize] = false,
                Action::Toggle => grid[x as usize][y as usize] = !grid[x as usize][y as usize],
            }
        }
    }
}

fn apply_on_ambient_grid(instruction: Instruction, grid: &mut GridAmbient) {
    for x in instruction.start.0..=instruction.end.0 {
        for y in instruction.start.1..=instruction.end.1 {
            grid[x as usize][y as usize] += match (instruction.action, grid[x as usize][y as usize])
            {
                (Action::On, _) => 1,
                (Action::Off, 0) => 0,
                (Action::Off, _) => -1,
                (_, _) => 2,
            }
        }
    }
}

fn apply_instructions(instructions: Vec<Instruction>, grid: &mut Grid) {
    for instruction in instructions {
        apply_on_grid(instruction, grid);
    }
}

fn apply_increasing_instructions(instructions: Vec<Instruction>, grid: &mut GridAmbient) {
    for instruction in instructions {
        apply_on_ambient_grid(instruction, grid);
    }
}

fn count_lit(grid: &Grid) -> i64 {
    grid.par_iter()
        .flat_map(|row| row.par_iter())
        .filter(|&&cell| cell)
        .count() as i64
}

fn count_brigthness(grid: &GridAmbient) -> i64 {
    grid.par_iter()
        .flat_map(|row| row.par_iter())
        .map(|&cell| cell as i64)
        .sum()
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Action {
    On,
    Off,
    Toggle,
}

impl TryFrom<&str> for Action {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.starts_with("turn on") {
            return Ok(Action::On);
        } else if value.starts_with("turn off") {
            return Ok(Action::Off);
        } else if value.starts_with("toggle") {
            return Ok(Action::Toggle);
        }
        Err(String::from("Invalid action"))
    }
}

struct Instruction {
    action: Action,
    start: Point,
    end: Point,
}

impl TryFrom<&str> for Instruction {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut split = value.split_whitespace().collect::<Vec<&str>>();
        let end: Point = Point::try_from(split.pop().unwrap())?;
        split.pop();
        let start: Point = Point::try_from(split.pop().unwrap())?;
        let action: Action = Action::try_from(split.join(" ").as_str())?;
        Ok(Instruction { action, start, end })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Point(i32, i32);

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point(x, y)
    }
}

impl TryFrom<&str> for Point {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut split = value.split(',');
        let x = split.next().unwrap().parse::<i32>().unwrap();
        let y = split.next().unwrap().parse::<i32>().unwrap();
        Ok(Point::new(x, y))
    }
}

impl Solvable for SixthDay {
    fn solve_first(&self, is_prod: bool) -> i64 {
        if is_prod {
            self.first(&self.exercise.content)
        } else {
            self.first(&self.exercise.example)
        }
    }

    fn solve_second(&self, is_prod: bool) -> i64 {
        if is_prod {
            self.second(&self.exercise.content)
        } else {
            self.second(&self.exercise.example)
        }
    }

    fn first(&self, content: &str) -> i64 {
        let instructions = content
            .lines()
            .map(|line| Instruction::try_from(line).unwrap())
            .collect::<Vec<Instruction>>();
        let mut grid: Grid = [[false; 1000]; 1000];
        apply_instructions(instructions, &mut grid);
        count_lit(&grid)
    }

    fn second(&self, content: &str) -> i64 {
        let instructions = content
            .lines()
            .map(|line| Instruction::try_from(line).unwrap())
            .collect::<Vec<Instruction>>();
        let mut grid: GridAmbient = [[0; 1000]; 1000];
        apply_increasing_instructions(instructions, &mut grid);
        count_brigthness(&grid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("inputs/6_test.txt");
    const PROD: &str = include_str!("inputs/6_prod.txt");

    #[test]
    fn instruction() {
        let instruction = Instruction::try_from("turn on 0,0 through 999,999").unwrap();
        assert_eq!(instruction.action, Action::On);
        assert_eq!(instruction.start, Point::new(0, 0));
        assert_eq!(instruction.end, Point::new(999, 999));
    }
    #[test]
    fn first_test() {
        let mut first_exercise = SixthDay {
            exercise: Exercise {
                content: String::from(PROD),
                example: String::from(EXAMPLE),
            },
        };

        let expected_example = 998996;
        let expected_prod = 569999;

        let result_example = first_exercise.solve_first(false);
        let result_prod = first_exercise.solve_first(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);

        first_exercise.exercise.example =
            String::from("turn on 0,0 through 0,0\ntoggle 0,0 through 999,999\n");

        let expected_example = 2000001;
        let expected_prod = 17836115;
        let result_example = first_exercise.solve_second(false);
        let result_prod = first_exercise.solve_second(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);
    }
}
