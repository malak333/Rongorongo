# E2E Strategy

The current repository has no application runtime, service backend, or executable CLI. E2E coverage is therefore documentation-level.

## Current E2E Path

A reviewer should be able to trace this path manually:

1. A source is registered in `research-dossier.md`.
2. A corpus object is listed in `data/corpus-index.csv`.
3. A small claim or observation receives a stable ID.
4. A hypothesis in `decipherment-notebook.md` cites the claim or observation.
5. A supported result is promoted back to the dossier with confidence and open risks.
6. Release notes summarize the promoted result without relying on private files.

This is the minimum end-to-end readiness path for research documentation.

## Future Automated E2E

When tooling is added, E2E tests should use fixture data that is safe to publish. Tests should verify:

- Source registry parsing.
- Corpus index parsing.
- Stable ID references across dossier and notebook files.
- Promotion gates for conclusions.
- Public-safe report generation.
- Nonzero failures for missing evidence links or unsafe source content.

## Test Data Rules

Fixture data must be:

- Synthetic, public domain, or clearly licensed for repository use.
- Small enough to audit in code review.
- Free of copyrighted scans, long source excerpts, or private notes.

## CI Expectations

Once a CLI exists, CI should run:

```sh
rongorongo validate --strict
rongorongo report --public-safe --dry-run
```

These command names are a proposed contract, not current repository commands.

