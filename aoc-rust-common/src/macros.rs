/// Generates standardized tests for an Advent of Code solution.
///
/// # Example
/// ```ignore
/// aoc_test!(Day01, "0", "5");
/// ```
#[macro_export]
macro_rules! aoc_test {
    ($day_struct:ident, $part1_want:expr, $part2_want:expr) => {
        #[cfg(test)]
        mod tests {
            use super::*;
            use $crate::Solution;

            #[test]
            fn test_part1() {
                let day = $day_struct;
                let input = $crate::input::get_input(day.year(), day.day());
                assert_eq!(day.part1(&input).unwrap(), $part1_want);
            }

            #[test]
            fn test_part2() {
                let day = $day_struct;
                let input = $crate::input::get_input(day.year(), day.day());
                assert_eq!(day.part2(&input).unwrap(), $part2_want);
            }
        }
    };
}
