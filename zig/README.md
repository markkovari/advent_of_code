# ⚡ Zig Advent of Code

This directory contains Zig solutions for Advent of Code, optimized for performance and type safety.

## 📁 Structure

Solutions are grouped by year, with each year containing its own `build.zig` and source files.

- **`20XX/`**: Year-specific implementations.
- **`20XX/src/`**: Daily logic.

## 🚀 Standardized Day Template

Every day is implemented in `src/XX/main.zig`:

```zig
const std = @import("std");

pub fn main() !void {
    // Runtime execution logic
}

fn part1(input: []const u8) !u32 {
    return 0;
}

test "day XX prod" {
    const input = @embedFile("../../../inputs/2021/X/prod.txt");
    try std.testing.expectEqual(@as(u32, 123), try part1(input));
}
```

## 🧪 Testing

We use Zig's built-in test runner to verify solutions at compile-time using `@embedFile`.

```bash
# Navigate to the year
cd zig/2021

# Run all tests
zig build test
```

## 🛠 Adding a New Year

1. Create a folder: `mkdir -p 20XX/src`.
2. Create a `build.zig` that iterates over days and creates test steps.
3. Implement your days in `src/XX/main.zig`.
