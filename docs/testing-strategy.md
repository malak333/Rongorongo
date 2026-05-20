# Testing Strategy

This project has two test layers.

## Evidence Tests

Evidence tests check whether claims are traceable and structurally sound:

- Corpus rows must have stable IDs and source references.
- Source rows must describe contribution, limits, access, and reliability.
- Hypotheses must include a claim, evidence, test, status, confidence, and notes.
- Semantic readings require evidence and alternative explanations before promotion.
- Reading-order assumptions must be explicit for sequence observations.
- Repeated sequences and tentative readings must stay in their dedicated
  registers until supported by cross-referenced evidence.

## Software Tests

The Rust CLI should enforce the project rules:

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- CLI E2E tests for validation and list commands
- Strict validation for the local release gate
- Full audit for cross-reference integrity
- Promotion-gate checks for claims and hypotheses
- List/intake/promotion commands exercised through `./scripts/ci.sh`

The only production-ready v1 claim is that tooling can validate research artifacts and catch unsupported promotion paths. It is not a production-ready decipherment.
