use std::{collections::HashMap, ops::AddAssign};

use crate::{Excercise, Solvable};
use iter_tools::{dependency::itertools::Product, Itertools};
use regex::Regex;

struct FifteenthDay {
    exercise: Excercise,
}

struct Ingredient {
    name: String,
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

impl TryFrom<&str> for Ingredient {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let re: Regex = Regex::new(
            r"(?P<name>\w+): capacity (?P<capacity>-?\d+), durability (?P<durability>-?\d+), flavor (?P<flavor>-?\d+), texture (?P<texture>-?\d+), calories (?P<calories>-?\d+)",
        ).unwrap();
        let caps = re.captures(value).unwrap();
        let name = caps.name("name").unwrap().as_str().to_owned();
        let capacity = caps
            .name("capacity")
            .unwrap()
            .as_str()
            .parse::<i64>()
            .unwrap();
        let durability = caps
            .name("durability")
            .unwrap()
            .as_str()
            .parse::<i64>()
            .unwrap();
        let flavor = caps
            .name("flavor")
            .unwrap()
            .as_str()
            .parse::<i64>()
            .unwrap();
        let texture = caps
            .name("texture")
            .unwrap()
            .as_str()
            .parse::<i64>()
            .unwrap();
        let calories = caps
            .name("calories")
            .unwrap()
            .as_str()
            .parse::<i64>()
            .unwrap();
        Ok(Ingredient {
            name,
            capacity,
            durability,
            flavor,
            texture,
            calories,
        })
    }
}
fn cookie_score(ingredients: &[Ingredient], ratios: &[i64]) -> u64 {
    let mut scores: Vec<i64> = vec![0, 0, 0, 0];
    assert_eq!(ingredients.len(), ratios.len());

    for (idx, ingredient) in ingredients.iter().enumerate() {
        scores[0] += ingredient.capacity * ratios[idx];
        scores[1] += ingredient.durability * ratios[idx];
        scores[2] += ingredient.flavor * ratios[idx];
        scores[3] += ingredient.texture * ratios[idx];
    }
    use std::iter::Product;
    Product::product(scores.iter().map(|s| if *s > 0 { *s as u64 } else { 0 }))
}

fn parse_ingredients(content: String) -> Vec<Ingredient> {
    content
        .lines()
        .map(|line| Ingredient::try_from(line).unwrap())
        .collect()
}

fn best_cookie_score1(ingredients: &[Ingredient]) -> u64 {
    let mut best = 0;
    for a in 1..100 {
        for b in 1..100 - a {
            for c in 1..100 - (a + b) {
                let d = 100 - a - b - c;
                let score = cookie_score(ingredients, &[a, b, c, d]);
                if score > best {
                    best = score
                }
            }
        }
    }
    best
}

fn best_cookie_score2(ingredients: &[Ingredient]) -> u64 {
    let mut best = 0;
    for a in 1..100i64 {
        for b in 1..100i64 - a {
            for c in 1..100i64 - (a + b) {
                let d = 100i64 - a - b - c;
                let score = cookie_score(ingredients, &[a, b, c, d]);
                if score > best && cookie_calories(ingredients, &[a, b, c, d]) == 500 {
                    best = score
                }
            }
        }
    }
    best
}

fn cookie_calories(ingredients: &[Ingredient], ratios: &[i64]) -> i64 {
    let mut calories = 0;
    for (idx, ratio) in ratios.iter().enumerate() {
        calories += ingredients[idx].calories * ratio;
    }
    calories
}

impl FifteenthDay {
    fn solve_first(&self, is_prod: bool) -> i64 {
        if is_prod {
            self.first(self.exercise.content.to_owned())
        } else {
            self.first(self.exercise.example.to_owned())
        }
    }

    fn solve_second(&self, is_prod: bool) -> i64 {
        if is_prod {
            self.second(self.exercise.content.to_owned())
        } else {
            self.second(self.exercise.example.to_owned())
        }
    }

    fn first(&self, content: String) -> i64 {
        let ingredients = parse_ingredients(content);
        best_cookie_score1(&ingredients) as i64
    }

    fn second(&self, content: String) -> i64 {
        let ingredients = parse_ingredients(content);
        best_cookie_score2(&ingredients) as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("15_test.txt");
    const PROD: &str = include_str!("15_prod.txt");

    #[test]
    fn parse_ingredients_fn() {
        let ingredients = parse_ingredients(String::from(EXAMPLE));
        assert_eq!(ingredients.len(), 2);
        assert_eq!(ingredients[0].name, "Butterscotch");
        assert_eq!(ingredients[0].capacity, -1);
        assert_eq!(ingredients[0].durability, -2);
        assert_eq!(ingredients[0].flavor, 6);
        assert_eq!(ingredients[0].texture, 3);
        assert_eq!(ingredients[0].calories, 8);

        assert_eq!(ingredients[1].name, "Cinnamon");
        assert_eq!(ingredients[1].capacity, 2);
        assert_eq!(ingredients[1].durability, 3);
        assert_eq!(ingredients[1].flavor, -2);
        assert_eq!(ingredients[1].texture, -1);
        assert_eq!(ingredients[1].calories, 3);
    }

    #[test]
    fn first_test() {
        let mut first_excersise = FifteenthDay {
            exercise: Excercise {
                content: String::from(PROD),
                example: String::from(EXAMPLE),
            },
        };

        // let expected_example = 62842880;
        let expected_prod = 13882464;
        // let result_example = first_excersise.solve_first(false);
        let result_prod = first_excersise.solve_first(true);
        // assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);

        // let expected_example = 57600000;
        let expected_prod = 11171160;
        // let result_example = first_excersise.solve_second(false);
        let result_prod = first_excersise.solve_second(true);
        // assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);
    }
}
