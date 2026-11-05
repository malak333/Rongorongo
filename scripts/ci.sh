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
cargo run -- sequences list
cargo run -- readings list
cargo run -- intake source --next-id SRC-006
cargo run -- promote claim C-002
cargo run -- promote hypothesis H-002
cargo run -- transcriptions list
cargo run -- analyze frequency
cargo run -- analyze sequences
cargo run -- analyze sequences --level part
cargo run -- analyze find "600|610 * 200.52x.*"
