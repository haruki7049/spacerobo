build:
    cargo build --workspace
    cargo build --release --workspace

test:
    cargo test --workspace
    cargo test --release --workspace

check:
    cargo check --workspace
    cargo check --release --workspace

clippy:
    cargo clippy --workspace
    cargo clippy --release --workspace

doc:
    cargo doc --workspace
    cargo doc --release --workspace

all: build test check clippy
