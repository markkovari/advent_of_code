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
    const EXAMPLE: &str = include_str!("1_test.txt");
    const PROD: &str = include_str!("1_prod.txt");

    #[test]
    fn first_test() {
        let expected = 3;
        let result = solve_first(&EXAMPLE);
        assert_eq!(expected, result);
    }

    #[test]
    fn first_prod() {
        let expected = 232;
        let result = solve_first(&PROD);
        assert_eq!(expected, result);
    }

    #[test]
    fn second_test() {
        let expected = 1;
        let result = solve_second(&EXAMPLE);
        assert_eq!(expected, result);
    }

    #[test]
    fn second_prod() {
        let expected = 1783;
        let result = solve_second(&PROD);
        assert_eq!(expected, result);
    }
}
