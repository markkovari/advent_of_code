# Repository Overview

This repository contains solutions to Advent of Code problems, organized by year. Each year typically has its own directory, and within those directories, solutions may be implemented in various programming languages.

## Project Structure

- **`YYYY/`**: Directories for each Advent of Code year (e.g., `2015/`, `2022/`).
- **`YYYY/<language>/`**: Within each year, there might be subdirectories for different programming languages used for solutions (e.g., `2024/golang/`, `2022/src/`).
- **`YYYY/<day>/` or `YYYY/src/_<day>/`**: Individual problem solutions are often found within day-specific directories or files.
- **`inputs/`**: Some years include an `inputs` directory for problem input data.

## Languages Used

The repository currently contains solutions implemented in:
- Rust (e.g., `2022/`, `2021/_7/Cargo.toml`)
- C# (e.g., `2021/cs/1/1.csproj`)
- Go (e.g., `2024/golang/go.mod`, `2019/go/go.mod`)
- TypeScript (e.g., `2021/_7/index.ts`)
- Zig (e.g., `2024/zig/.gitignore`)

## Essential Commands

Due to the polyglot nature of this repository, build and test commands are specific to each language and year.

### Rust Projects

For Rust projects (identified by `Cargo.toml`):
- **Build**: `cargo build`
- **Run**: `cargo run`
- **Test**: `cargo test`

Example (for 2022 solutions):
```bash
cd 2022
cargo run --bin _1 -- input.data
cargo test
```

### Go Projects

For Go projects (identified by `go.mod`):
- **Run**: `go run .` or `go run main.go` (if `main.go` exists)
- **Test**: `go test ./...`

Example (for 2024 Go solutions):
```bash
cd 2024/golang
go run .
go test ./...
```

### C# Projects

For C# projects (identified by `.csproj` and `.sln` files):
- **Build**: `dotnet build`
- **Run**: `dotnet run` (from the project directory containing the `.csproj`)
- **Test**: `dotnet test`

Example (for 2021 C# solutions, day 1):
```bash
cd 2021/cs/1
dotnet run
```

### TypeScript Projects

TypeScript projects typically use `npm` or `yarn`. Look for `package.json` to identify scripts.
- **Install dependencies**: `npm install` or `yarn install`
- **Run**: `npm start` or `yarn start` (or a specific script defined in `package.json`)
- **Test**: `npm test` or `yarn test`

Example (for 2021 _7 TypeScript solution):
```bash
cd 2021/_7
# Assuming package.json with a start script
npm install
npm start
```

## Code Organization and Patterns

- **Input Data**: Input files are often named `input.data` or `input.txt` and sometimes `input.example` or `input.test` for smaller test cases.
- **Solution Structure**: Solutions often reside in files or directories named after the day (e.g., `_1.rs`, `_5/mod.rs`, `Program.cs`).
- **Modules/Packages**: Projects generally follow standard language-specific module/package conventions.

## Naming Conventions and Style Patterns

- **File Naming**: Day-specific solutions often use `_DAY` prefix (e.g., `_1.rs`, `_5/`).
- **Input Files**: `input.data`, `input.example`, `input.test`.

## Testing Approach and Patterns

- **Test Data**: Small test cases are often stored in files like `input.test` or `input.example`.
- **Language-Specific Testing**: Testing follows the conventions of the respective language (e.g., `cargo test` for Rust, `go test` for Go, `dotnet test` for C#).

## Gotchas and Non-obvious Patterns

- **Polyglot Nature**: Be aware that solutions for different years, or even different days within the same year, might be implemented in entirely different languages. Always check for language-specific configuration files (`Cargo.toml`, `go.mod`, `.csproj`, `package.json`) to determine the context.
- **Input Pathing**: Solutions often expect input files to be in a specific relative path, typically in an `inputs` directory or directly alongside the solution file.
