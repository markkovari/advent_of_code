use std::collections::HashSet;

pub fn double(i: i32) -> i32 {
    i * 2
}

fn read_numbers(is_prod: bool) -> Vec<i32> {
    let mut content = include_str!("./example.data");
    if is_prod {
        content = include_str!("./prod.data");
    }
    content = content.trim();
    content.lines().map(|s| s.parse::<i32>().unwrap()).collect()
}

fn first_revisit(of_numbers: Vec<i32>) -> Option<i32> {
    let mut visited = HashSet::new();
    let mut current = 0;
    visited.insert(current);
    loop {
        for i in of_numbers.iter() {
            current += i;
            if visited.contains(&current) {
                return Some(current);
            }
            visited.insert(current);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        let nums = read_numbers(false);
        let sum: i32 = nums.iter().sum();

        assert_eq!(sum, 3);

        let nums = read_numbers(true);
        let sum: i32 = nums.iter().sum();

        assert_eq!(sum, 454);
    }

    #[test]
    fn test_second() {
        let example = first_revisit(read_numbers(false));
        let prod = first_revisit(read_numbers(true));
        assert_eq!(example, Some(2));
        assert_eq!(prod, Some(566));
    }
}
