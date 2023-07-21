use std::error::Error;
use std::io::{self, Write};
use std::result;

type Result<T> = result::Result<T, Box<dyn Error>>;

fn part1(recipe_count: usize) -> Result<usize> {
    let mut recipes = Recipes::new();
    while recipes.scores.len() < recipe_count + 10 {
        recipes.step();
    }

    Ok(recipes.scores[recipe_count..recipe_count + 10]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .concat()
        .parse::<usize>()
        .unwrap())
}

fn part2(digits: &[u32]) -> Result<usize> {
    let mut recipes = Recipes::new();
    let ends_at;
    loop {
        if recipes.scores.ends_with(&digits) {
            ends_at = recipes.scores.len() - digits.len();
            break;
        } else if recipes.scores[..recipes.scores.len() - 1].ends_with(&digits) {
            ends_at = recipes.scores.len() - digits.len() - 1;
            break;
        }
        recipes.step();
    }

    Ok(ends_at)
}

#[derive(Clone, Debug)]
struct Recipes {
    elves: Vec<usize>,
    scores: Vec<u32>,
}

impl Recipes {
    fn new() -> Recipes {
        Recipes {
            scores: vec![3, 7],
            elves: vec![0, 1],
        }
    }

    fn step(&mut self) {
        let new_recipe: u32 = self.elves.iter().map(|&e| self.scores[e]).sum();
        for &digit in new_recipe.to_string().as_bytes() {
            let digit_value = digit - b'0';
            self.scores.push(digit_value as u32);
        }
        for e in &mut self.elves {
            *e = (*e + self.scores[*e] as usize + 1) % self.scores.len();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_first() {
        let amount: usize = include_str!("./example.data").parse().unwrap();
        let result = part1(amount).unwrap();
        assert_eq!(result, 5158916779);

        let prod: usize = include_str!("./prod.data").parse().unwrap();
        let result = part1(prod).unwrap();
        assert_eq!(result, 1221283494);
    }

    #[test]
    #[ignore]
    fn test_second() {
        // let amount: usize = include_str!("./example.data").parse().unwrap();
        // let result = part2(&[3, 7]).unwrap();
        // assert_eq!(result, 20291131);

        let prod: usize = include_str!("./prod.data").parse().unwrap();
        let result = part2(&[6, 5, 2, 6, 0, 1]).unwrap();
        assert_eq!(result, 20261485);
    }
}
