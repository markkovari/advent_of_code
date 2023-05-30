use crate::{Excercise, Solvable};

struct EleventhDay {
    exercise: Excercise,
}

fn increment(character: char) -> char {
    match character {
        'z' => 'a',
        _ => (character as u8 + 1) as char,
    }
}

fn has_two_non_overlapping_pairs(password: &mut String) -> bool {
    let mut pairs = 0;
    let mut index = 0;
    loop {
        if index >= password.len() - 1 {
            break;
        }
        let character = password.chars().nth(index).unwrap();
        let next_character = password.chars().nth(index + 1).unwrap();
        if character == next_character {
            pairs += 1;
            index += 2;
        } else {
            index += 1;
        }
    }
    pairs >= 2
}


fn is_valid_password(password: &mut String) -> bool {
    let mut triples = 0;
    for window in password.chars().collect::<Vec<char>>().windows(3) {
        if window[0] == 'i' || window[0] == 'o' || window[0] == 'l' {
            return false;
        }
        if window[1] == 'i' || window[1] == 'o' || window[1] == 'l' {
            return false;
        }
        if window[2] == 'i' || window[2] == 'o' || window[2] == 'l' {
            return false;
        }
        if window[0] as u8 + 1 == window[1] as u8 && window[1] as u8 + 1 == window[2] as u8 {
            triples += 1;
        }
    }
    if !has_two_non_overlapping_pairs(password) {
       return false;
    }

    triples >= 1
}

fn create_new_password(password: String) -> String {
    let mut password = password;
    loop {
        password = increment_password(password);
        if is_valid_password(&mut password) {
            break;
        }
    }
    password
}

fn increment_password(password: String) -> String {
    let mut password = password;
    let mut index = password.len() - 1;
    loop {
        let character = password.chars().nth(index).unwrap();
        let new_character = increment(character);
        password.replace_range(index..=index, &new_character.to_string());
        if new_character == 'a' {
            index -= 1;
        } else {
            break;
        }
    }
    password
}

impl EleventhDay {
    fn solve_first(&self, is_prod: bool) -> String {
        if is_prod {
            self.first(self.exercise.content.to_owned())
        } else {
            self.first(self.exercise.example.to_owned())
        }
    }

    fn solve_second(&self, is_prod: bool) -> String {
        if is_prod {
            self.second(self.exercise.content.to_owned())
        } else {
            self.second(self.exercise.example.to_owned())
        }
    }

    fn first(&self, content: String) -> String {
        create_new_password(content)
    }

    fn second(&self, content: String) -> String {
        "asd".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("11_test.txt");
    const PROD: &str = include_str!("11_prod.txt");

    #[test]
    fn test_valid_password() {
        let is_valid = is_valid_password(&mut "abcdffaa".to_owned());
        assert_eq!(is_valid, true);
    }

    #[test]
    fn first_test() {
        let mut first_excersise = EleventhDay {
            exercise: Excercise {
                content: String::from(PROD),
                example: String::from(EXAMPLE),
            },
        };

        let expected_example = "abcdffaa";
        let expected_prod = "hxbxxyzz";

        let result_example = first_excersise.solve_first(false);
        let result_prod = first_excersise.solve_first(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);

        let expected_prod = "hxcaabcc";
        let result_prod = create_new_password(result_prod);
        assert_eq!(expected_prod, result_prod);
    }
}
