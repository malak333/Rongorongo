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
| O-003 | Text D / Da3 | Four instances of the template 600 or 610, any element, then a 200.52x ligature (Da3.3–5, 6–8, 9–11, 12–14), as in SRC-003 §4.3. | Line-internal | High | Reproduced exactly by `analyze find`; all readings certain. |
| O-004 | Text D | The ligature 200.52x occurs five times, all in Da3; inverted 52x otherwise only in Da4. | Not order-dependent | High | From `analyze sequences --level part`. |
| O-005 | Text D | Only two exact whole-element repeats among certain readings: 200.52x.s–600 (Da3) and 7–600 (Db4, Db6); the latter extends to 7–600–7 at sign level. | Line-internal | Medium | Depends on excluding doubtful readings. |
| O-006 | Text D / Da1 | Da1 ends 522f–99 at Da1.18–19; the 99 is underlined (doubtful), and Barthel read 522. | Line-internal | Medium | SRC-003 compares Rb2; untestable without Text R. |
| O-007 | Text D | 211 graphic elements under this parser vs 212 reported; the gap is Da7, printed with 15 elements but drawn with 16. | Not order-dependent | High | All 32 cited positions match. |

## 4. Repeated Sequences

| Sequence ID | Sequence | Occurrences | Position Pattern | Possible Function | Confidence | Notes |
|---|---|---:|---|---|---|---|
| S-001 | none-recorded | 0 | not-yet-analyzed | none-proposed | Low | Superseded placeholder. |
| S-002 | 200.52x.s–600 | 2 | Da3.5–6, Da3.14–15 | none-proposed | Medium | Exact repeat among certain readings. |
| S-003 | 7–600–7 | 2 | Db4.12–14, Db6.7–9 (sign level) | none-proposed | Medium | SRC-003 compares 7–600–7.10 with Sb6. |
| S-004 | 600/610 – * – 200.52x.* | 4 | Da3.3–5, 6–8, 9–11, 12–14 | none-proposed | High | A template; `*` is any element. |

Unregistered leads (do not cite until registered):

- **522-99.** `522f–99` closes line Da1 at Da1.18–19, matching SRC-003 §4.2.1. The `99` is underlined in the source (a doubtful reading): Barthel read the last element as 522, and the authors propose 99. The authors compare the line ending with `206s–522f–99` in Rb2 and note that `522–99` is attested in Text R. That comparison cannot be checked here until Text R is in the corpus index.
- **Exact element repeats in Text D** (`cargo run -- analyze sequences`): `200.52x.s–600` (Da3.5–6, Da3.14–15) and `7–600` (Db4.12–13, Db6.7–8). Only exact matches count; uncertain, doubtful (underlined), ambiguous, and unparsed elements are excluded, and runs break at doubtful or possible-join boundaries. The authors also note `]380y–2V.3–730` and `1.380y–2V.3–730` in Da5 as identical, but the tool excludes that pair because its `2V.3` join and `380y` are underlined. The `7–600` lead is part of a parallel the authors discuss with Sb6 (SRC-003 §4.3); at sign level it extends to `7–600–7` in Db4 and Db6, matching the authors' note.
- **Da3 structure.** SRC-003 §4.3 describes Da3 as repetitions of "bird-sign 600 (or 610) + changing glyph + 200.52x.appendage". `analyze find "600|610 * 200.52x.*"` returns exactly the four strings the authors list (Da3.3–5, 6–8, 9–11, 12–14), all with certain readings. The template is confirmed against the transcription, but its significance can't be judged from one tablet.
- **The `200.52x` core** (`analyze sequences --level part`) occurs 5 times, always as a ligature and only in Da3; `2–200.52x` occurs twice. Inverted `52x` appears elsewhere only in Da4 (`52x.590.52x`); upright `200.52` occurs once in Db1. Whether the compound or its pattern recurs on other tablets is the obvious next test, and needs more of the corpus. The same section notes variations in Da4, duplications in Db3 (`383–383`, `1.63–1.63`, `2–2`) and Db4 (`50–50`, `700.244–700.244`, `240.8–240.8`), and repeated `445` at the end of Db5.

## 5. Sign Frequency

| Sign | Count | Corpus Scope | Position Notes | Interpretation Risk |
|---|---:|---|---|---|
| 522 | 1 certain, 1 doubtful | Text D (SRC-003 transcription) | Da1.18 (`522f`, followed by `99`); Db5.1 (`52{2}f.700`, last digit underlined) | Do not infer meaning from isolated comparison |
| 99 | 0 certain, 1 doubtful | Text D (SRC-003 transcription) | Da1.19, underlined; Barthel read 522 | Single doubtful occurrence |

Counts come from `cargo run -- analyze frequency` (add `--exact` to keep variant suffixes such as `522f`). The command is the source of truth, so this table records only signs under discussion. `count` is certain readings; underlined, `•`-marked, and `/`-alternative readings count as `possible`.

## 6. Structural Hypotheses

| Hypothesis ID | Hypothesis | Test | Result | Confidence | Status |
|---|---|---|---|---|---|
| H-001 | Accurate palaeographic transcription must precede semantic decipherment attempts. | Validate every semantic reading against a named transcription source before promotion. | Adopted as governance rule. | High | Accepted |
| H-002 | Reverse boustrophedon should be the default directionality assumption unless text-specific evidence says otherwise. | Require sequence observations to record reading-order assumption. | Active validation requirement. | Medium | Active |
| H-003 | The Da3 template is a structured, list-like formula rather than an accident of one text. | Search other tablets for `600\|610 * 200.52x.*` and the 200.52x ligature. | Untestable: no other tablet's transcription is available under a usable license. | Low | Active |

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
