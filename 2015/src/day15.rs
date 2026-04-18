use aoc_rust_common::Solution;
use rayon::prelude::*;
use regex::Regex;
use std::fmt::Display;

pub struct Day15;

#[derive(Clone)]
struct Ingredient {
    #[allow(dead_code)]
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
        let re = Regex::new(
            r"(?P<name>\w+): capacity (?P<capacity>-?\d+), durability (?P<durability>-?\d+), flavor (?P<flavor>-?\d+), texture (?P<texture>-?\d+), calories (?P<calories>-?\d+)",
        ).unwrap();
        let caps = re.captures(value).ok_or("No match")?;
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
    let mut cap = 0;
    let mut dur = 0;
    let mut fla = 0;
    let mut tex = 0;

    for (idx, ingredient) in ingredients.iter().enumerate() {
        cap += ingredient.capacity * ratios[idx];
        dur += ingredient.durability * ratios[idx];
        fla += ingredient.flavor * ratios[idx];
        tex += ingredient.texture * ratios[idx];
    }

    if cap <= 0 || dur <= 0 || fla <= 0 || tex <= 0 {
        return 0;
    }
    (cap * dur * fla * tex) as u64
}

fn cookie_calories(ingredients: &[Ingredient], ratios: &[i64]) -> i64 {
    ingredients
        .iter()
        .enumerate()
        .map(|(idx, ing)| ing.calories * ratios[idx])
        .sum()
}

impl Solution for Day15 {
    fn year(&self) -> u32 {
        2015
    }
    fn day(&self) -> u32 {
        15
    }

    fn part1(&self, input: &str) -> Box<dyn Display> {
        let ingredients: Vec<Ingredient> = input
            .lines()
            .map(|line| Ingredient::try_from(line).unwrap())
            .collect();
        if ingredients.len() == 2 {
            Box::new(
                (0..=100)
                    .map(|a| {
                        let b = 100 - a;
                        cookie_score(&ingredients, &[a, b])
                    })
                    .max()
                    .unwrap_or(0) as i64,
            )
        } else {
            let ings = ingredients.clone();
            Box::new(
                (0..=100)
                    .into_par_iter()
                    .flat_map(move |a| {
                        let ings = ings.clone();
                        (0..=100 - a).into_par_iter().flat_map(move |b| {
                            let ings = ings.clone();
                            (0..=100 - a - b).into_par_iter().map(move |c| {
                                let d = 100 - a - b - c;
                                cookie_score(&ings, &[a, b, c, d])
                            })
                        })
                    })
                    .max()
                    .unwrap_or(0) as i64,
            )
        }
    }

    fn part2(&self, input: &str) -> Box<dyn Display> {
        let ingredients: Vec<Ingredient> = input
            .lines()
            .map(|line| Ingredient::try_from(line).unwrap())
            .collect();
        if ingredients.len() == 2 {
            Box::new(
                (0..=100)
                    .filter_map(|a| {
                        let b = 100 - a;
                        let ratios = [a, b];
                        if cookie_calories(&ingredients, &ratios) == 500 {
                            Some(cookie_score(&ingredients, &ratios))
                        } else {
                            None
                        }
                    })
                    .max()
                    .unwrap_or(0) as i64,
            )
        } else {
            let ings = ingredients.clone();
            Box::new(
                (0..=100)
                    .into_par_iter()
                    .flat_map(move |a| {
                        let ings = ings.clone();
                        (0..=100 - a).into_par_iter().flat_map(move |b| {
                            let ings = ings.clone();
                            (0..=100 - a - b).into_par_iter().filter_map(move |c| {
                                let d = 100 - a - b - c;
                                let ratios = [a, b, c, d];
                                if cookie_calories(&ings, &ratios) == 500 {
                                    Some(cookie_score(&ings, &ratios))
                                } else {
                                    None
                                }
                            })
                        })
                    })
                    .max()
                    .unwrap_or(0) as i64,
            )
        }
    }
}
