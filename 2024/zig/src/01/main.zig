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

fn get_solution(allocator: Allocator, content: []const u8) anyerror!u32 {
    // Parse pairs
    var pairs = try get_number_pairs_of_lines(allocator, content);
    defer pairs.deinit(allocator);

    // Create left and right arrays
    var left: std.ArrayList(u32) = .{};
    defer left.deinit(allocator);
    var right: std.ArrayList(u32) = .{};
    defer right.deinit(allocator);

    // Populate left and right arrays
    for (pairs.items) |pair| {
        try left.append(allocator, pair[0]);
        try right.append(allocator, pair[1]);
    }

    // Sort both arrays
    std.mem.sort(u32, left.items, {}, std.sort.asc(u32));
    std.mem.sort(u32, right.items, {}, std.sort.asc(u32));

    // Calculate sum of absolute differences
    var sum: u32 = 0;
    for (left.items, right.items) |l, r| {
        const diff = if (l > r) l - r else r - l;
        sum += diff;
    }

    return sum;
}

pub fn count_frequency(allocator: Allocator, numbers: [] const u8) anyerror!std.AutoHashMap(u32, u32) {
    var freq = std.AutoHashMap(u32, u32).init(allocator);

    for (numbers) |num| {
        const currentValue = freq.get(num);
        if (currentValue) |value| {
            try freq.put(num, value + 1);
        } else {
            try freq.put(num, 1);
        }
    }
    return freq;
}

test "test example is 11" {
    const ta = std.testing.allocator;
    const content = try read_file("./data/01/example", ta);
    defer ta.free(content);
    const solution = try get_solution(ta, content);
    try expect(solution == 11);
}

test "test inputs solution is is 1970720" {
    const ta = std.testing.allocator;
    const content = try read_file("./data/01/input", ta);
    defer ta.free(content);
    const solution = try get_solution(ta, content);
    try expectEqual(solution, 1970720);
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

test "count frequency" {
    const ta = std.testing.allocator;

    const numbers = [_]u8{ 1, 2, 3, 4, 4, 5, 1 };
    var freq = try count_frequency(ta, &numbers);
    defer freq.deinit();
    try expect(freq.count() == 5);
    const ones_count = freq.get(1) orelse 0;
    try expect(ones_count == 2);
    const twos_count = freq.get(2) orelse 0;
    try expect(twos_count == 1);
    const threes_count = freq.get(3) orelse 0;
    try expect(threes_count == 1);
    const fours_count = freq.get(4) orelse 0;
    try expect(fours_count == 2);
    const fives_count = freq.get(5) orelse 0;
    try expect(fives_count == 1);
    const sixes_count = freq.get(6) orelse 0;
    try expect(sixes_count == 0);
}
