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
