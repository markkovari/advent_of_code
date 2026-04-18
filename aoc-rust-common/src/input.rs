use std::env;
use std::fs;

pub fn get_input(year: u32, day: u32) -> String {
    let day_padded = format!("{:02}", day);
    let day_unpadded = format!("{}", day);

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());

    let paths_to_try = [
        // aoc_rust specific path
        format!("{}/inputs/{}/prod.txt", manifest_dir, day_unpadded),
        // General paths relative to manifest dir
        format!("{}/{}/inputs/day{}.txt", manifest_dir, year, day_padded),
        format!("{}/{}/inputs/{}/prod.txt", manifest_dir, year, day_unpadded),
        format!(
            "{}/{}/src/inputs/{}_prod.txt",
            manifest_dir, year, day_unpadded
        ),
        format!("{}/{}/inputs/{}.txt", manifest_dir, year, day_unpadded),
        format!("{}/inputs/{}.txt", manifest_dir, day_padded),
        format!("{}/inputs/day{}.txt", manifest_dir, day_padded),
        // General paths relative to current working dir
        format!("{}/inputs/day{}.txt", year, day_padded),
        format!("{}/inputs/{}/prod.txt", year, day_unpadded),
        format!("{}/src/inputs/{}_prod.txt", year, day_unpadded),
        format!("{}/inputs/{}.txt", year, day_unpadded),
        format!("inputs/{}.txt", day_padded),
    ];

    for path in &paths_to_try {
        if let Ok(content) = fs::read_to_string(path) {
            return content.trim_end().to_string();
        }
    }

    panic!(
        "Could not find input for year {} day {} in any of the checked paths.",
        year, day
    );
}
