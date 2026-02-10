const std = @import("std");
const Io = std.Io;

const day01 = @import("day01/day01.zig");

pub fn main(init: std.process.Init) !void {
    const arena: std.mem.Allocator = init.arena.allocator();
    const args = try init.minimal.args.toSlice(arena);

    const io = init.io;
    var stdout_buffer: [1024]u8 = undefined;
    var stdout_file_writer: Io.File.Writer = .init(.stdout(), io, &stdout_buffer);
    const stdout_writer = &stdout_file_writer.interface;

    if (args.len < 2) {
        try stdout_writer.writeAll("Usage: zig build run -- <day>\n");
        try stdout_writer.writeAll("Example: zig build run -- 1\n");
        try stdout_writer.flush();
        return;
    }

    const day = try std.fmt.parseInt(u8, args[1], 10);

    switch (day) {
        1 => {
            const input = @embedFile("day01/input.txt");
            const part1_result = try day01.part1(input);
            const part2_result = try day01.part2(input);

            try stdout_writer.print("Day 1:\n", .{});
            try stdout_writer.print("  Part 1: {}\n", .{part1_result});
            try stdout_writer.print("  Part 2: {}\n", .{part2_result});
        },
        else => {
            try stdout_writer.print("Day {} not implemented yet\n", .{day});
        },
    }

    try stdout_writer.flush();
}

test "simple test" {
    const gpa = std.testing.allocator;
    var list: std.ArrayList(i32) = .empty;
    defer list.deinit(gpa); // Try commenting this out and see if zig detects the memory leak!
    try list.append(gpa, 42);
    try std.testing.expectEqual(@as(i32, 42), list.pop());
}

test "fuzz example" {
    const Context = struct {
        fn testOne(context: @This(), input: []const u8) anyerror!void {
            _ = context;
            // Try passing `--fuzz` to `zig build test` and see if it manages to fail this test case!
            try std.testing.expect(!std.mem.eql(u8, "canyoufindme", input));
        }
    };
    try std.testing.fuzz(Context{}, Context.testOne, .{});
}

test {
    @import("std").testing.refAllDecls(@This());
    _ = day01;
}
