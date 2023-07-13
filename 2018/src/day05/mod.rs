fn part1(content: String) -> String {
    let mut local = content;
    loop {
        let len = local.len();
        local = react(local);
        if len == local.len() {
            break;
        }
    }

    local
}

fn part2(content: String) -> (Vec<char>, Vec<usize>) {
    let local = content;
    let mut remaining = Vec::new();
    let mut scores = Vec::new();
    for c in "abcdefghijklmnopqrstuvwxyz".chars() {
        let mut local = local.clone();
        local = local.replace(c, "");
        local = local.replace(c.to_ascii_uppercase(), "");
        loop {
            let len = local.len();
            local = react(local);
            if len == local.len() {
                break;
            }
        }
        remaining.push(c);
        scores.push(local.len());
    }

    (remaining, scores)
}

fn react(content: String) -> String {
    let mut local = content;
    let mut i = 0;
    while i < local.len() - 1 {
        let a = local.chars().nth(i).unwrap();
        let b = local.chars().nth(i + 1).unwrap();
        if a != b && a.to_ascii_lowercase() == b.to_ascii_lowercase() {
            local.remove(i);
            local.remove(i);
            if i > 0 {
                i = i.saturating_sub(1);
            }
        } else {
            i += 1;
        }
    }

    local.to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_first() {
        let example = include_str!("./example.data");
        let remaining = part1(example.to_owned());
        assert_eq!(10, remaining.len());
        let prod = include_str!("./prod.data");
        let remaining = part1(prod.to_owned());
        assert_eq!(10384, remaining.len());
    }

    #[test]
    #[ignore]
    fn test_second() {
        let example = include_str!("./example.data");
        let (remaining, scores) = part2(example.to_owned());
        let result = remaining
            .iter()
            .zip(scores.iter())
            .min_by_key(|(_, &v)| v)
            .unwrap();
        assert_eq!(4, *result.1);

        let example = include_str!("./prod.data");
        let (remaining, scores) = part2(example.to_owned());
        let result = remaining
            .iter()
            .zip(scores.iter())
            .min_by_key(|(_, &v)| v)
            .unwrap();
        assert_eq!(5412, *result.1);
    }
}
