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

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("1_test.txt");
    const PROD: &str = include_str!("1_prod.txt");

    #[test]
    fn it_works_example() {
        let expected = 3;
        let result = solve_first(&EXAMPLE);
        assert_eq!(expected, result);
    }

    #[test]
    fn it_works_prod() {
        let expected = 232;
        let result = solve_first(&PROD);
        assert_eq!(expected, result);
    }
}
