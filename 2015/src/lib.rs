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

fn calculate_wrapper_needed_materials(line: &str) -> (i32, i32) {
    let dimensions = line
        .split("x")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let l = dimensions[0];
    let w = dimensions[1];
    let h = dimensions[2];
    let mut smallest = l * w;
    let mut smallest_perimeter = l + w;

    if w * h < smallest {
        smallest = w * h;
        smallest_perimeter = w + h;
    }
    if h * l < smallest {
        smallest = h * l;
        smallest_perimeter = h + l;
    }
    (
        2 * l * w + 2 * w * h + 2 * h * l + smallest,
        smallest_perimeter * 2 + l * w * h,
    )
}

fn solve_first_first(content: &str) -> (i32, i32) {
    let materials = content
        .lines()
        .map(calculate_wrapper_needed_materials)
        .collect::<Vec<_>>();
    let mut wrapper = 0;
    let mut ribbon = 0;
    for (w, r) in materials {
        wrapper += w;
        ribbon += r;
    }
    (wrapper, ribbon)
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
        let (expected_wrapper, expected_ribbon) = (58, 34);
        let (wrapper, ribbon) = solve_first_first(&EXAMPLE_2);
        assert_eq!(expected_wrapper, wrapper);
        assert_eq!(expected_ribbon, ribbon);
    }

    #[test]
    fn first_prod() {
        let expected = 232;
        let result = solve_first(&PROD_1);
        assert_eq!(expected, result);
    }

    #[test]
    fn first_second_prod() {
        let (expected_wrapper, expexted_ribbon) = (1588178, 3783758);
        let (wrapper, ribbon) = solve_first_first(&PROD_2);
        assert_eq!(expected_wrapper, wrapper);
        assert_eq!(expexted_ribbon, ribbon);
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
