# E2E Strategy

The repository has automated CLI E2E coverage for the full research-governance workflow. There is no service backend or browser runtime.

## Automated E2E Path

The E2E tests in `tests/e2e_cli.rs` verify that a reviewer can trace this path:

1. Strict validation passes for the public workspace.
2. The full audit checks schema and cross-reference integrity.
3. Corpus, source, hypothesis, claim, and observation registers can be listed.
4. JSON output is available for local scripts or downstream tooling.
5. Source intake emits a public-safe source-registry row template.
6. Claim and hypothesis promotion gates pass for known seed records.

## Test Data Rules

Fixture data must be:

- Synthetic, public domain, or clearly licensed for repository use.
- Small enough to audit in code review.
- Free of copyrighted scans, long source excerpts, or private notes.

## Local Gate Expectations

The local release gate is:

```sh
./scripts/ci.sh
```

That script executes formatting, clippy, unit tests, E2E tests, build, strict validation, full audit, list commands, intake template generation, and promotion-gate checks.
