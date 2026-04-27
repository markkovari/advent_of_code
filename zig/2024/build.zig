const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    const mod = b.addModule("zig", .{
        .root_source_file = b.path("src/root.zig"),
        .target = target,
    });
    const exe = b.addExecutable(.{
        .name = "zig",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src/main.zig"),
            .target = target,
            .optimize = optimize,
            .imports = &.{
                .{ .name = "zig", .module = mod },
            },
        }),
    });

    b.installArtifact(exe);
    const run_step = b.step("run", "Run the app");
    const run_cmd = b.addRunArtifact(exe);
    run_step.dependOn(&run_cmd.step);

    run_cmd.step.dependOn(b.getInstallStep());

    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    const mod_tests = b.addTest(.{
        .root_module = mod,
    });

    const run_mod_tests = b.addRunArtifact(mod_tests);
    const exe_tests = b.addTest(.{
        .root_module = exe.root_module,
    });

    const run_exe_tests = b.addRunArtifact(exe_tests);
    const test_step = b.step("test", "Run tests");
    test_step.dependOn(&run_mod_tests.step);
    test_step.dependOn(&run_exe_tests.step);

    // Dynamically add tests for all /src/{{number}}/*.zig files
    const src_dir = std.fs.cwd().openDir("src", .{ .iterate = true }) catch |err| {
        std.debug.print("Failed to open src directory: {}\n", .{err});
        return;
    };
    var src_iter = src_dir.iterate();
    while (src_iter.next() catch null) |entry| {
        if (entry.kind == .directory) {
            // Check if directory name is numeric
            const is_numeric = blk: {
                for (entry.name) |c| {
                    if (c < '0' or c > '9') break :blk false;
                }
                break :blk entry.name.len > 0;
            };

            if (is_numeric) {
                const day_path = b.fmt("src/{s}", .{entry.name});
                var day_dir = std.fs.cwd().openDir(day_path, .{ .iterate = true }) catch continue;
                defer day_dir.close();

                var day_iter = day_dir.iterate();
                while (day_iter.next() catch null) |file_entry| {
                    if (file_entry.kind == .file and std.mem.endsWith(u8, file_entry.name, ".zig")) {
                        const file_path = b.fmt("src/{s}/{s}", .{ entry.name, file_entry.name });
                        const day_module = b.addModule(b.fmt("day_{s}_{s}", .{ entry.name, file_entry.name }), .{
                            .root_source_file = b.path(file_path),
                            .target = target,
                            .optimize = optimize,
                        });
                        const day_test = b.addTest(.{
                            .root_module = day_module,
                        });
                        const run_day_test = b.addRunArtifact(day_test);
                        test_step.dependOn(&run_day_test.step);
                    }
                }
            }
        }
    }
}
