fn main() {
    let content = include_str!("../input.txt");
    let numbers = content
        .lines()
        .filter_map(|a| a.parse::<i32>().ok())
        .collect::<Vec<i32>>();

    let increments_two = numbers
        .windows(2)
        .into_iter()
        .filter(|e| e.get(0).unwrap() < e.get(e.len() - 1).unwrap())
        .count();

    let increments_four = numbers
        .windows(4)
        .into_iter()
        .filter(|e| e.get(0).unwrap() < e.get(e.len() - 1).unwrap())
        .count();
    println!("{:?}", increments_two);
    println!("{:?}", increments_four);
}

// use std::fs;

// fn calculate_growings(numbers: Vec<u64>) -> u64 {
//     let mut count = 0;
//     for i in 0..(numbers.len() - 1) {
//         if numbers[i] < numbers[i + 1] {
//             count += 1;
//         }
//     }
//     count
// }

// fn calcualte_window_growings(numbers: Vec<u64>) -> u64 {
//     let mut count = 0;
//     for i in 0..(numbers.len() - 3) {
//         let window_sum: u64 = numbers[i] + numbers[i + 1] + numbers[i + 2];
//         let next_window_sum: u64 = numbers[i + 1] + numbers[i + 2] + numbers[i + 3];
//         if next_window_sum > window_sum {
//             count += 1;
//         }
//     }
//     count
// }

// fn main() {
//     let file_name = "input.txt";
//     let content = fs::read_to_string(file_name).expect("Something went wrong reading the file");
//     let depths: std::vec::Vec<u64> = content
//         .lines()
//         .filter_map(|line| line.trim().parse::<u64>().ok())
//         .collect::<Vec<u64>>();

//     println!("{}", calculate_growings(depths.clone()));
//     println!("{}", calcualte_window_growings(depths));
// }
