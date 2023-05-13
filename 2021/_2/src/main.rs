fn main() {
    let depth_and_width_aim: (i32, i32, i32) = include_str!("../input.txt")
        .lines()
        .into_iter()
        .filter_map(|line| {
            let parts = line.split(" ").collect::<Vec<&str>>();
            Some((
                parts.get(0).unwrap().chars().nth(0).unwrap(),
                parts.get(1).unwrap().parse::<i32>().unwrap(),
            ))
        })
        .fold((0, 0, 0), |(px, py, aim), (d, w)| match d {
            'f' => (px + w, py + aim * w, aim),
            'u' => (px, py, aim - w),
            'd' => (px, py, aim + w),
            _ => (px, py, aim),
        });
    println!("{:?}", depth_and_width_aim);
    println!("{:?}", depth_and_width_aim.0 * depth_and_width_aim.1);
}
