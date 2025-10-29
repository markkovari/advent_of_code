use crate::{Exercise, Solvable};
use rayon::prelude::*;
use regex::Regex;

struct FifteenthDay {
    exercise: Exercise,
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

fn parse_ingredients(content: &str) -> Vec<Ingredient> {
    content
        .lines()
        .map(|line| Ingredient::try_from(line).unwrap())
        .collect()
}

fn best_cookie_score1(ingredients: &[Ingredient]) -> u64 {
    (1..100)
        .into_par_iter()
        .flat_map(|a| {
            (1..100 - a)
                .into_par_iter()
                .flat_map(move |b| {
                    (1..100 - (a + b))
                        .into_par_iter()
                        .map(move |c| {
                            let d = 100 - a - b - c;
                            cookie_score(ingredients, &[a, b, c, d])
                        })
                })
        })
        .max()
        .unwrap_or(0)
}

fn best_cookie_score2(ingredients: &[Ingredient]) -> u64 {
    (1..100i64)
        .into_par_iter()
        .flat_map(|a| {
            (1..100i64 - a)
                .into_par_iter()
                .flat_map(move |b| {
                    (1..100i64 - (a + b))
                        .into_par_iter()
                        .filter_map(move |c| {
                            let d = 100i64 - a - b - c;
                            let ratios = [a, b, c, d];
                            if cookie_calories(ingredients, &ratios) == 500 {
                                Some(cookie_score(ingredients, &ratios))
                            } else {
                                None
                            }
                        })
                })
        })
        .max()
        .unwrap_or(0)
}

fn cookie_calories(ingredients: &[Ingredient], ratios: &[i64]) -> i64 {
    let mut calories = 0;
    for (idx, ratio) in ratios.iter().enumerate() {
        calories += ingredients[idx].calories * ratio;
    }
    calories
}

impl Solvable for FifteenthDay {
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
        let ingredients = parse_ingredients(content);
        best_cookie_score1(&ingredients) as i64
    }

    fn second(&self, content: &str) -> i64 {
        let ingredients = parse_ingredients(content);
        best_cookie_score2(&ingredients) as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("inputs/15_test.txt");
    const PROD: &str = include_str!("inputs/15_prod.txt");

    #[test]
    fn parse_ingredients_fn() {
        let ingredients = parse_ingredients(EXAMPLE);
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
        let first_exercise = FifteenthDay {
            exercise: Exercise {
                content: String::from(PROD),
                example: String::from(EXAMPLE),
            },
        };

        // let expected_example = 62842880;
        let expected_prod = 13882464;
        // let result_example = first_exercise.solve_first(false);
        let result_prod = first_exercise.solve_first(true);
        // assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);

        // let expected_example = 57600000;
        let expected_prod = 11171160;
        // let result_example = first_exercise.solve_second(false);
        let result_prod = first_exercise.solve_second(true);
        // assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);
    }
}
