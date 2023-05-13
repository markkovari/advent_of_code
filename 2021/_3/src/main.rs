fn main() {
    let content = include_str!("../input.data");
    let lines = content.lines();
    let element_count = lines.clone().nth(0).unwrap().len();
    let line_count = lines.clone().count();
    let summed = lines
        .into_iter()
        .fold(vec![0; element_count], |mut acc, line| {
            acc.iter_mut()
                .enumerate()
                .map(|(index, &mut element)| {
                    element
                        + line
                            .chars()
                            .nth(index)
                            .unwrap()
                            .to_string()
                            .parse::<i32>()
                            .unwrap()
                })
                .collect::<Vec<i32>>()
        });
    let bigger_at_position: Vec<bool> = summed
        .iter()
        .map(|a| *a > (line_count / 2) as i32)
        .collect::<Vec<bool>>();

    let gamma_and_epsilon: (i32, i32) =
        bigger_at_position
            .iter()
            .enumerate()
            .fold((0, 0), |(gamma, epsilon), (index, bigger)| {
                let current_value = 2_i32.pow((element_count - index - 1) as u32);
                match bigger {
                    true => (gamma + current_value, epsilon),
                    _ => (gamma, epsilon + current_value),
                }
            });

    println!("{}", gamma_and_epsilon.0 * gamma_and_epsilon.1);
}
