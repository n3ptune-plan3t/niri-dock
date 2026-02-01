# Install to system
install: build
    sudo install -Dm755 target/release/niri-dock /usr/bin/niri-dock

# Build release
build:
    cargo build --release --locked

# Run with debug logging
run:
    RUST_LOG=debug cargo run --release

# Check code
check:
    cargo clippy --all-targets --all-features -- -D warnings
    cargo fmt --check
    cargo test --lib

# Create release tag
release version:
    git tag -a "v{{version}}" -m "Release {{version}}"
    git push origin "v{{version}}"