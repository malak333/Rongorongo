# Testing Strategy

This project has two test layers.

## Evidence Tests

Evidence tests check whether claims are traceable and structurally sound:

- Corpus rows must have stable IDs and source references.
- Source rows must describe contribution, limits, access, and reliability.
- Hypotheses must include a claim, evidence, test, status, confidence, and notes.
- Semantic readings require evidence and alternative explanations before promotion.
- Reading-order assumptions must be explicit for sequence observations.

## Software Tests

The Rust CLI should enforce the project rules:

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- CLI E2E tests for validation and list commands
- Strict validation for CI and release gates
- Full audit for cross-reference integrity
- Promotion-gate checks for claims and hypotheses

The only production-ready v1 claim is that tooling can validate research artifacts and catch unsupported promotion paths. It is not a production-ready decipherment.
