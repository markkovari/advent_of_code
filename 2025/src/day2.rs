use std::str::FromStr;

type Id = u64;

#[derive(Debug, Clone, Copy)]
struct Range {
    lower: Id,
    upper: Id,
}

fn is_double(n: Id) -> bool {
    let s = n.to_string();
    let len = s.len();
    if len % 2 != 0 {
        return false;
    }

    let mid = len / 2;
    &s[..mid] == &s[mid..]
}

impl Range {
    pub fn get_doubles(&self) -> Vec<Id> {
        (self.lower..=self.upper).into_iter().filter(|&e|is_double(e)).collect()
    }
}

#[derive(Debug)]
enum ParseRangeError {
    CannotParseRange,
    NotExactlyTwoNumbers
}

impl FromStr for Range {
    type Err = ParseRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split('-').collect::<Vec<&str>>();
        if parts.len() != 2  {
            return Err(ParseRangeError::NotExactlyTwoNumbers);
        }

        if let [lower, upper] = parts.as_slice() {
            match ( lower.parse::<Id>(), upper.parse::<Id>()) {
                (Ok(lower_number), Ok(upper_number)) => {
                    return Ok(Range { lower: lower_number, upper: upper_number});
                },
                _ => Err(ParseRangeError::CannotParseRange)
            }
        } else {
            return Err(ParseRangeError::CannotParseRange)
        }
    }

}

fn parse_ranges(input: &str) -> Option<Vec<Range>> {
    input
    .trim()
    .split(',')
    .map(|line|line.parse())
    .collect::<Result<Vec<_>, _>>()
    .ok()
}

fn solve_1(ranges: &Vec<Range>) -> Id {
    ranges.into_iter().map(|range| range.get_doubles().iter().sum::<Id>()).sum()
}

pub fn run() {
    let input = include_str!("2_input");

    if let Some(ranges) = parse_ranges(input) {
        println!("Day 2 Part 1: {}", solve_1(&ranges));
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_double_valid() {
        assert!(is_double(11));
        assert!(is_double(22));
        assert!(is_double(99));
        assert!(is_double(1010));
        assert!(is_double(6464));
        assert!(is_double(123123));
        assert!(is_double(222222));
        assert!(is_double(446446));
        assert!(is_double(1188511885));
        assert!(is_double(38593859));
    }

    #[test]
    fn test_is_double_invalid() {
        assert!(!is_double(101)); // Not a double, has leading zero if split
        assert!(!is_double(100));
        assert!(!is_double(123));
        assert!(!is_double(1234));
        assert!(!is_double(98));
        assert!(!is_double(1011));
    }

    #[test]
    fn test_is_double_odd_length() {
        assert!(!is_double(1));
        assert!(!is_double(111));
        assert!(!is_double(12345));
    }

    #[test]
    fn test_range_11_22() {
        let range = Range { lower: 11, upper: 22 };
        assert_eq!(range.get_doubles(), vec![11, 22]);
    }

    #[test]
    fn test_range_95_115() {
        let range = Range { lower: 95, upper: 115 };
        assert_eq!(range.get_doubles(), vec![99]);
    }

    #[test]
    fn test_range_998_1012() {
        let range = Range { lower: 998, upper: 1012 };
        assert_eq!(range.get_doubles(), vec![1010]);
    }

    #[test]
    fn test_range_1188511880_1188511890() {
        let range = Range { lower: 1188511880, upper: 1188511890 };
        assert_eq!(range.get_doubles(), vec![1188511885]);
    }

    #[test]
    fn test_range_222220_222224() {
        let range = Range { lower: 222220, upper: 222224 };
        assert_eq!(range.get_doubles(), vec![222222]);
    }

    #[test]
    fn test_range_1698522_1698528() {
        let range = Range { lower: 1698522, upper: 1698528 };
        assert_eq!(range.get_doubles(), Vec::<Id>::new());
    }

    #[test]
    fn test_range_446443_446449() {
        let range = Range { lower: 446443, upper: 446449 };
        assert_eq!(range.get_doubles(), vec![446446]);
    }

    #[test]
    fn test_range_38593856_38593862() {
        let range = Range { lower: 38593856, upper: 38593862 };
        assert_eq!(range.get_doubles(), vec![38593859]);
    }

    #[test]
    fn test_range_565653_565659() {
        let range = Range { lower: 565653, upper: 565659 };
        assert_eq!(range.get_doubles(), Vec::<Id>::new());
    }

    #[test]
    fn test_range_824824821_824824827() {
        let range = Range { lower: 824824821, upper: 824824827 };
        assert_eq!(range.get_doubles(), Vec::<Id>::new());
    }

    #[test]
    fn test_range_2121212118_2121212124() {
        let range = Range { lower: 2121212118, upper: 2121212124 };
        assert_eq!(range.get_doubles(), Vec::<Id>::new());
    }

    #[test]
    fn test_parse_ranges() {
        let input = "11-22,95-115,998-1012";
        let ranges = parse_ranges(input).unwrap();
        assert_eq!(ranges.len(), 3);
        assert_eq!(ranges[0].lower, 11);
        assert_eq!(ranges[0].upper, 22);
        assert_eq!(ranges[1].lower, 95);
        assert_eq!(ranges[1].upper, 115);
    }

    #[test]
    fn test_example_sum() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let ranges = parse_ranges(input).unwrap();
        let sum: u64 = ranges.iter()
            .flat_map(|r| (r.lower..=r.upper).filter(|&n| is_double(n)))
            .map(|n| n as u64)
            .sum();
        assert_eq!(sum, 1227775554);
    }
}
