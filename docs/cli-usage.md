# CLI Usage

The repository includes a Rust CLI for validating research artifacts and listing the current machine-readable registers.

## Validation Commands

```sh
cargo run -- validate
cargo run -- validate --strict
cargo run -- audit --strict
```

Use `validate` during drafting and `validate --strict` for schema gates. Use `audit --strict` for the full production gate because it also checks source/corpus/claim/hypothesis cross-references.

## Listing Commands

```sh
cargo run -- corpus list
cargo run -- corpus list --format json
cargo run -- sources list
cargo run -- hypotheses list
cargo run -- claims list
cargo run -- observations list
cargo run -- sequences list
cargo run -- readings list
cargo run -- intake source --next-id SRC-006
cargo run -- promote claim C-003
cargo run -- promote hypothesis H-002
```

The table output is for humans. JSON output is available where local scripts or downstream tooling need structured data. `./scripts/ci.sh` is the complete local CLI gate; this repository intentionally does not use GitHub Actions.

## Intake And Promotion

`intake source` prints a public-safe CSV row template for `data/source-registry.csv`.

`promote claim` and `promote hypothesis` do not modify files. They check whether the selected item has the evidence, source references, tests, and confidence level needed before a maintainer manually promotes it into the dossier.

## Safety Contract

The CLI should default to safe, reviewable behavior:

- Read public-safe repository files by default.
- Require explicit paths for any private source notes.
- Never emit copyrighted source text into generated reports.
- Exit nonzero when evidence gates fail.
- Keep semantic readings separate from validated evidence.

## Repository Inspection Commands

```sh
git status --short --branch
rg -n "TODO|TBD|copyright|scan|translation" README.md research-dossier.md decipherment-notebook.md docs data
```
