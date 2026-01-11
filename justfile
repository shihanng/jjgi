lint-rust:
    #!/usr/bin/env bash
    set -euxo pipefail
    cargo fmt --all -- --check
    cargo check --all-targets --all-features
    cargo clippy --all-targets --all-features -- -D warnings

lint-mise:
    mise fmt --check

lint-sh:
    #!/usr/bin/env bash
    set -euxo pipefail
    shfmt -d ./tests/scripts/
    shellcheck ./tests/scripts/*

lint-gh:
    #!/usr/bin/env bash
    set -euxo pipefail
    yamlfmt -lint
    actionlint

lint-md:
    markdownlint-cli2 "**/*.md" "#target"

lint-just:
    just --fmt --check --unstable

lint: lint-rust lint-mise lint-sh lint-gh lint-md lint-just

test:
    cargo test --all-features
