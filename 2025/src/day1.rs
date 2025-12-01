use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Rotation {
    Left(i32),
    Right(i32),
}

#[derive(Debug, Clone, Copy)]
struct Dial {
    position: i32,
}

impl Default for Dial {
    fn default() -> Self {
        Dial { position: 50 }
    }
}

impl Dial {
    pub fn rotate(&mut self, rotation: Rotation) -> u16 {
        let old_position = self.position;
        let angle = match rotation {
            Rotation::Left(x) => -x,
            Rotation::Right(x) => x,
        };
        self.position = (self.position + angle).rem_euclid(100);

        // Count how many times we pass through or land on 0
        // For right rotation: count multiples of 100 in (old_position, old_position + angle]
        // For left rotation: count multiples of 100 in [old_position + angle, old_position)
        let count = if angle >= 0 {
            (old_position + angle).div_euclid(100) - old_position.div_euclid(100)
        } else {
            (old_position - 1).div_euclid(100) - (old_position + angle - 1).div_euclid(100)
        };

        count as u16
    }

    pub fn is_zero(&self) -> bool {
        self.position == 0
    }
}

#[derive(Debug)]
pub enum ParseRotationError {
    InvalidDirectionLength,
    InvalidDirectionChar,
}

impl FromStr for Rotation {
    type Err = ParseRotationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseRotationError::InvalidDirectionLength);
        }
        let (dir_char, rotation_amount) = s.split_at(1);

        let rotation = rotation_amount
            .parse::<i32>()
            .map_err(|_| ParseRotationError::InvalidDirectionLength)?;

        match dir_char {
            "R" => Ok(Rotation::Right(rotation)),
            "L" => Ok(Rotation::Left(rotation)),
            _ => Err(ParseRotationError::InvalidDirectionChar),
        }
    }
}

pub fn parse_rotations(input: String) -> Option<Vec<Rotation>> {
    input
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<_>, _>>()
        .ok()
}

fn solve_1(rotations: Vec<Rotation>) -> u16 {
    let mut dial = Dial::default();
    let mut result: u16 = 0;
    for rotation in rotations.iter() {
        dial.rotate(*rotation);
        if dial.is_zero() {
            result += 1;
        }
    }
    result
}

fn solve_2(rotations: Vec<Rotation>) -> u16 {
    let mut dial = Dial::default();
    let mut result: u16 = 0;
    for rotation in rotations.iter() {
        result += dial.rotate(*rotation);
    }
    result
}

pub fn run() {
    let input = include_str!("1_input");

    if let Some(rotations) = parse_rotations(input.to_string()) {
        println!("Day 1 Part 1: {}", solve_1(rotations.clone()));
        println!("Day 1 Part 2: {}", solve_2(rotations));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn rotate_l68() {
        let mut dial = Dial::default();
        dial.rotate(Rotation::Left(68));
        assert_eq!(dial.position, 82);
    }

    #[test]
    pub fn rotate_l30() {
        let mut dial = Dial { position: 82 };
        dial.rotate(Rotation::Left(30));
        assert_eq!(dial.position, 52);
    }

    #[test]
    pub fn rotate_r48() {
        let mut dial = Dial { position: 52 };
        dial.rotate(Rotation::Right(48));
        assert_eq!(dial.position, 0);
    }

    #[test]
    pub fn rotate_l5() {
        let mut dial = Dial { position: 0 };
        dial.rotate(Rotation::Left(5));
        assert_eq!(dial.position, 95);
    }

    #[test]
    pub fn rotate_r60() {
        let mut dial = Dial { position: 95 };
        dial.rotate(Rotation::Right(60));
        assert_eq!(dial.position, 55);
    }

    #[test]
    pub fn rotate_l55() {
        let mut dial = Dial { position: 55 };
        dial.rotate(Rotation::Left(55));
        assert_eq!(dial.position, 0);
    }

    #[test]
    pub fn rotate_l1() {
        let mut dial = Dial { position: 0 };
        dial.rotate(Rotation::Left(1));
        assert_eq!(dial.position, 99);
    }

    #[test]
    pub fn rotate_l99() {
        let mut dial = Dial { position: 99 };
        dial.rotate(Rotation::Left(99));
        assert_eq!(dial.position, 0);
    }

    #[test]
    pub fn rotate_r14() {
        let mut dial = Dial { position: 0 };
        dial.rotate(Rotation::Right(14));
        assert_eq!(dial.position, 14);
    }

    #[test]
    pub fn rotate_l82() {
        let mut dial = Dial { position: 14 };
        dial.rotate(Rotation::Left(82));
        assert_eq!(dial.position, 32);
    }

    #[test]
    pub fn test_1_file() {
        let file_content = include_str!("1_input");
        if let Some(rotations) = parse_rotations(file_content.to_owned()) {
            let result = solve_1(rotations);
            assert_eq!(result, 1071);
        }
    }

    #[test]
    pub fn test_1_input() {
        let file_content = r#"L68
    L30
    R48
    L5
    R60
    L55
    L1
    L99
    R14
    L82"#;
        if let Some(rotations) = parse_rotations(file_content.to_owned()) {
            let result = solve_1(rotations);
            assert_eq!(result, 3);
        }
    }

    #[test]
    pub fn test_2_input() {
        let file_content = r#"L68
    L30
    R48
    L5
    R60
    L55
    L1
    L99
    R14
    L82"#;
        if let Some(rotations) = parse_rotations(file_content.to_owned()) {
            let result = solve_2(rotations);
            assert_eq!(result, 6);
        }
    }

    #[test]
    pub fn test_2_file() {
        let file_content = include_str!("1_input");
        if let Some(rotations) = parse_rotations(file_content.to_owned()) {
            let result = solve_2(rotations);
            assert_eq!(result, 6700);
        }
    }
}
