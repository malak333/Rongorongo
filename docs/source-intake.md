# Source Intake Workflow

Use one source at a time. Do not promote a claim until it has a source row and a confidence label.

## Intake Steps

1. Add the source to `data/source-registry.csv`.
2. Extract small claims into `research-dossier.md`.
3. Add or update affected corpus rows in `data/corpus-index.csv`.
4. Move only testable questions into `decipherment-notebook.md`.
5. If a hypothesis survives checks, promote the conclusion back into `research-dossier.md`.

CLI helpers:

```sh
cargo run -- intake source --next-id SRC-006
cargo run -- audit --strict
```

Use the intake template as a starting row, then edit it manually with the real citation, contribution, limits, reliability, and public-safe notes.

## Source Reliability

- `High`: peer-reviewed work, museum records, primary corpus catalogues, or directly inspectable corpus evidence.
- `Medium`: reputable expert secondary source or digital corpus reference that still needs scholarly confirmation.
- `Low`: orientation material, unspecialized summaries, or speculative proposals.
- `Mixed`: useful source with separable strong and weak parts.

## Public-Safe Rule

Do not commit copyrighted scans, long excerpts, paywalled PDFs, or unrestricted copies of third-party transcriptions. Store citations and short notes instead.
