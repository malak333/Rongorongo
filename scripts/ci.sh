#!/usr/bin/env bash
set -euo pipefail

cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo build
cargo run -- validate --strict
cargo run -- audit --strict
cargo run -- corpus list
cargo run -- sources list
cargo run -- hypotheses list
cargo run -- claims list
cargo run -- observations list
cargo run -- intake source --next-id SRC-006
cargo run -- promote claim C-003
cargo run -- promote hypothesis H-002
