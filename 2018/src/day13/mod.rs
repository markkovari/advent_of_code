






fn part1(_content: String) -> usize {
    2
}

fn part2(_content: String) -> usize {
    2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_first() {
        let example = include_str!("./example.data");
        let result = part1(example.to_string());
        assert_eq!(result, 325);

        let prod = include_str!("./prod.data");
        let result = part1(prod.to_string());
        assert_eq!(result, 1430);
    }

    #[test]
    fn test_second() {
        let example = include_str!("./example.data");
        let _ = part2(example.to_string());

        let prod = include_str!("./prod.data");
        let _ = part2(prod.to_string());
    }
}
