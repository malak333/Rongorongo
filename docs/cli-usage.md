# CLI Usage

The repository includes a Rust CLI for validating research artifacts and listing the current machine-readable registers.

## Validation Commands

```sh
cargo run -- validate
cargo run -- validate --strict
cargo run -- audit --strict
```

Use `validate` during drafting and `validate --strict` for schema and vocabulary gates. Use `audit --strict` for the full gate: it also checks cross-references, confidence caps, promotion gates on rows already marked promoted, semantic-reading rules, and CSV/Markdown consistency. See `docs/evidence-rules.md` for the full rule list. With `--format json`, the summary goes to stdout and findings go to stderr.

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
cargo run -- promote claim C-002
cargo run -- promote hypothesis H-002
```

## Transcriptions And Analysis

```sh
cargo run -- transcriptions list
cargo run -- transcriptions elements --line Da3
cargo run -- transcriptions elements --problems
cargo run -- analyze frequency
cargo run -- analyze frequency --exact
cargo run -- analyze sequences --min-length 2 --min-count 2
cargo run -- analyze sequences --level part
cargo run -- analyze find "600|610 * 200.52x.*"
```

Transcription lines are stored verbatim. Elements, sign counts, and repeats are derived each time, so changing the parser never rewrites source data. The notation the parser understands is documented at the top of `src/transcription.rs`.

- `transcriptions elements` shows how each line splits into graphic elements. Underlined (doubtful) spans appear in `{}`. Each element is flagged `doubtful` (underlined sign or join), `uncertain` (`•`), `ambiguous` (`/`), `unparsed`, `lacuna`, `possible-join` (after `(−)`), `doubtful-boundary` (after an underlined separator), or `irregular-separator`.
- `analyze frequency` counts signs by Barthel base number. `count` is certain single readings; `possible` is readings that are underlined, marked `•`, or offered as one of several alternatives. Signs that do not fit the notation are listed and excluded.
- `analyze sequences` reports maximal exact repeats of elements within a line. Runs never cross a line, a doubtful, uncertain, ambiguous, or unparsed element, or a doubtful or possible-join boundary, so uncertainty cannot manufacture a repeat. Positions are 1-based element indices; for Text D they match the source's numbering (checked in `tests/e2e_cli.rs`).

- `analyze sequences --level part` compares individual signs instead of whole elements, ignoring whether neighbouring signs are ligatured (the source says this is often unclear). The `forms` column shows how each occurrence was written: `.` for a ligature join, `–` for separate elements. Runs still break at lacunae and at doubtful, uncertain, ambiguous, or unparsed signs.
- `analyze find <pattern>` searches for a sequence of elements within lines. Separate element patterns with spaces or dashes, join alternatives with `|`, and use `*` as a wildcard; a lone `*` matches any one element. Patterns match the element's normalized text, in which `:` ligatures become `.`. `*` works on characters, so `200.52*` also matches `200.524f.78`; write `200.52x.*` to require a whole part. The `certain` column is `no` when any matched element is doubtful, uncertain, ambiguous, or unparsed, or a soft boundary falls inside the match.

These commands report structure only. They never propose meanings. Output derived from licensed transcription data carries that data's attribution line.

The table output is for humans. JSON output is available where local scripts or downstream tooling need structured data. `./scripts/ci.sh` is the complete local CLI gate; this repository intentionally does not use GitHub Actions.

## Intake And Promotion

`intake source` prints a public-safe CSV row template for `data/source-registry.csv`.

`promote claim` and `promote hypothesis` do not modify files. They check whether the selected item has the evidence, source references, tests, and confidence level needed before a maintainer manually sets its status and adds it to the dossier. `audit --strict` re-runs the same gate on every promoted row, so editing the status by hand cannot skip it.

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
