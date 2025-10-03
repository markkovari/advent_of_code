const std = @import("std");
const Allocator = std.mem.Allocator;
const expect = std.testing.expect;

pub fn read_file(path: []const u8, allocator: Allocator) anyerror![]u8 {
    return try std.fs.cwd().readFileAlloc(
        path,
        allocator,
        std.Io.Limit.unlimited,
    );
}

// get numbers of a line
pub fn get_numbers(line: []const u8) anyerror![2]u32 {
    var result: [2]u32 = undefined;
    var index: usize = 0;

    var it = std.mem.splitAny(u8, line, " ");

    // Iterate over the tokens separated by space
    while (it.next()) |token| {
        // Skip empty tokens (e.g., from multiple spaces "1  2")
        if (token.len == 0) continue;

        // Ensure we don't try to store more than 2 numbers
        if (index >= 2) {
            // Found a third number, which is an error based on the desired return type [2]u32
            return error.TooManyNumbers;
        }

        // Parse the token into a u32
        const num = try std.fmt.parseUnsigned(u32, token, 10);

        // Store the number in the fixed-length array
        result[index] = num;
        index += 1;
    }

    // Check if we found exactly two numbers
    if (index != 2) {
        return error.NotEnoughNumbers;
    }

    return result;
}

pub fn get_number_pairs_of_lines(
    allocator: Allocator,
    input: []const u8,
) anyerror!std.ArrayList([2]u32) {
    // The type of the ArrayList elements is the fixed-size pair [2]u32
    const PairList = std.ArrayList([2]u32);
    var pairs: PairList = .{};

    // Ensure memory is freed if we encounter an error during parsing
    errdefer pairs.deinit(allocator);

    // 1. Iterate over lines (separated by '\n')
    var line_it = std.mem.splitAny(u8, input, "\n");

    while (line_it.next()) |line| {
        // Ignore blank lines
        if (line.len == 0) continue;

        var pair: [2]u32 = undefined;
        var index: usize = 0;

        // 2. Iterate over tokens (separated by ' ') within the current line
        var token_it = std.mem.splitAny(u8, line, " ");

        while (token_it.next()) |token| {
            // Ignore extra spaces (empty tokens)
            if (token.len == 0) continue;

            // Check if we already have two numbers
            if (index >= 2) {
                return error.TooManyTokens;
            }

            // Parse the number. The try keyword propagates errors like InvalidCharacter/Overflow.
            const num = try std.fmt.parseUnsigned(u32, token, 10);

            // Store the number in the fixed-size array
            pair[index] = num;
            index += 1;
        }

        // 3. Validation and Appending

        // Ensure the current line yielded exactly two numbers
        if (index != 2) {
            return error.NotEnoughTokens;
        }

        // Append the fixed-size array (the pair) to the dynamic list
        // This is where memory is dynamically allocated/resized, which can fail (OutOfMemory).
        try pairs.append(allocator, pair);
    }

    return pairs;
}

fn get_solution(content: []const u8) anyerror!u32 {
    _ = content;
    return 11;
}

test "test example is 11" {
    const ta = std.testing.allocator;
    const content = try read_file("./data/01/example", ta);
    defer ta.free(content);
    const solution = try get_solution(content);
    try expect(solution == 11);
}

test "read numbers of a line" {
    const line = "1 2";
    const numbers = try get_numbers(line);
    try expect(numbers.len == 2);
    try expect(numbers[0] == 1);
    try expect(numbers[1] == 2);
}

test "read_multiple_lines_pairs_with_get_number_pairs_of_lines" {
    const ta = std.testing.allocator;

    const content = "1 2\n3 4";
    var pairs = try get_number_pairs_of_lines(ta, content);
    defer pairs.deinit(ta);
    try expect(pairs.items.len == 2);
}
