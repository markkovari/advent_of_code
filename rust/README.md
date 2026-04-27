# 🦀 Rust Advent of Code

This directory contains Rust solutions for Advent of Code, organized as a Cargo workspace.

## 📁 Structure

Each year is a separate crate within the workspace, depending on a shared `common` library.

- **`common/`**: Shared utilities (input handling, geometry, BFS/Dijkstra algorithms).
- **`20XX/`**: Year-specific implementations.
- **`runner/`**: A master CLI tool to run any day from any year.

## 🚀 Standardized Day Template

Every day is implemented as a struct that satisfies the `Solution` trait:

```rust
use anyhow::Result;
use aoc_common::Solution;

pub struct DayXX;

impl Solution for DayXX {
    fn year(&self) -> u32 { 2023 }
    fn day(&self) -> u32 { XX }

    fn part1(&self, input: &str) -> Result<String> {
        Ok("result".to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        Ok("result".to_string())
    }
}

aoc_common::aoc_test!(DayXX);
```

## 🧪 Testing

We use `insta` for snapshot testing and a custom `aoc_test!` macro for regression testing.

### Local Execution
```bash
# Run all tests in the workspace
cargo test

# Run a specific year
cargo test -p aoc-rust-2023

# Run regression tests (including those ignored in CI)
cargo test -- --ignored
```

### CI Environment
In CI, heavy file-reading tests are marked as `#[ignore]` via the `advent_of_code_ci` flag to maintain pipeline speed.

## 🛠 Adding a New Year

1. Create a new folder: `mkdir -p 20XX/src`.
2. Initialize `Cargo.toml` with `aoc-rust-20XX` name and dependency on `../common`.
3. Add the folder to `rust/Cargo.toml` workspace members.
4. Implement your days in `src/dayXX.rs`.
5. Register them in `src/lib.rs` and `src/main.rs`.
