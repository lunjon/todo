default: fmt check test lint

check:
    cargo check
test:
    cargo test
fmt:
    cargo fmt
lint:
    cargo clippy --fix --allow-dirty

install:
    cargo install --path .
