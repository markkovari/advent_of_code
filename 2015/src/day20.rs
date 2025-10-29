use crate::{Exercise, Solvable};

struct TwentiethDay {
    exercise: Exercise,
}

fn presents_at1(house_nr: usize) -> usize {
    divisors(house_nr).into_iter().sum::<usize>() * 10
}

fn presents_at2(until: usize) -> usize {
    divisors(until)
        .into_iter()
        .filter(|d| until / d <= 50)
        .sum::<usize>()
        * 11
}

fn divisors(until: usize) -> Vec<usize> {
    let small_divisors: Vec<usize> = (1..((until as f64).sqrt() as usize + 1))
        .filter(|i| until.is_multiple_of(*i))
        .collect();

    let large_divisors: Vec<usize> = small_divisors
        .iter()
        .filter(|d| until != **d * **d)
        .map(|d| until / d)
        .collect();
    [small_divisors, large_divisors].concat()
}

impl Solvable for TwentiethDay {
    fn solve_first(&self, _is_prod: bool) -> i64 {
        self.first("36000000")
    }

    fn solve_second(&self, _is_prod: bool) -> i64 {
        self.second("36000000")
    }

    fn first(&self, input: &str) -> i64 {
        let target: usize = input.trim().parse().unwrap_or(36000000);
        for house_nr in 1..usize::MAX {
            if presents_at1(house_nr) >= target {
                return house_nr as i64;
            }
        }
        0
    }

    fn second(&self, input: &str) -> i64 {
        let target: usize = input.trim().parse().unwrap_or(36000000);
        for house_nr in 1..usize::MAX {
            if presents_at2(house_nr) >= target {
                return house_nr as i64;
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("inputs/19_test.txt");
    const PROD: &str = include_str!("inputs/19_prod.txt");

    #[test]
    fn first_test() {
        let first_exercise = TwentiethDay {
            exercise: Exercise {
                content: String::from(PROD),
                example: String::from(EXAMPLE),
            },
        };

        let expected_example = 4;
        let expected_prod = 831600;
        let result_example = first_exercise.first("70");
        let result_prod = first_exercise.first("36000000");
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);

        let expected_example = 4;
        let expected_prod = 884520;
        let result_example = first_exercise.second("70");
        let result_prod = first_exercise.second("36000000");
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);
    }
}
