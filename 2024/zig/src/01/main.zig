const std = @import("std");
const expect = std.testing.expect;



pub fn read_file() const u8[]!void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const file_path = "example.txt";

    const file_contents = try std.fs.cwd().readFileAlloc(
        allocator,
        file_path,
        std.math.maxInt(usize),
    );

    defer allocator.free(file_contents);

    return file_contents;
}


test "always succeeds" {

    try expect(true);
}
