const std = @import("std");
const utils = @import("../utils/utils.zig");

const Direction = enum { Left, Right };

const RotationParseError = error{
    InvalidDirection,
    InvalidAmount,
    InvalidLength,
};

fn Rotation(comptime T: type) type {
    return struct {
        dir: Direction,
        amount: T,
    };
}

fn Dial(comptime T: type) type {
    return struct {
        const Self = @This();

        initial: T,
        position: T,

        pub fn init(initial_position: T) Self {
            return .{
                .initial = initial_position,
                .position = initial_position,
            };
        }
        pub fn rotate(self: *Self, rotation: Rotation(T)) void {
            switch (rotation.dir) {
                .Left => {
                    self.position = @mod(self.position - rotation.amount, 100);
                },
                .Right => {
                    self.position = @mod(self.position + rotation.amount, 100);
                },
            }
        }
        pub fn getCurrentPosition(self: Self) T {
            return self.position;
        }
    };
}

fn parseFromString(comptime T: type, input: []const u8) RotationParseError!Rotation(T) {
    if (input.len < 2) {
        return RotationParseError.InvalidLength;
    }
    const direction: Direction = switch (input[0]) {
        'L' => .Left,
        'R' => .Right,
        else => return RotationParseError.InvalidDirection,
    };

    const amount = utils.parseInt(T, input[1..]) catch {
        return RotationParseError.InvalidAmount;
    };
    return .{ .dir = direction, .amount = amount };
}

pub fn parseAllLines(comptime T: type, allocator: std.mem.Allocator, input: []const u8) ![]Rotation(T) {
    var results: std.ArrayList(Rotation(T)) = .{};
    errdefer results.deinit(allocator);

    var line_it = std.mem.splitScalar(u8, input, '\n');
    while (line_it.next()) |line| {
        // Skip empty lines
        if (line.len == 0) continue;

        // Parse the line - will return early if error occurs
        const rotation = try parseFromString(T, line);
        try results.append(allocator, rotation);
    }

    return try results.toOwnedSlice(allocator);
}

pub fn part1(input: []const u8) !u32 {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    var amount: u32 = 0;

    const rotations = try parseAllLines(i32, allocator, input);
    defer allocator.free(rotations);

    var dial = Dial(i32).init(50);
    for (rotations) |rotation| {
        dial.rotate(rotation);
        if (dial.getCurrentPosition() == 0) {
            amount += 1;
        }
    }

    return amount;
}

pub fn part2(input: []const u8) !i32 {
    // Parse and solve part 2
    _ = input;
    return 0;
}

test "parse to struct" {
    const asText = "L23";
    const parsed = try parseFromString(u32, asText);
    try std.testing.expectEqual(Direction.Left, parsed.dir);
    try std.testing.expectEqual(@as(u32, 23), parsed.amount);
}

test "parse to struct with error" {
    const asText = "G23";
    try std.testing.expectError(RotationParseError.InvalidDirection, parseFromString(u32, asText));
}

test "parse multiple lines" {
    const input =
        \\L10
        \\R25
        \\L5
    ;
    const allocator = std.testing.allocator;
    const rotations = try parseAllLines(u32, allocator, input);
    defer allocator.free(rotations);

    try std.testing.expectEqual(@as(usize, 3), rotations.len);
    try std.testing.expectEqual(Direction.Left, rotations[0].dir);
    try std.testing.expectEqual(@as(u32, 10), rotations[0].amount);
    try std.testing.expectEqual(Direction.Right, rotations[1].dir);
    try std.testing.expectEqual(@as(u32, 25), rotations[1].amount);
}

test "parse multiple lines with error fails early" {
    const input =
        \\L10
        \\X99
        \\R25
    ;
    const allocator = std.testing.allocator;
    try std.testing.expectError(RotationParseError.InvalidDirection, parseAllLines(u32, allocator, input));
}

test "parse with invalid length" {
    const asText = "G";
    try std.testing.expectError(RotationParseError.InvalidLength, parseFromString(u32, asText));
}

test "dial initialization" {
    var dial = Dial(i32).init(100);
    try std.testing.expectEqual(@as(i32, 100), dial.initial);
    try std.testing.expectEqual(@as(i32, 100), dial.getCurrentPosition());
}

test "dial rotation right" {
    var dial = Dial(i32).init(0);
    const rotation = Rotation(i32){ .dir = .Right, .amount = 10 };
    dial.rotate(rotation);
    try std.testing.expectEqual(@as(i32, 0), dial.initial); // initial unchanged
    try std.testing.expectEqual(@as(i32, 10), dial.getCurrentPosition());
}

test "dial rotation left" {
    var dial = Dial(i32).init(50);
    const rotation = Rotation(i32){ .dir = .Left, .amount = 20 };
    dial.rotate(rotation);
    try std.testing.expectEqual(@as(i32, 50), dial.initial); // initial unchanged
    try std.testing.expectEqual(@as(i32, 30), dial.getCurrentPosition());
}

test "dial multiple rotations" {
    var dial = Dial(i32).init(0);
    dial.rotate(.{ .dir = .Right, .amount = 10 });
    dial.rotate(.{ .dir = .Left, .amount = 5 });
    dial.rotate(.{ .dir = .Right, .amount = 15 });
    try std.testing.expectEqual(@as(i32, 0), dial.initial); // initial always unchanged
    try std.testing.expectEqual(@as(i32, 20), dial.getCurrentPosition());
}

test "first part example" {
    const example_input = @embedFile("example.txt");
    const result = try part1(example_input);

    try std.testing.expectEqual(3, result);
}

test "first part input" {
    const example_input = @embedFile("input.txt");
    const result = try part1(example_input);

    try std.testing.expectEqual(1071, result);
}
