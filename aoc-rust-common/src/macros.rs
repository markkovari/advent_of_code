/// Generates standardized tests for an Advent of Code solution, including snapshot tests.
///
/// # Example
/// ```ignore
/// aoc_test!(Day01);
/// ```
#[macro_export]
macro_rules! aoc_test {
    ($day_struct:ident) => {
        #[cfg(test)]
        mod tests {
            use super::*;
            use $crate::Solution;

            #[test]
            fn test_part1_snapshot() {
                let day = $day_struct;
                let input = $crate::input::get_input(day.year(), day.day());
                let result = day.part1(&input).unwrap();
                $crate::insta::assert_snapshot!(format!("{}_part1", stringify!($day_struct).to_lowercase()), result);
            }

            #[test]
            fn test_part2_snapshot() {
                let day = $day_struct;
                let input = $crate::input::get_input(day.year(), day.day());
                let result = day.part2(&input).unwrap();
                $crate::insta::assert_snapshot!(format!("{}_part2", stringify!($day_struct).to_lowercase()), result);
            }
        }
    };
    ($day_struct:ident, $part1_want:expr, $part2_want:expr) => {
        #[cfg(test)]
        mod tests {
            use super::*;
            use $crate::Solution;

            #[test]
            fn test_part1_regression() {
                let day = $day_struct;
                let input = $crate::input::get_input(day.year(), day.day());
                assert_eq!(day.part1(&input).unwrap(), $part1_want);
            }

            #[test]
            fn test_part2_regression() {
                let day = $day_struct;
                let input = $crate::input::get_input(day.year(), day.day());
                assert_eq!(day.part2(&input).unwrap(), $part2_want);
            }

            #[test]
            fn test_part1_snapshot() {
                let day = $day_struct;
                let input = $crate::input::get_input(day.year(), day.day());
                let result = day.part1(&input).unwrap();
                $crate::insta::assert_snapshot!(format!("{}_part1", stringify!($day_struct).to_lowercase()), result);
            }

            #[test]
            fn test_part2_snapshot() {
                let day = $day_struct;
                let input = $crate::input::get_input(day.year(), day.day());
                let result = day.part2(&input).unwrap();
                $crate::insta::assert_snapshot!(format!("{}_part2", stringify!($day_struct).to_lowercase()), result);
            }
        }
    };
}
