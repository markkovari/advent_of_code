use std::str::FromStr;
use std::error::Error;


#[derive(Debug)]
enum Rotation {
    Left(i32),
    Right(i32),
}

#[derive(Debug)]
struct Dial {
    position: i32,
}

impl Default for Dial {
    fn default() -> Self {
        Dial { position: 50 }
    }
}


impl Dial {
    pub fn rotate(&mut self, _rotation: Rotation) {
        self.position;
    }

    pub fn isZero(self) -> bool {
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

    // Required method
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseRotationError::InvalidDirectionLength);
        }
        let (dir_char, rotation_amount) = s.split_at(1);

        let rotation: i32 = rotation_amount.parse()?;
        match dir_char {
            "R" => Rotation { Right: rotation },
            "L" => Rotation { Left: rotation },
            _ => Err(ParseRotationError::InvalidDirectionChar),
        }
    }
}

pub fn parse_rotations(input: String) -> Option<Vec<Rotation>> {
    input.lines().map(|l| l.parse())
}

fn main() {
    let initial_text = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;

    if let Ok(rotations) = parse_rotations(initial_text.to_string()) {
        println!("{:?}", rotations);
    }

    println!("Hello, world!");
}
