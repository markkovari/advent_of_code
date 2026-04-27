const std = @import("std");

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    const input = try std.fs.cwd().readFileAlloc(allocator, "../../inputs/2/prod.txt", 1024 * 1024);
    defer allocator.free(input);

    const p1 = try part1(input);
    const p2 = try part2(input);

    std.debug.print("Day 02 Part 1: {d}\n", .{p1});
    std.debug.print("Day 02 Part 2: {d}\n", .{p2});
}

fn part1(input: []const u8) !u32 {
    var it = std.mem.tokenizeAny(u8, input, "\r\n");
    var x: u32 = 0;
    var y: u32 = 0;
    while (it.next()) |line| {
        var parts = std.mem.splitScalar(u8, line, ' ');
        const command = parts.next().?;
        const dist = try std.fmt.parseInt(u32, parts.next().?, 10);
        if (std.mem.eql(u8, command, "forward")) {
            x += dist;
        } else if (std.mem.eql(u8, command, "down")) {
            y += dist;
        } else if (std.mem.eql(u8, command, "up")) {
            y -= dist;
        }
    }
    return x * y;
}

fn part2(input: []const u8) !u32 {
    var it = std.mem.tokenizeAny(u8, input, "\r\n");
    var x: u32 = 0;
    var y: u32 = 0;
    var aim: u32 = 0;
    while (it.next()) |line| {
        var parts = std.mem.splitScalar(u8, line, ' ');
        const command = parts.next().?;
        const dist = try std.fmt.parseInt(u32, parts.next().?, 10);
        if (std.mem.eql(u8, command, "forward")) {
            x += dist;
            y += aim * dist;
        } else if (std.mem.eql(u8, command, "down")) {
            aim += dist;
        } else if (std.mem.eql(u8, command, "up")) {
            aim -= dist;
        }
    }
    return x * y;
}

test "day 2 prod" {
    const input = @embedFile("../../inputs/2/prod.txt");
    try std.testing.expectEqual(@as(u32, 150), try part1(input));
    try std.testing.expectEqual(@as(u32, 900), try part2(input));
}
