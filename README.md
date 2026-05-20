# Rongorongo Research Project

This project is a disciplined research-and-decipherment workspace for Rongorongo, the undeciphered glyphic tradition of Rapa Nui.

The project has two phases:

1. Build an evidence-based research dossier from academic and reputable expert secondary sources.
2. Use that foundation for a cautious decipherment attempt focused first on structure, then on partial semantic readings only where evidence is strong.

## Working Principles

- Research comes before decipherment.
- Evidence and speculation stay separated.
- The starting corpus is the canonical, best-attested Rongorongo material only.
- Existing decipherment proposals are treated as testable hypotheses, not authorities.
- Rapa Nui and Eastern Polynesian context is the default linguistic frame.
- Broader Polynesian comparison is supporting context.
- Public-safe materials are preferred by default.
- Claims must be traceable to either a cited source or an explicit corpus observation.

## Project Files

- `research-dossier.md`: Evidence layer for sources, corpus facts, prior proposals, and confidence-labeled conclusions.
- `decipherment-notebook.md`: Hypothesis layer for structural tests, rejected ideas, and tentative readings.
- `data/corpus-index.csv`: Manual index of canonical Rongorongo objects and transcription sources.
- `data/source-registry.csv`: Source provenance and reliability table.
- `data/hypotheses.csv`: Machine-readable register of working hypotheses and tests.
- `docs/source-intake.md`: Intake workflow for adding public-safe sources.
- `docs/testing-strategy.md`: Evidence and software testing expectations.

## Rust Tooling

```sh
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
```

CI runs `./scripts/ci.sh`, which checks formatting, clippy, tests, build, strict validation, and the core list commands.

## Reliability Standard

The project aims to be audit-ready rather than publication-grade from day one. Notes should remain readable and lightweight, but every important claim should be checkable.

## Confidence Labels

- `High`: Strong agreement across reliable sources or repeated corpus evidence.
- `Medium`: Plausible and supported, but incomplete or dependent on unresolved assumptions.
- `Low`: Speculative, weakly supported, or based on a single source or limited pattern.

## Non-Goals

- Do not present fluent translations without strong evidence.
- Do not mix disputed or weakly attested inscriptions into the first corpus pass.
- Do not redistribute copyrighted scans, images, or long excerpts.
- Do not build a database or image annotation pipeline until manual analysis shows a real need.

## Production-Ready Scope

For v1, production-ready means the repository has enforceable research-governance tooling, traceable data files, documented source intake, and automated validation. It does not mean Rongorongo has been deciphered.
