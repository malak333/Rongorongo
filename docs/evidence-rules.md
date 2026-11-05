# Evidence Rules

Production readiness depends on traceable evidence, not confidence by assertion.

## Claim Types

- `C-*`: Dossier claim extracted from a source or corpus observation.
- `O-*`: Corpus observation.
- `S-*`: Repeated sequence.
- `H-*`: Structural hypothesis.
- `R-*`: Tentative semantic reading.

IDs are stable once published. If a claim changes materially, add a new ID and mark the old one superseded or rejected.

## Source Quality

Preferred sources:

- Academic publications.
- Museum or archive catalog records.
- Reputable expert secondary sources.
- Clearly identified corpus transcriptions.

Lower-confidence sources may be used for context, but they should not carry high-confidence conclusions alone.

## Confidence Labels

- `High`: Strong agreement across reliable sources or repeated corpus evidence.
- `Medium`: Plausible and supported, but incomplete or dependent on unresolved assumptions.
- `Low`: Speculative, weakly supported, or based on a single source or limited pattern.

Confidence labels must account for source reliability, corpus coverage, competing explanations, and transcription uncertainty.

## Evidence Gate

Before a claim is promoted:

- The claim is small enough to verify.
- The evidence chain is explicit.
- The corpus scope is stated.
- Reading-order assumptions are stated where relevant.
- Conflicts and limits are recorded.
- Public-safe handling has been checked.

## Rules Enforced By `audit --strict`

These rules are checked automatically. A row can be typed in as `promoted`
by hand, so the audit re-applies the gates to every row already promoted.

- **Vocabularies.** Row confidence must be `High`, `Medium`, or `Low`. Source
  reliability may also be `Mixed`, which counts as Medium for the cap below.
  Statuses are `draft`, `active`, `promoted`, `rejected`, `superseded`
  (hypotheses use `accepted-working-rule` in place of `promoted`).
- **Confidence cap.** A row's confidence may not exceed the reliability of the
  strongest source it cites. A Low source alone cannot carry a High claim.
- **Corpus reliability.** A corpus row's `source_reliability` must match the
  reliability of its `transcription_source` in the source registry.
- **Promoted rows.** Claims marked `promoted` and hypotheses marked
  `accepted-working-rule` must pass the same gate as `promote claim` /
  `promote hypothesis`. Promoted observations, sequences, and readings need
  Medium or High confidence.
- **Semantic readings.** A reading whose `proposed_meaning` is anything other
  than `none-proposed` must cite only promoted observations, and may not be
  more confident than the weakest observation it cites.
- **Reading order.** Every observation must state its reading-order assumption.
- **Markdown sync.** The CSVs are the source of truth. Every register id must
  appear in a table in `research-dossier.md` or `decipherment-notebook.md`;
  every id a table mentions must exist in the CSVs; and the Confidence,
  Reliability, and Sequence columns must match the CSV. Prose columns are not
  compared, so keep them faithful by review.
- **Transcriptions.** If `data/transcriptions/lines.csv` exists, every row
  must cite a registered source and corpus object, use one of that object's
  sides, have a line id of the form `<side><number>`, name its license, and
  parse into at least one element. `marked_text` must have balanced, unnested
  `{}` marks and must equal `raw_text` once the marks are removed.
  `data/transcriptions/LICENSE.md` must exist.
- **Project decisions are not claims.** A choice the project makes without a
  supporting source belongs in the notebook decision log, not in
  `data/claims.csv` with a borrowed citation.

## Failure Rules

Do not promote:

- Fluent translations without strong structural evidence.
- Claims based only on visual resemblance.
- Claims that depend on private files not available to reviewers.
- Claims that require copyrighted scans or long excerpts to understand.
- Claims that ignore known contradictory evidence.

