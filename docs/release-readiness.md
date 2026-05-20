# Release Readiness

Use this checklist before making the repository public or tagging a release.

Related production-readiness docs:

- `workflow.md`
- `evidence-rules.md`
- `cli-usage.md`
- `e2e-strategy.md`
- `public-safe-sources.md`
- `contributing-and-release.md`

## Source Safety

- No credentials, tokens, private keys, account exports, or private contact
  details are committed.
- No copyrighted scans, images, article PDFs, book chapters, bulk
  transcriptions, or long excerpts are committed.
- Any third-party material that remains in the repository has clear license,
  public-domain, or permission evidence.
- Source records are linked or summarized rather than redistributed.
- Cultural, museum, archive, and community restrictions are respected even when
  a file is technically accessible online.

## Research Integrity

- `research-dossier.md` contains evidence, source summaries, and confidence
  labels.
- `decipherment-notebook.md` contains hypotheses, tests, tentative readings, and
  rejected ideas.
- Dossier conclusions are traceable to cited sources or explicit corpus
  observations.
- Low-confidence ideas are not presented as conclusions.
- Prior decipherment proposals are framed as testable hypotheses, not adopted
  authorities.

## Repository Hygiene

- `LICENSE`, `SECURITY.md`, `CONTRIBUTING.md`, `CHANGELOG.md`, and `.gitignore`
  are present.
- `CHANGELOG.md` reflects release-relevant changes.
- Ignored paths cover local secrets, generated files, dependency directories,
  and raw source-material folders.
- The git working tree is clean before tagging or publishing.

## Manual Review Commands

Run these from the repository root:

```sh
./scripts/ci.sh
git status --short
find . -maxdepth 3 -type f \( -name '.env*' -o -name '*secret*' -o -name '*token*' -o -name '*.pem' -o -name '*.p12' \) -print
find . -maxdepth 3 -type f \( -name '*.pdf' -o -name '*.jpg' -o -name '*.jpeg' -o -name '*.png' -o -name '*.tif' -o -name '*.tiff' -o -name '*.webp' \) -print
```

Any intentional match should be reviewed before release and documented with a
clear reason.
