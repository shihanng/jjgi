lint-rust:
    #!/usr/bin/env bash
    set -euxo pipefail
    cargo fmt --all -- --check
    cargo check --all-targets --all-features
    cargo clippy --all-targets --all-features -- -D warnings

lint: lint-rust

test:
    cargo test --all-features
