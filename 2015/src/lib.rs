use std::println;

pub fn solve_first(content: &str) -> i32 {
    let mut result: i32 = 0;
    for c in content.chars() {
        if c == '(' {
            result += 1;
        } else if c == ')' {
            result -= 1;
        }
    }
    result
}

fn calculate_wrapper_needed_paper_surface(line: &str) -> i32 {
    let dimensions = line
        .split("x")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let l = dimensions[0];
    let w = dimensions[1];
    let h = dimensions[2];
    let mut smallest = l * w;
    if w * h < smallest {
        smallest = w * h;
    }
    if h * l < smallest {
        smallest = h * l;
    }
    2 * l * w + 2 * w * h + 2 * h * l + smallest
}

fn solve_first_first(content: &str) -> i32 {
    content
        .lines()
        .map(calculate_wrapper_needed_paper_surface)
        .sum()
}

pub fn solve_second(content: &str) -> usize {
    let mut result: i32 = 0;
    for (at, c) in content.chars().enumerate() {
        if c == '(' {
            result += 1;
        } else if c == ')' {
            result -= 1;
        }
        if result == -1 {
            return at + 1;
        }
    }
    return content.len() + 1;
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = include_str!("1_test.txt");
    const EXAMPLE_2: &str = include_str!("2_test.txt");
    const PROD_1: &str = include_str!("1_prod.txt");
    const PROD_2: &str = include_str!("2_prod.txt");

    #[test]
    fn first_test() {
        let expected = 3;
        let result = solve_first(&EXAMPLE_1);
        assert_eq!(expected, result);
    }

    #[test]
    fn first_second_test() {
        let expected = 58;
        let result = solve_first_first(&EXAMPLE_2);
        assert_eq!(expected, result);
    }

    #[test]
    fn first_prod() {
        let expected = 232;
        let result = solve_first(&PROD_1);
        assert_eq!(expected, result);
    }

    #[test]
    fn first_second_prod() {
        let expected = 1588178;
        let result = solve_first_first(&PROD_2);
        assert_eq!(expected, result);
    }

    #[test]
    fn second_test() {
        let expected = 1;
        let result = solve_second(&EXAMPLE_1);
        assert_eq!(expected, result);
    }

    #[test]
    fn second_prod() {
        let expected = 1783;
        let result = solve_second(&PROD_1);
        assert_eq!(expected, result);
    }
}
