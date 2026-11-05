# Public-Safe Source Handling

This repository should remain safe to publish publicly.

## Allowed

- Bibliographic citations.
- Short paraphrased summaries.
- Museum or archive identifiers.
- Links to official public records.
- Small derived observations, such as sequence counts or object metadata.
- Public domain or clearly licensed material with attribution.

## Not Allowed

- Copyrighted scans or restricted images.
- Long excerpts from books, articles, or catalog records.
- Private research notes copied from third parties.
- Paywalled text pasted into repository files.
- Images or transcriptions with unclear redistribution rights.

## Openly Licensed Transcriptions

Transcriptions may be committed under `data/transcriptions/` when all of these hold:

- The source states an open license that permits redistribution (for example
  CC BY or CC BY-NC). "Freely readable" or "open access" alone is not enough.
- `data/transcriptions/LICENSE.md` names the source, rights holder, license,
  any use restriction, and what was changed. The audit fails if it is missing.
- Every row records its `source_ref` and `license`.
- Lines are copied verbatim, with known capture losses documented. Derived
  data is never written back into the transcription rows.
- Non-commercial (NC) material stays in that directory and is never folded
  into files covered by the repository's CC BY 4.0 or MIT terms.

## Source Notes

If private or restricted material is needed for personal review, keep it outside
the repository. Repository entries should record enough metadata for another
researcher to locate the source independently.

## Derived Data

Derived data may be committed when it is small, factual, and does not reconstruct a restricted source. Examples include:

- Object names and catalog IDs.
- Inclusion confidence.
- Count summaries.
- Stable observation IDs.

If derived data becomes large enough to substitute for a restricted source, do not commit it without a rights review.
