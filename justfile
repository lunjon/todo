default: fmt check test lint

check:
    cargo check
test:
    cargo test
fmt:
    cargo fmt
lint:
    cargo clippy

install:
    cargo install --path .
