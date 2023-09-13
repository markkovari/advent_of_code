#[derive(Debug, Default)]
struct Node {
    metadata: Vec<i32>,
    children: Vec<Node>,
    len: usize,
}

impl Node {
    fn from_flat(flat: Vec<i32>) -> Option<Node> {
        if flat.len() < 2 {
            return None;
        }

        let (child_count, meta_count) = (flat[0], flat[1]);
        let mut node = Node {
            len: 2,
            ..Node::default()
        };
        for _ in 0..child_count {
            let child = Node::from_flat(flat[node.len..].to_vec())?;
            node.len += child.len;
            node.children.push(child);
        }
        for _ in 0..meta_count {
            let meta = match flat.get(node.len) {
                None => return None,
                Some(&i) if i < 1 => None,
                Some(&i) => Some(i),
            };
            node.metadata.push(meta.unwrap());
            node.len += 1;
        }
        Some(node)
    }

    fn sum_all_metadata(&self) -> i32 {
        let mut sum = self.metadata.iter().cloned().sum();
        for child in &self.children {
            sum += child.sum_all_metadata();
        }
        sum
    }

    fn value(&self) -> i32 {
        if self.children.is_empty() {
            return self.metadata.iter().cloned().sum::<i32>();
        }

        let mut sum = 0;
        for &i in &self.metadata {
            let child = match self.children.get(i as usize - 1) {
                None => continue,
                Some(child) => child,
            };
            sum += child.value();
        }
        sum
    }
}

fn part1(content: String) -> usize {
    let flat = read_numbers(content);
    let root = Node::from_flat(flat.clone()).unwrap();
    root.sum_all_metadata() as usize
}

fn part2(content: String) -> usize {
    let flat = read_numbers(content);
    let root = Node::from_flat(flat.clone()).unwrap();
    root.value() as usize
}

fn read_numbers(content: String) -> Vec<i32> {
    content
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_first() {
        let example = include_str!("./example.data");
        let result: usize = part1(example.to_owned());
        assert_eq!(138, result);
        let prod = include_str!("./prod.data");
        let result: usize = part1(prod.to_owned());
        assert_eq!(41521, result);
    }

    #[test]
    #[ignore]
    fn test_second() {
        let example = include_str!("./example.data");
        let result: usize = part2(example.to_owned());
        assert_eq!(66, result);
        let prod = include_str!("./prod.data");
        let result: usize = part2(prod.to_owned());
        assert_eq!(19990, result);
    }
}
