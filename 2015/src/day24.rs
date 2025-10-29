use std::cmp;

use iter_tools::Itertools;
use rayon::prelude::*;

use crate::Exercise;

struct TwentyFourthDay {
    exercise: Exercise,
}

fn weight(packages: &[u32]) -> u32 {
    packages.iter().sum()
}

fn weight_ref(packages: &[&u32]) -> u32 {
    packages.iter().map(|&&p| p).sum()
}

fn quantum_entanglement(packages: &[&u32]) -> u64 {
    packages.iter().fold(1u64, |a, &b| a * (*b as u64))
}

fn find_quantum_entanglement_of_best_group(packages: &[u32], amount_of_groups: u32) -> u64 {
    let ideal_weight = weight(packages) / amount_of_groups;

    for length in 1..=packages.len() {
        let best = packages
            .iter()
            .permutations(length)
            .par_bridge()
            .filter(|packages| weight_ref(packages) == ideal_weight)
            .map(|combination| quantum_entanglement(&combination))
            .min();

        println!("Len: {}, best so far: {:?}", length, best);

        if let Some(qe) = best {
            return qe;
        }
    }

    u64::MAX
}

impl TwentyFourthDay {
    fn solve_first(&self, input: &str) -> usize {
        self.first(input)
    }

    fn solve_second(&self, input: &str) -> usize {
        self.second(input)
    }

    fn first(&self, input: &str) -> usize {
        let packages = input
            .lines()
            .map(|l| l.parse().unwrap())
            .collect::<Vec<_>>();

        find_quantum_entanglement_of_best_group(&packages, 3) as usize
    }

    fn second(&self, input: &str) -> usize {
        let packages = input
            .lines()
            .map(|l| l.parse().unwrap())
            .collect::<Vec<_>>();

        find_quantum_entanglement_of_best_group(&packages, 4) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("inputs/24_prod.txt");
    const PROD: &str = include_str!("inputs/24_prod.txt");

    #[test]
    #[ignore = "Takes too long"]
    fn first_test() {
        let mut first_exercise = TwentyFourthDay {
            exercise: Exercise {
                content: String::from(PROD),
                example: String::from(EXAMPLE),
            },
        };

        let expected_example = 10439961859;
        let expected_prod = 10439961859;
        let result_example = first_exercise.first(PROD);
        let result_prod = first_exercise.first(PROD);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);

        let expected_example = 72050269;
        let expected_prod = 72050269;
        let result_example = first_exercise.second(PROD);
        let result_prod = first_exercise.second(PROD);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);
    }
}
