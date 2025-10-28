pub fn first(content: String) -> i32 {
    // Solution for part 1 of Day 26
    0
}

pub fn solve_first(is_prod: bool) -> i32 {
    let content = if is_prod {
        include_str!("./inputs/26_prod.txt")
    } else {
        include_str!("./inputs/26_test.txt")
    };
    first(content.to_string())
}

pub fn second(content: String) -> i32 {
    // Solution for part 2 of Day 26
    0
}

pub fn solve_second(is_prod: bool) -> i32 {
    let content = if is_prod {
        include_str!("./inputs/26_prod.txt")
    } else {
        include_str!("./inputs/26_test.txt")
    };
    second(content.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_26_first() {
        assert_eq!(solve_first(false), 0);
    }

    #[test]
    fn test_26_second() {
        assert_eq!(solve_second(false), 0);
    }
}