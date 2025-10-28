use crate::{Excercise, Solvable};

struct SixthDay {
    exercise: Excercise,
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

fn count_lit(grid: &Grid) -> i32 {
    let mut count = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            if grid[x][y] {
                count += 1;
            }
        }
    }
    count
}

fn count_brigthness(grid: &GridAmbient) -> i32 {
    let mut count: i32 = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            count += grid[x][y] as i32;
        }
    }
    count
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
    fn solve_first(&self, is_prod: bool) -> i32 {
        if is_prod {
            self.first(self.exercise.content.to_owned())
        } else {
            self.first(self.exercise.example.to_owned())
        }
    }

    fn solve_second(&self, is_prod: bool) -> i32 {
        if is_prod {
            self.second(self.exercise.content.to_owned())
        } else {
            self.second(self.exercise.example.to_owned())
        }
    }

    fn first(&self, content: String) -> i32 {
        let instructions = content
            .lines()
            .map(|line| Instruction::try_from(line).unwrap())
            .collect::<Vec<Instruction>>();
        let mut grid: Grid = [[false; 1000]; 1000];
        apply_instructions(instructions, &mut grid);
        count_lit(&grid)
    }

    fn second(&self, content: String) -> i32 {
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
    const EXAMPLE: &str = include_str!("6_test.txt");
    const PROD: &str = include_str!("6_prod.txt");

    #[test]
    fn instruction() {
        let instruction = Instruction::try_from("turn on 0,0 through 999,999").unwrap();
        assert_eq!(instruction.action, Action::On);
        assert_eq!(instruction.start, Point::new(0, 0));
        assert_eq!(instruction.end, Point::new(999, 999));
    }
    #[test]
    fn first_test() {
        let mut first_excersise = SixthDay {
            exercise: Excercise {
                content: String::from(PROD),
                example: String::from(EXAMPLE),
            },
        };

        let expected_example = 998996;
        let expected_prod = 569999;

        let result_example = first_excersise.solve_first(false);
        let result_prod = first_excersise.solve_first(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);

        first_excersise.exercise.example =
            String::from("turn on 0,0 through 0,0\ntoggle 0,0 through 999,999\n");

        let expected_example = 2000001;
        let expected_prod = 17836115;
        let result_example = first_excersise.solve_second(false);
        let result_prod = first_excersise.solve_second(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);
    }
}
