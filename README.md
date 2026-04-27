# 🎄 Advent of Code Collection

A comprehensive, multi-language collection of [Advent of Code](https://adventofcode.com/) solutions, standardized and optimized for performance and maintainability.

## 🚀 Overview

This repository contains solutions for various years of Advent of Code, implemented in multiple languages. The project is structured as a monorepo, grouping solutions by language and providing shared utilities and a unified testing interface.

### 📊 Implementation Matrix

| Year | Rust 🦀 | Golang 🐹 | Zig ⚡ | Status |
| :--- | :---: | :---: | :---: | :--- |
| 2015 | ✅ | - | - | Complete |
| 2017 | - | ✅ | - | Refactored |
| 2018 | ✅ | - | - | Complete (Opt.) |
| 2019 | ✅ | ✅ | - | Migrated from OCaml |
| 2020 | ✅ | ✅ | - | Standardized |
| 2021 | ✅ | - | ✅ | Migrated from C# |
| 2022 | ✅ | - | - | Complete |
| 2023 | ✅ | - | - | Migrated from Scala |
| 2024 | - | ✅ | ✅ | Standardized |
| 2025 | ✅ | - | - | In Progress |

---

## 🛠 Project Structure

The repository follows a strict language-based organization:

```text
.
├── golang/     # Go projects grouped by year
├── inputs/     # Centralized puzzle inputs (Source of Truth)
├── rust/       # Rust workspace (Cargo-based)
├── zig/        # Zig projects grouped by year
└── justfile    # Task runner for all languages
```

### 🗝 Key Features

- **Standardized Architecture**: Every project follows a "Dispatcher" model, where a single entry point runs individual days.
- **Unified Inputs**: All puzzle inputs are consolidated in a root `inputs/` folder, shared across all languages.
- **Smart Test Caching**: Local and CI tests use a SHA-256 hashing strategy via `just` to skip tests that haven't changed.
- **Optimized History**: The repository history is scrubbed of large build artifacts, making it extremely lean (~1.7MB).
- **Fast CI**: GitHub Actions pipeline is optimized with conditional testing and language-specific caching.

---

## 🏎 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/) (stable/nightly)
- [Go](https://go.dev/) (1.22+)
- [Zig](https://ziglang.org/) (0.14.0+)
- [Just](https://github.com/casey/just) (Task runner)

### Running Tests

We use `just` to provide a consistent interface across all languages.

```bash
# Run all tests across all languages (with smart caching)
just test-all

# Run only Rust tests
just test-rust

# Run only Go tests
just test-go

# Run only Zig tests
just test-zig

# Force a full re-run by cleaning the cache
just clean-cache
```

### Adding New Solutions

1.  Place your input in `inputs/<year>/<day>/prod.txt`.
2.  Follow the standardized template for your chosen language (see language-specific READMEs).
3.  Update the language dispatcher (`main.rs`, `main.go`, etc.).

---

## 📜 Documentation by Language

- [Rust Documentation](./rust/README.md)
- [Golang Documentation](./golang/README.md)
- [Zig Documentation](./zig/README.md)

---

## ⚖️ License

MIT © [Mark Kovari](https://github.com/markkovari)
