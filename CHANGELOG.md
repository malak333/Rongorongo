# Changelog

All notable changes to this project should be documented in this file.

This project uses date-based release notes because it is a research repository,
not a packaged software artifact.

## Unreleased

### Added

- Added public release hygiene files for licensing, contribution guidance,
  security reporting, ignore rules, and release-readiness review.
- Added Rust research-governance CLI commands for validation, audit, register
  listing, source intake templates, and promotion-gate checks.
- Added machine-readable claims and observations registers.
- Added automated E2E coverage for the source-to-claim-to-hypothesis workflow.

### Added (transcriptions)

- Imported the Text D (Échancrée tablet) transcription from Lastilla et al.
  2022 (SRC-003) verbatim: 14 lines in `data/transcriptions/lines.csv`, under
  CC BY-NC 4.0 with a notice in `data/transcriptions/LICENSE.md`.
- Added a notation parser and `transcriptions list|elements` and
  `analyze frequency|sequences` commands. They report structure only.
- The audit checks transcription rows: known source and corpus object, a valid
  side, line ids that match the side, a license on every row, and a license
  notice file.

### Added (phase-one conclusion)

- Registered SRC-006 (2024 radiocarbon dating, CC BY 4.0) and SRC-007
  (2022 Tablet C transcription, paywalled), claim C-004 (dating of Text D's
  wood as a terminus post quem), observations O-003 to O-007, sequences S-002
  to S-004, and hypothesis H-003. S-001 is superseded.
- Added `docs/project-conclusion.md`, with a licensing survey showing that no
  other tablet's transcription is currently redistributable.
- The Markdown sync check now honours `\|` escapes in table cells.

### Added (analysis)

- `analyze sequences --level part` compares individual signs, ignoring
  ligature status, and reports how each occurrence was written.
- `analyze find <pattern>` searches for element sequences with `|`
  alternatives and `*` wildcards, flagging matches that are not certain.

### Fixed (transcriptions)

- Checked the Text D transcription against the typeset PDF. Corrected three
  errors from the web capture (Da6 `60•`, Db1 `3••/40•`, Db3 en dash).
- Added `marked_text` with the source's underlined doubtful readings.
  Doubtful readings now count as `possible`, not certain, and are excluded
  from repeat detection.
- `(−)` now separates elements, matching the authors' numbering. The parser
  counts 211 elements (source: 212; the gap is in the source's Da7). A test
  checks all 32 element positions the authors cite.

### Changed

- `audit --strict` now enforces the evidence rules instead of only checking
  that references resolve: controlled status vocabularies, confidence capped
  by cited-source reliability, promotion gates re-applied to rows already
  marked promoted, semantic readings restricted to promoted observations, and
  CSV/Markdown consistency for the dossier and notebook.
- Missing CSV headers are reported alongside other findings instead of
  aborting validation. All seven registers are now required.
- `Mixed` is accepted only as a source reliability label, not as a row
  confidence.
- Rule tests now run against a fixture workspace instead of live data counts.

### Fixed

- Dossier showed C-001 as High while the register had Medium.
- Notebook showed S-001 as a "522-99 pattern" while the register had no
  sequence; the lead is now listed as unverified and unregistered.
- O-002 was missing from the notebook.
- SRC-004 and SRC-005 showed their access year as a publication date.
- Removed C-003 (default linguistic frame) from the claims register: it is a
  project decision and cited a source that does not address it. It is now in
  the notebook decision log.

### Safety

- Amended the public-safe policy to allow openly licensed transcriptions under
  `data/transcriptions/` with a license notice. NC data is kept separate from
  CC BY 4.0 and MIT material, and is excluded from the Cargo package.

- Documented that third-party scans, images, bulk transcriptions, long excerpts,
  private materials, and restricted source files must not be committed.
- Documented that production-ready means validated research governance, not a
  completed decipherment.
