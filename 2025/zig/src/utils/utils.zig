const std = @import("std");

/// Split input by newlines
pub fn lines(input: []const u8) std.mem.SplitIterator(u8, .sequence) {
    return std.mem.splitSequence(u8, input, "\n");
}

/// Parse an integer from a string
pub fn parseInt(comptime T: type, s: []const u8) !T {
    return std.fmt.parseInt(T, s, 10);
}

/// Trim whitespace from both ends
pub fn trim(s: []const u8) []const u8 {
    return std.mem.trim(u8, s, " \t\r\n");
}

test "lines iterator" {
    const input = "line1\nline2\nline3";
    var iter = lines(input);
    try std.testing.expectEqualStrings("line1", iter.next().?);
    try std.testing.expectEqualStrings("line2", iter.next().?);
    try std.testing.expectEqualStrings("line3", iter.next().?);
}
