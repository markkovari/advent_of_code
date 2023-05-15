use std::println;

struct Excersise {
    content: String,
    example: String,
}

trait Readable {
    fn get_example(&self) -> String;
    fn get_prod(&self) -> String;
}

impl Readable for Excersise {
    fn get_example(&self) -> String {
        self.example.clone()
    }
    fn get_prod(&self) -> String {
        self.content.clone()
    }
}

trait Solvable {
    fn first(&self, content: String) -> i32;
    fn solve_first(&self, is_prod: bool) -> i32;
    fn second(&self, content: String) -> i32;
    fn solve_second(&self, is_prod: bool) -> i32;
}

impl Solvable for Excersise {
    fn solve_first(&self, is_prod: bool) -> i32 {
        if is_prod {
            self.first(self.content.to_owned())
        } else {
            self.first(self.example.to_owned())
        }
    }

    fn solve_second(&self, is_prod: bool) -> i32 {
        if is_prod {
            self.second(self.content.to_owned())
        } else {
            self.second(self.example.to_owned())
        }
    }
    fn first(&self, content: String) -> i32 {
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

    fn second(&self, content: String) -> i32 {
        let mut result: i32 = 0;
        for (at, c) in content.chars().enumerate() {
            if c == '(' {
                result += 1;
            } else if c == ')' {
                result -= 1;
            }
            if result == -1 {
                return (at + 1) as i32;
            }
        }
        return (content.len() + 1) as i32;
    }
}

fn calculate_wrapper_needed_materials(line: &str) -> (i32, i32) {
    let dimensions = line
        .split("x")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let l = dimensions[0];
    let w = dimensions[1];
    let h = dimensions[2];
    let mut smallest = l * w;
    let mut smallest_perimeter = l + w;

    if w * h < smallest {
        smallest = w * h;
        smallest_perimeter = w + h;
    }
    if h * l < smallest {
        smallest = h * l;
        smallest_perimeter = h + l;
    }
    (
        2 * l * w + 2 * w * h + 2 * h * l + smallest,
        smallest_perimeter * 2 + l * w * h,
    )
}

fn solve_first_first(content: &str) -> (i32, i32) {
    let materials = content
        .lines()
        .map(calculate_wrapper_needed_materials)
        .collect::<Vec<_>>();
    let mut wrapper = 0;
    let mut ribbon = 0;
    for (w, r) in materials {
        wrapper += w;
        ribbon += r;
    }
    (wrapper, ribbon)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = include_str!("1_test.txt");
    const EXAMPLE_2: &str = include_str!("2_test.txt");
    const PROD_1: &str = include_str!("1_prod.txt");
    const PROD_2: &str = include_str!("2_prod.txt");

    #[test]
    fn first_test() {
        let first_excersise = Excersise {
            content: String::from(PROD_1),
            example: String::from(EXAMPLE_1),
        };

        let expected_example = 3;
        let expected_prod = 232;

        let result_example = first_excersise.solve_first(false);
        let result_prod = first_excersise.solve_first(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);

        let expected_example = 1;
        let expected_prod = 1783;
        let result_example = first_excersise.solve_second(false);
        let result_prod = first_excersise.solve_second(true);
        assert_eq!(expected_example, result_example);
        assert_eq!(expected_prod, result_prod);
    }

    #[test]
    fn first_second_test() {
        let (expected_wrapper, expected_ribbon) = (58, 34);
        let (wrapper, ribbon) = solve_first_first(&EXAMPLE_2);
        assert_eq!(expected_wrapper, wrapper);
        assert_eq!(expected_ribbon, ribbon);
    }

    #[test]
    fn first_second_prod() {
        let (expected_wrapper, expexted_ribbon) = (1588178, 3783758);
        let (wrapper, ribbon) = solve_first_first(&PROD_2);
        assert_eq!(expected_wrapper, wrapper);
        assert_eq!(expexted_ribbon, ribbon);
    }

    // #[test]
    // fn second_test() {
    //     let expected = 1;
    //     let result = solve_second(&EXAMPLE_1);
    //     assert_eq!(expected, result);
    // }

    // #[test]
    // fn second_prod() {
    //     let expected = 1783;
    //     let result = solve_second(&PROD_1);
    //     assert_eq!(expected, result);
    // }
}
