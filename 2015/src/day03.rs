use std::collections::HashMap;

use crate::{Exercise, Solvable};

type Position = (i32, i32);

struct Santa {
    position: Position,
    visited: HashMap<Position, usize>,
}

impl Santa {
    pub fn new() -> Self {
        let mut visited = HashMap::new();
        visited.insert((0, 0), 1);
        Self {
            position: (0, 0),
            visited,
        }
    }
    pub fn move_to(&mut self, direction: char) {
        match direction {
            '>' => {
                self.position.0 += 1;
            }
            '<' => {
                self.position.0 -= 1;
            }
            '^' => {
                self.position.1 -= 1;
            }
            'v' => {
                self.position.1 += 1;
            }
            _ => {}
        }
        let counter = self.visited.entry(self.position).or_insert(0);
        *counter += 1;
    }

    pub fn get_visited_multiple(&self) -> usize {
        let visiteds = self
            .visited
            .values()
            .filter(|&x| *x >= 1)
            .collect::<Vec<_>>();
        visiteds.len()
    }
}

struct ThirdDay {
    exercise: Exercise,
}

fn get_visited_multiple_times(first: Santa, second: Santa) -> usize {
    let mut visited = HashMap::new();
    for (key, value) in first.visited.iter() {
        visited.insert(key, value);
    }
    for (key, value) in second.visited.iter() {
        visited.insert(key, value);
    }
    visited.values().filter(|&x| *x >= &1).count()
}

impl Solvable for ThirdDay {
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
        let mut santa = Santa::new();
        for direction in content.chars() {
            santa.move_to(direction);
        }
        santa.get_visited_multiple() as i64
    }

    fn second(&self, content: &str) -> i64 {
        let mut santa = Santa::new();
        let mut robo_santa = Santa::new();
        for direction in content.chars().enumerate() {
            if direction.0 % 2 == 0 {
                santa.move_to(direction.1);
            } else {
                robo_santa.move_to(direction.1);
            }
        }
        get_visited_multiple_times(santa, robo_santa) as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("inputs/3_test.txt");
    const PROD: &str = include_str!("inputs/3_prod.txt");

    #[test]
    fn first_test() {
        let first_exercise = ThirdDay {
            exercise: Exercise {
                content: String::from(PROD),
                example: String::from(EXAMPLE),
            },
        };

        let expected_example = 4;
        let expected_prod = 2572;

        let result_example = first_exercise.solve_first(false);
        let result_prod = first_exercise.solve_first(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);

        let expected_example = 3;
        let expected_prod = 2631;
        let result_example = first_exercise.solve_second(false);
        let result_prod = first_exercise.solve_second(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);
    }
}
