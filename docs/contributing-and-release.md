# Contributing And Release

Contributions should improve the reliability of the research record without blurring evidence and speculation.

Use this file for production-readiness expectations. Use `../CONTRIBUTING.md`
for the contributor-facing pull request checklist and `release-readiness.md` for
the final public-release gate.

## Contribution Checklist

Before opening a pull request:

- Run `git status --short --branch`.
- Keep dossier claims and notebook hypotheses separate.
- Use stable IDs for new claims, observations, sequences, hypotheses, and readings.
- Add source limits and confidence labels.
- Check public-safe handling.
- Record verification commands in the pull request.

## Review Checklist

Reviewers should check:

- Source entries are findable and public-safe.
- Claims are small and traceable.
- Confidence labels are justified.
- Notebook hypotheses include tests.
- Promoted conclusions cite evidence and state open risks.
- Existing IDs are not renumbered without a migration note.

## Release Expectations

A release should include:

- Corpus scope.
- Source registry changes.
- New or changed evidence IDs.
- Promoted conclusions.
- Rejected or superseded hypotheses worth noting.
- Known limitations.
- Commands or manual checks used for verification.

Before tagging or publishing, also complete `release-readiness.md`.

This repository intentionally does not use GitHub Actions. Run `./scripts/ci.sh`
locally before merging or tagging.

## Versioning

Use conservative release language. A release may be production-ready as a
research workflow even when there are no decipherment results. Do not imply
scholarly acceptance unless the repository contains explicit, cited support.
