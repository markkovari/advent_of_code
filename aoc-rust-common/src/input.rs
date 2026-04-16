use std::fs;

pub fn get_input(year: u32, day: u32) -> String {
    let day_padded = format!("{:02}", day);
    let day_unpadded = format!("{}", day);
    
    // Potential locations for input files
    let paths = [
        format!("{}/inputs/day{}.txt", year, day_padded),
        format!("{}/inputs/{}/prod.txt", year, day_unpadded),
        format!("{}/src/inputs/{}_prod.txt", year, day_unpadded),
        format!("{}/inputs/{}.txt", year, day_unpadded),
    ];

    for path in &paths {
        if let Ok(content) = fs::read_to_string(path) {
            return content.trim_end().to_string();
        }
    }

    panic!("Could not find input for year {} day {} in any of: {:?}", year, day, paths);
}
