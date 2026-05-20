# Contributing

This repository is a research-and-decipherment workspace for Rongorongo. The
highest priority is keeping evidence, speculation, and source rights clearly
separated.

## Contribution Rules

- Keep research claims traceable to a cited source or an explicit corpus
  observation.
- Use the existing confidence labels: `High`, `Medium`, and `Low`.
- Put source summaries and established facts in `research-dossier.md`.
- Put hypotheses, tests, tentative readings, and rejected ideas in
  `decipherment-notebook.md`.
- Keep the phase-one corpus limited to canonical, best-attested material unless
  the scope is intentionally changed and documented.
- Do not present fluent translations or high-confidence semantic claims without
  strong evidence.
- Do not add copyrighted scans, images, book chapters, article PDFs, long
  excerpts, or bulk copied transcriptions.
- Prefer source links, catalog references, short quotations, and original
  summaries.
- Respect Rapa Nui and Eastern Polynesian context; broader comparison should be
  supporting evidence, not a shortcut to decipherment.

## Data Guidelines

Use `data/corpus-index.csv` for lightweight source indexing only. Keep raw
source material out of the repository unless it is clearly public domain,
properly licensed, and necessary for review.

If a new data file is added, include:

- provenance or source reference
- license or public-domain status
- date accessed, where relevant
- inclusion rationale

## Pull Request Checklist

Before opening a pull request, confirm that:

- `./scripts/ci.sh` passes
- `cargo run -- audit --strict` passes for evidence changes
- claims are cited or marked as observations
- confidence labels match the evidence strength
- speculative material stays out of the dossier conclusions
- no restricted, private, or copyrighted source files are included
- generated or local scratch files are excluded
- the changelog is updated for user-visible research, documentation, or release
  hygiene changes

## Style

Write concise, auditable notes. Summaries are preferred over copied source text.
When a claim is uncertain, preserve the uncertainty instead of smoothing it into
stronger language.
