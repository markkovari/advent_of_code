const std = @import("std");
const Allocator = std.mem.Allocator;
const expect = std.testing.expect;
const expectEqual = std.testing.expectEqual;

pub fn read_file(path: []const u8, allocator: Allocator) anyerror![]u8 {
    return try std.fs.cwd().readFileAlloc(
        path,
        allocator,
        std.Io.Limit.unlimited,
    );
}

const Direction = struct {
    dx: i32,
    dy: i32,
};

// All 8 directions: right, left, down, up, and 4 diagonals
const directions = [_]Direction{
    .{ .dx = 0, .dy = 1 },   // right
    .{ .dx = 0, .dy = -1 },  // left
    .{ .dx = 1, .dy = 0 },   // down
    .{ .dx = -1, .dy = 0 },  // up
    .{ .dx = 1, .dy = 1 },   // down-right
    .{ .dx = 1, .dy = -1 },  // down-left
    .{ .dx = -1, .dy = 1 },  // up-right
    .{ .dx = -1, .dy = -1 }, // up-left
};

fn check_word(
    grid: []const []const u8,
    word: []const u8,
    start_row: usize,
    start_col: usize,
    dir: Direction,
) bool {
    const rows = grid.len;
    if (rows == 0) return false;
    const cols = grid[0].len;

    var row: i32 = @intCast(start_row);
    var col: i32 = @intCast(start_col);

    for (word) |char| {
        // Check bounds
        if (row < 0 or row >= rows or col < 0 or col >= cols) {
            return false;
        }

        const urow: usize = @intCast(row);
        const ucol: usize = @intCast(col);

        // Check character match
        if (grid[urow][ucol] != char) {
            return false;
        }

        // Move to next position
        row += dir.dx;
        col += dir.dy;
    }

    return true;
}

fn count_word_occurrences(
    allocator: Allocator,
    content: []const u8,
    word: []const u8,
) anyerror!u32 {
    // Parse the grid into lines
    var lines: std.ArrayList([]const u8) = .{};
    defer lines.deinit(allocator);

    var line_it = std.mem.splitAny(u8, content, "\n");
    while (line_it.next()) |line| {
        if (line.len > 0) {
            try lines.append(allocator, line);
        }
    }

    if (lines.items.len == 0) return 0;

    var count: u32 = 0;

    // For each position in the grid
    for (0..lines.items.len) |row| {
        for (0..lines.items[row].len) |col| {
            // Try all 8 directions
            for (directions) |dir| {
                if (check_word(lines.items, word, row, col, dir)) {
                    count += 1;
                }
            }
        }
    }

    return count;
}

fn get_solution(allocator: Allocator, content: []const u8) anyerror!u32 {
    return try count_word_occurrences(allocator, content, "XMAS");
}

fn check_x_mas(grid: []const []const u8, center_row: usize, center_col: usize) bool {
    const rows = grid.len;
    if (rows == 0) return false;
    const cols = grid[0].len;

    // Check if we can form an X (need space for diagonals)
    if (center_row < 1 or center_row >= rows - 1) return false;
    if (center_col < 1 or center_col >= cols - 1) return false;

    // Center must be 'A'
    if (grid[center_row][center_col] != 'A') return false;

    // Check the four corners of the X
    const top_left = grid[center_row - 1][center_col - 1];
    const top_right = grid[center_row - 1][center_col + 1];
    const bottom_left = grid[center_row + 1][center_col - 1];
    const bottom_right = grid[center_row + 1][center_col + 1];

    // Check diagonal from top-left to bottom-right (can be MAS or SAM)
    const diag1_is_mas = (top_left == 'M' and bottom_right == 'S');
    const diag1_is_sam = (top_left == 'S' and bottom_right == 'M');
    const diag1_valid = diag1_is_mas or diag1_is_sam;

    // Check diagonal from top-right to bottom-left (can be MAS or SAM)
    const diag2_is_mas = (top_right == 'M' and bottom_left == 'S');
    const diag2_is_sam = (top_right == 'S' and bottom_left == 'M');
    const diag2_valid = diag2_is_mas or diag2_is_sam;

    return diag1_valid and diag2_valid;
}

fn count_x_mas(allocator: Allocator, content: []const u8) anyerror!u32 {
    // Parse the grid into lines
    var lines: std.ArrayList([]const u8) = .{};
    defer lines.deinit(allocator);

    var line_it = std.mem.splitAny(u8, content, "\n");
    while (line_it.next()) |line| {
        if (line.len > 0) {
            try lines.append(allocator, line);
        }
    }

    if (lines.items.len == 0) return 0;

    var count: u32 = 0;

    // Check each potential center position
    for (0..lines.items.len) |row| {
        for (0..lines.items[row].len) |col| {
            if (check_x_mas(lines.items, row, col)) {
                count += 1;
            }
        }
    }

    return count;
}

fn get_solution_part2(allocator: Allocator, content: []const u8) anyerror!u32 {
    return try count_x_mas(allocator, content);
}

test "test example is 18" {
    const ta = std.testing.allocator;
    const content = try read_file("./data/04/example", ta);
    defer ta.free(content);
    const solution = try get_solution(ta, content);
    try expectEqual(solution, 18);
}

test "check_word finds XMAS horizontally" {
    const grid = [_][]const u8{
        "XMAS",
        "ABCD",
    };
    const found = check_word(&grid, "XMAS", 0, 0, .{ .dx = 0, .dy = 1 });
    try expect(found);
}

test "check_word finds XMAS vertically" {
    const grid = [_][]const u8{
        "X...",
        "M...",
        "A...",
        "S...",
    };
    const found = check_word(&grid, "XMAS", 0, 0, .{ .dx = 1, .dy = 0 });
    try expect(found);
}

test "check_word finds XMAS diagonally" {
    const grid = [_][]const u8{
        "X...",
        ".M..",
        "..A.",
        "...S",
    };
    const found = check_word(&grid, "XMAS", 0, 0, .{ .dx = 1, .dy = 1 });
    try expect(found);
}

test "check_word finds SAMX backwards" {
    const grid = [_][]const u8{
        "SAMX",
        "ABCD",
    };
    const found = check_word(&grid, "XMAS", 0, 3, .{ .dx = 0, .dy = -1 });
    try expect(found);
}

test "test input solution" {
    const ta = std.testing.allocator;
    const content = try read_file("./data/04/input", ta);
    defer ta.free(content);
    const solution = try get_solution(ta, content);
    std.debug.print("Part 1 solution: {}\n", .{solution});
}

test "test example part 2 is 9" {
    const ta = std.testing.allocator;
    const content = try read_file("./data/04/example", ta);
    defer ta.free(content);
    const solution = try get_solution_part2(ta, content);
    try expectEqual(solution, 9);
}

test "check_x_mas finds basic X pattern" {
    const grid = [_][]const u8{
        "M.S",
        ".A.",
        "M.S",
    };
    const found = check_x_mas(&grid, 1, 1);
    try expect(found);
}

test "check_x_mas finds reversed X pattern" {
    const grid = [_][]const u8{
        "S.M",
        ".A.",
        "S.M",
    };
    const found = check_x_mas(&grid, 1, 1);
    try expect(found);
}

test "test input solution part 2" {
    const ta = std.testing.allocator;
    const content = try read_file("./data/04/input", ta);
    defer ta.free(content);
    const solution = try get_solution_part2(ta, content);
    std.debug.print("Part 2 solution: {}\n", .{solution});
}
