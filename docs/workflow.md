# Workflow

This repository uses a two-track workflow:

- `research-dossier.md` is the evidence layer.
- `decipherment-notebook.md` is the hypothesis layer.

The goal is to keep every public claim traceable while still allowing cautious exploration.

## Standard Flow

1. Register the source in `research-dossier.md`.
2. Add or update corpus metadata in `data/corpus-index.csv`.
3. Extract small, checkable claims into the dossier evidence tables.
4. Record structural observations or tests in `decipherment-notebook.md`.
5. Compare competing explanations before assigning confidence.
6. Promote supported findings back into the dossier only after the promotion gate passes.

## Review Roles

- Research changes should be reviewed for source quality, citation completeness, and public-safe handling.
- Notebook changes should be reviewed for testability, corpus scope, and separation from promoted conclusions.
- Release changes should be reviewed for reproducibility and clear limitations.

## Evidence Promotion

A notebook item can be promoted only when it has:

- A stable ID.
- A defined corpus scope.
- A source or observation chain.
- A confidence label.
- Known conflicts or limits.
- A public-safe representation.

Promotion should preserve the original notebook trail. Do not delete failed or superseded hypotheses if they explain why a path was rejected.

## Concurrent Work

When multiple contributors are editing the repository:

- Check `git status --short --branch` before changing files.
- Avoid rewriting unrelated sections.
- Keep table row additions scoped and stable.
- Prefer adding new IDs over renumbering existing IDs.
- Resolve conflicts by preserving both evidence trails unless one is demonstrably obsolete.

