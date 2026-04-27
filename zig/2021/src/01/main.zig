const std = @import("std");

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    const input = try std.fs.cwd().readFileAlloc(allocator, "../../inputs/2021/1/prod.txt", 1024 * 1024);
    defer allocator.free(input);

    const p1 = try part1(input);
    const p2 = try part2(input);

    std.debug.print("Day 01 Part 1: {d}\n", .{p1});
    std.debug.print("Day 01 Part 2: {d}\n", .{p2});
}

fn part1(input: []const u8) !u32 {
    var it = std.mem.tokenizeAny(u8, input, "\r\n");
    var prev: ?u32 = null;
    var count: u32 = 0;
    while (it.next()) |line| {
        const val = try std.fmt.parseInt(u32, line, 10);
        if (prev) |p| {
            if (val > p) count += 1;
        }
        prev = val;
    }
    return count;
}

fn part2(input: []const u8) !u32 {
    var list = std.ArrayList(u32).init(std.heap.page_allocator);
    defer list.deinit();

    var it = std.mem.tokenizeAny(u8, input, "\r\n");
    while (it.next()) |line| {
        try list.append(try std.fmt.parseInt(u32, line, 10));
    }

    var count: u32 = 0;
    if (list.items.len < 4) return 0;
    for (3..list.items.len) |i| {
        if (list.items[i] > list.items[i-3]) count += 1;
    }
    return count;
}

test "day 1 prod" {
    const input = @embedFile("../../../../inputs/2021/2021/1/prod.txt");
    try std.testing.expectEqual(@as(u32, 1681), try part1(input));
    try std.testing.expectEqual(@as(u32, 1704), try part2(input));
}
