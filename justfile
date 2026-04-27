set shell := ["bash", "-c"]

# Run all tests across all languages
test-all:
    just test-rust
    just test-go
    just test-zig

# --- Rust ---

# Run all Rust tests
test-rust:
    @for project in 2015 2018 2019/aoc_rust 2020/01 2021/_1 2021/_2 2021/_3 2021/_4 2021/_5 2021/_6 2021/_7 2022 2023/rust 2025/rust runner; do \
        just test-project-rust $project; \
    done

# Run a specific Rust project if its files or aoc-rust-common changed
test-project-rust project:
    @mkdir -p .cache/rust
    @SAFE_NAME=$(echo {{project}} | tr '/' '_')
    @# Hash the project dir AND the shared rust library
    @HASH=$(find {{project}} aoc-rust-common -type f -not -path "*/target/*" -not -path "*/.git/*" 2>/dev/null | sort | xargs shasum -a 256 | shasum -a 256 | awk '{print $1}')
    @if [ -z "$HASH" ]; then HASH="empty"; fi
    @if [ -f .cache/rust/$SAFE_NAME.success ] && [ "$(cat .cache/rust/$SAFE_NAME.success)" = "$HASH" ]; then \
        echo "✅ Rust: {{project}} (Cached)"; \
    else \
        echo "🚀 Rust: Running {{project}}..."; \
        (cd {{project}} && cargo test) && echo "$HASH" > .cache/rust/$SAFE_NAME.success; \
    fi

# --- Go ---

# Run all Go tests
test-go:
    @for project in 2017 2019/go 2020/02 2024/golang; do \
        just test-project-go $project; \
    done

# Run a specific Go project if its files or aoc-go-common changed
test-project-go project:
    @mkdir -p .cache/go
    @SAFE_NAME=$(echo {{project}} | tr '/' '_')
    @# Hash the project dir AND the shared go library
    @HASH=$(find {{project}} aoc-go-common -type f -not -path "*/.git/*" 2>/dev/null | sort | xargs shasum -a 256 | shasum -a 256 | awk '{print $1}')
    @if [ -z "$HASH" ]; then HASH="empty"; fi
    @if [ -f .cache/go/$SAFE_NAME.success ] && [ "$(cat .cache/go/$SAFE_NAME.success)" = "$HASH" ]; then \
        echo "✅ Go: {{project}} (Cached)"; \
    else \
        echo "🚀 Go: Running {{project}}..."; \
        (cd {{project}} && go test ./...) && echo "$HASH" > .cache/go/$SAFE_NAME.success; \
    fi

# --- Zig ---

# Run all Zig tests
test-zig:
    @for project in 2021/zig 2024/zig; do \
        just test-project-zig $project; \
    done

# Run a specific Zig project if its files changed
test-project-zig project:
    @mkdir -p .cache/zig
    @SAFE_NAME=$(echo {{project}} | tr '/' '_')
    @HASH=$(find {{project}} -type f -not -path "*/.zig-cache/*" -not -path "*/zig-out/*" -not -path "*/.git/*" 2>/dev/null | sort | xargs shasum -a 256 | shasum -a 256 | awk '{print $1}')
    @if [ -z "$HASH" ]; then HASH="empty"; fi
    @if [ -f .cache/zig/$SAFE_NAME.success ] && [ "$(cat .cache/zig/$SAFE_NAME.success)" = "$HASH" ]; then \
        echo "✅ Zig: {{project}} (Cached)"; \
    else \
        echo "🚀 Zig: Running {{project}}..."; \
        (cd {{project}} && zig build test) && echo "$HASH" > .cache/zig/$SAFE_NAME.success; \
    fi

# Clean all test caches
clean-cache:
    rm -rf .cache
