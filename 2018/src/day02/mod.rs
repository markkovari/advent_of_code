use std::{
    collections::{HashMap, HashSet},
    vec,
};

pub fn double(i: i32) -> i32 {
    i * 2
}

fn read_freqs(is_prod: bool) -> Vec<HashMap<char, usize>> {
    let mut content = include_str!("./example.data");
    if is_prod {
        content = include_str!("./prod.data");
    }
    content = content.trim();

    let mut freqs = vec![];
    for line in content.lines() {
        let mut freq = HashMap::new();
        for c in line.chars() {
            let counter = freq.entry(c).or_insert(0);
            *counter += 1;
        }
        freqs.push(freq);
    }
    freqs
}

fn get_with_occ(occ: &HashMap<char, usize>, n: usize) -> HashSet<char> {
    let mut set: HashSet<char> = HashSet::new();
    for (c, count) in occ.iter() {
        if *count == n {
            set.insert(*c);
        }
    }
    set
}

fn get_two_closest(of_ids: Vec<String>) -> [String; 2] {
    let mut closest = ["".to_string(), "".to_string()];
    let mut min = usize::max_value();
    for (i, id) in of_ids.iter().enumerate() {
        for (j, other) in of_ids.iter().enumerate() {
            if i == j {
                continue;
            }
            let mut diff = 0;
            for (c, o) in id.chars().zip(other.chars()) {
                if c != o {
                    diff += 1;
                }
            }
            if diff < min {
                min = diff;
                closest[0] = id.to_string();
                closest[1] = other.to_string();
            }
        }
    }
    closest
}

fn get_common_chars(a: &str, b: &str) -> String {
    let mut common = String::new();
    for (c, o) in a.chars().zip(b.chars()) {
        if c == o {
            common.push(c);
        }
    }
    common
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        let nums = read_freqs(false);
        let with_2 = nums.iter().filter(|e| e.values().any(|&v| v == 2)).count();
        let with_3 = nums.iter().filter(|e| e.values().any(|&v| v == 3)).count();

        assert_eq!(with_2 * with_3, 12);
        let nums = read_freqs(true);
        let with_2 = nums.iter().filter(|e| e.values().any(|&v| v == 2)).count();
        let with_3 = nums.iter().filter(|e| e.values().any(|&v| v == 3)).count();

        assert_eq!(with_2 * with_3, 5000);
    }

    #[test]
    fn test_second() {
        let [first, second] = get_two_closest(vec![
            "abcde".to_string(),
            "fghij".to_string(),
            "klmno".to_string(),
            "pqrst".to_string(),
            "fguij".to_string(),
            "axcye".to_string(),
            "wvxyz".to_string(),
        ]);
        let common = get_common_chars(&first, &second);

        assert_eq!(common, "fgij".to_owned());
        let ids = include_str!("prod.data")
            .trim()
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let [first, second] = get_two_closest(ids);
        let common = get_common_chars(&first, &second);

        assert_eq!(common, "ymdrchgpvwfloluktajxijsqb".to_owned());
    }
}
