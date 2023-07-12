const std = @import("std");
const testing = std.testing;
const fs = std.fs;
const reader = std.io.getStdIn().reader();
const heap = std.heap.page_allocator;

pub fn solveFirst() !usize {
    const file = try std.fs.cwd().openFile("./libs/1/example.data", .{});
    defer file.close();

    const contents = try file.reader().readAllAlloc(
        heap,
        10000,
    );
    defer heap.free(contents);
    std.debug.print("Content: {s}\n", .{contents});
    return 2;
}

pub fn main() !void {
    const a = try solveFirst();
    std.debug.print("{d}", .{a});
}

test "basic add functionality" {
    try testing.expect(try solveFirst() == 2);
}
