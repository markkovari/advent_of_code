const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const test_step = b.step("test", "Run all tests");

    const days = [_][]const u8{ "01", "02" };

    for (days) |day| {
        const path = b.fmt("src/{s}/main.zig", .{day});
        const exe = b.addExecutable(.{
            .name = b.fmt("day{s}", .{day}),
            .root_module = b.createModule(.{
                .root_source_file = b.path(path),
                .target = target,
                .optimize = optimize,
            }),
        });
        b.installArtifact(exe);

        const day_test = b.addTest(.{
            .root_module = b.createModule(.{
                .root_source_file = b.path(path),
                .target = target,
                .optimize = optimize,
            }),
        });
        const run_day_test = b.addRunArtifact(day_test);
        test_step.dependOn(&run_day_test.step);
    }
}
