# Rongorongo Decipherment Notebook

This document is the hypothesis layer. It may contain speculative ideas, but every hypothesis must be labeled, testable, and traceable to a source or corpus observation.

## 1. Method

Work from structure before meaning:

1. Establish corpus and transcription assumptions.
2. Track repetitions, sign positions, line patterns, and object-level structure.
3. Test prior proposals as falsifiable claims.
4. Promote only well-supported findings back into the research dossier.

## 2. Active Assumptions

| Assumption | Basis | Risk | Status |
|---|---|---|---|
| Canonical corpus only in phase one | Project scope | May miss useful comparative material | Active |
| Rapa Nui / Eastern Polynesian first | Linguistic frame | Similarity can mislead | Active |
| Structural progress before semantics | Reliability standard | Slower semantic output | Active |

## 3. Corpus Observations

| Observation ID | Object / Side | Observation | Reading Order Assumed | Confidence | Notes |
|---|---|---|---|---|---|
| O-001 | Text D / Da-Db | Side ordering has multiple theoretical possibilities in the modern Text D discussion. | Reverse boustrophedon at line level; side order unresolved | Medium | Use this as a warning against unqualified sequence claims. |
| O-002 | Text D | Text D is the first seeded canonical object because open-access modern palaeographic analysis is available. | Text-specific | High | Corpus governance observation, not a semantic reading. |

## 4. Repeated Sequences

| Sequence ID | Sequence | Occurrences | Position Pattern | Possible Function | Confidence | Notes |
|---|---|---:|---|---|---|---|
| S-001 | none-recorded | 0 | not-yet-analyzed | none-proposed | Low | No repeated sequence has been registered yet. |

Unregistered leads (do not cite until registered):

- **522-99.** The imported transcription confirms `522f–99` closes line Da1 (parser elements Da1.17–18). An earlier note placed it at Da1.18–19; the difference is a numbering question not yet reconciled (see the 208 vs 212 element count in `data/transcriptions/LICENSE.md`). SRC-003's comparison with Text R remains unverified because Text R is not in the corpus index.
- **Exact element repeats in Text D** (`cargo run -- analyze sequences`): `200.52x.s–600` (Da3.5–6, Da3.14–15), `2V.3–730` (Da5.2–3, Da5.10–11), `7–600` (Db4.12–13, Db6.7–8). These are exact matches only; uncertain, ambiguous, and unparsed elements are excluded.
- **Da3 structure.** Da3 alternates `600` with compounds built on `200.52x` (`200.52x.s`, `200.52x.52`, `200.52x.69`). This is visible by eye but is not an exact element repeat, so the sequence command does not report it. A part-level comparison would be needed to test it.

## 5. Sign Frequency

| Sign | Count | Corpus Scope | Position Notes | Interpretation Risk |
|---|---:|---|---|---|
| 522 | 2 | Text D (SRC-003 transcription) | Da1.17 (`522f`, followed by `99`); Db5.1 (`522f.700`) | Do not infer meaning from isolated comparison |

Counts come from `cargo run -- analyze frequency` (add `--exact` to keep variant suffixes such as `522f`). The command is the source of truth, so this table records only signs under discussion. Underlined doubtful readings in the source are not captured, so treat every count as provisional.

## 6. Structural Hypotheses

| Hypothesis ID | Hypothesis | Test | Result | Confidence | Status |
|---|---|---|---|---|---|
| H-001 | Accurate palaeographic transcription must precede semantic decipherment attempts. | Validate every semantic reading against a named transcription source before promotion. | Adopted as governance rule. | High | Accepted |
| H-002 | Reverse boustrophedon should be the default directionality assumption unless text-specific evidence says otherwise. | Require sequence observations to record reading-order assumption. | Active validation requirement. | Medium | Active |

## 7. Prior Proposal Tests

| Proposal | Testable Claim | Corpus Test | Result | Status | Notes |
|---|---|---|---|---|---|
| Fischer-style semantic readings | Claimed readings should predict repeatable structural behavior. | Not yet tested beyond source intake. | Incomplete | Draft | Keep separate from accepted conclusions. |

## 8. Tentative Semantic Readings

Semantic readings are allowed only where structural evidence is already strong.

| Reading ID | Sign / Sequence | Proposed Meaning | Evidence | Alternative Explanations | Confidence |
|---|---|---|---|---|---|
| R-001 | none | none proposed | Structural evidence is not yet sufficient. | N/A | Low |

## 9. Rejected Hypotheses

Rejected ideas remain useful. Record why each failed so the same weak pattern is not rediscovered later.

| Hypothesis ID | Rejected Claim | Reason Rejected | Evidence | Date |
|---|---|---|---|---|
| none | none | No rejected hypotheses have been recorded yet. | N/A | 2026-05-19 |

## 10. Decision Log

| Decision | Alternatives Considered | Reason |
|---|---|---|
| Use two-track dossier and notebook | Dossier-only, notebook-only | Keeps evidence and speculation separate while allowing controlled exploration |
| Start with canonical corpus only | Include related artifacts, include everything | Reduces noise while the method is being established |
| Treat prior decipherments as hypotheses | Survey only, ignore initially | Allows useful claims to be tested without granting authority |
| Use Rapa Nui / Eastern Polynesian as the default comparison frame | Broad Polynesian first, no default frame | Geographic and historical proximity; kept as a project decision because no registered source is cited for it |
| Use layered confidence | Very conservative only, fully exploratory | Preserves ideas while preventing low-confidence claims from becoming conclusions |
