lint-rust:
    #!/usr/bin/env bash
    set -euxo pipefail
    cargo fmt --all -- --check
    cargo check --all-targets --all-features
    cargo clippy --all-targets --all-features -- -D warnings

lint-mise:
    mise fmt --check

lint: lint-rust lint-mise

test:
    cargo test --all-features
