set shell := ["bash", "-c"]

# Run all tests across all languages
test-all:
    just test-rust
    just test-go
    just test-zig

# --- Rust ---

# Run all Rust tests
test-rust:
    @for project in 2015 2018 2019 2020 2021 2022 2023 2025 runner; do \
        just test-project-rust $project; \
    done

# Run a specific Rust project
test-project-rust project:
    @mkdir -p .cache/rust
    @SAFE_NAME=$(echo {{project}} | tr '/' '_')
    @# Hash the project dir AND the shared rust library
    @HASH=$(find rust/{{project}} rust/common -type f -not -path "*/target/*" -not -path "*/.git/*" 2>/dev/null | sort | xargs shasum -a 256 | shasum -a 256 | awk '{print $1}')
    @if [ -z "$HASH" ]; then HASH="empty"; fi
    @if [ -f .cache/rust/$SAFE_NAME.success ] && [ "$(cat .cache/rust/$SAFE_NAME.success)" = "$HASH" ]; then \
        echo "✅ Rust: {{project}} (Cached)"; \
    else \
        echo "🚀 Rust: Running {{project}}..."; \
        (cd rust/{{project}} && cargo test) && echo "$HASH" > .cache/rust/$SAFE_NAME.success; \
    fi

# --- Go ---

# Run all Go tests
test-go:
    @for project in 2017 2019 2020 2024; do \
        just test-project-go $project; \
    done

# Run a specific Go project
test-project-go project:
    @mkdir -p .cache/go
    @SAFE_NAME=$(echo {{project}} | tr '/' '_')
    @# Hash the project dir AND the shared go library
    @HASH=$(find golang/{{project}} golang/common -type f -not -path "*/.git/*" 2>/dev/null | sort | xargs shasum -a 256 | shasum -a 256 | awk '{print $1}')
    @if [ -z "$HASH" ]; then HASH="empty"; fi
    @if [ -f .cache/go/$SAFE_NAME.success ] && [ "$(cat .cache/go/$SAFE_NAME.success)" = "$HASH" ]; then \
        echo "✅ Go: {{project}} (Cached)"; \
    else \
        echo "🚀 Go: Running {{project}}..."; \
        (cd golang/{{project}} && go test ./...) && echo "$HASH" > .cache/go/$SAFE_NAME.success; \
    fi

# --- Zig ---

# Run all Zig tests
test-zig:
    @for project in 2021 2024; do \
        just test-project-zig $project; \
    done

# Run a specific Zig project
test-project-zig project:
    @mkdir -p .cache/zig
    @SAFE_NAME=$(echo {{project}} | tr '/' '_')
    @HASH=$(find zig/{{project}} -type f -not -path "*/.zig-cache/*" -not -path "*/zig-out/*" -not -path "*/.git/*" 2>/dev/null | sort | xargs shasum -a 256 | shasum -a 256 | awk '{print $1}')
    @if [ -z "$HASH" ]; then HASH="empty"; fi
    @if [ -f .cache/zig/$SAFE_NAME.success ] && [ "$(cat .cache/zig/$SAFE_NAME.success)" = "$HASH" ]; then \
        echo "✅ Zig: {{project}} (Cached)"; \
    else \
        echo "🚀 Zig: Running {{project}}..."; \
        (cd zig/{{project}} && zig build test) && echo "$HASH" > .cache/zig/$SAFE_NAME.success; \
    fi

# Clean all test caches
clean-cache:
    rm -rf .cache
