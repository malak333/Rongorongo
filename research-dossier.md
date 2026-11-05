# Rongorongo Research Dossier

This document is the evidence layer. It records what is known, disputed, and testable about Rongorongo before any decipherment claim is promoted.

## 1. Project Scope

Phase one covers the canonical, best-attested Rongorongo objects with established transcriptions. Disputed, fragmentary, or weakly attested material is excluded until the method is stable.

## 2. Source Registry

| Source | Date | Type | Contribution | Limits | Reliability |
|---|---:|---|---|---|---|
| SRC-001: Thomas S. Barthel, Grundlagen zur Entzifferung der Osterinselschrift | 1958 | scholarly-corpus | Foundational sign catalogue and corpus nomenclature | Historic drawings/transcriptions require checking against later palaeography | High |
| SRC-002: Steven Roger Fischer, Rongorongo: The Easter Island Script | 1997 | scholarly-monograph | Catalogue and interpretation history | Decipherment proposals require independent tests | Medium |
| SRC-003: Lastilla et al., Modelling the Rongorongo tablets | 2022 | peer-reviewed-open-access | Modern 3D and palaeographic analysis of Text D | Focused on Text D | High |
| SRC-004: Kohaumotu / rongorongo.org digitized corpus pages | n.d. | digital-corpus-reference | Public corpus navigation and transliteration notes | Needs scholarly confirmation before high-confidence use | Medium |
| SRC-005: Omniglot Rongorongo overview | n.d. | expert-secondary-web | Orientation and public-facing summary | Not suitable for detailed corpus claims | Low |
| SRC-006: Ferrara et al., The invention of writing on Rapa Nui (Scientific Reports, CC BY 4.0) | 2024 | peer-reviewed-open-access | Radiocarbon dates for the wood of Tablets A–D | Dates the wood, not the inscription (terminus post quem) | High |
| SRC-007: Valério, Lastilla, Ravanelli, The Rongorongo Tablet C (Lingue e linguaggio) | 2022 | peer-reviewed-article | New transcription of Tablet C | Paywalled; not accessed; transcription cannot be imported | High |

## 3. Corpus Scope

| Object | Catalog ID | Location | Sides | Transcription Source | Inclusion Confidence | Notes |
|---|---|---|---|---|---|---|
| Echancree tablet | Text D / D | Congregation of the Sacred Hearts of Jesus and Mary, Rome | Da; Db | SRC-003 | High | First seeded canonical object because open-access modern analysis is available. |

## 4. Transcription Systems

Record sign inventory systems, glyph numbering schemes, normalization choices, and known disagreements here.

| System / Author | Source | Scope | Strengths | Limits | Adopted? |
|---|---|---|---|---|---|
| Barthel numerical transliteration | SRC-001; SRC-003 | Corpus-wide reference system | Common reference point for signs and variants | Includes variants and ligatures; not definitive | Provisionally |
| Lastilla et al. Text D palaeographic analysis | SRC-003 | Text D | Based on 3D model and close palaeographic examination | One inscription only; `(−)`, `(=...)`, and edge brackets are not defined in the article; printed transcription of Da7 has one element fewer than Fig. 16 | For Text D. Imported verbatim to `data/transcriptions/lines.csv` under CC BY-NC 4.0, checked against the typeset PDF, with underlined doubtful readings in `marked_text` |

## 5. Directionality And Reading Order

Claims about reading order should state whether they assume reverse boustrophedon or another ordering.

| Claim | Source / Observation | Agreement / Conflict | Confidence | Notes |
|---|---|---|---|---|
| Rongorongo lines are typically treated as reverse boustrophedon. | SRC-003 | Broadly consistent with standard descriptions, but side order can remain uncertain for individual objects. | Medium | Text D side order has multiple theoretical possibilities, so sequence claims must state assumptions. |

## 6. Linguistic And Cultural Context

Default frame: Rapa Nui / Eastern Polynesian. Broader Polynesian comparison may be used as supporting context, but similarity alone is not evidence of decipherment.

| Claim | Source | Context Type | Confidence | Notes |
|---|---|---|---|---|
| Rapa Nui / Eastern Polynesian is the default comparison frame. | Project decision (no source cited) | linguistic-frame | Medium | Comparative similarity is a constraint, not proof of decipherment. Recorded in the notebook decision log rather than the claims register because no registered source supports it. |

## 7. Prior Decipherment Proposals

Existing proposals are reduced to testable claims before being used in the decipherment notebook.

| Proposal | Source | Core Claim | Testable Predictions | Scholarly Reception | Confidence |
|---|---|---|---|---|---|
| Fischer-style semantic readings | SRC-002 | Some sequences may encode chant-like or genealogical material. | Claims should predict repeatable sign/sequence behavior across canonical texts. | Contested; use as hypothesis only. | Low |

## 8. Evidence Tables

Use this section for small, checkable claims extracted from sources or corpus observations.

| Claim ID | Claim | Evidence | Source Type | Agreement / Conflict | Confidence | Notes |
|---|---|---|---|---|---|---|
| C-001 | Rongorongo remains undeciphered. | SRC-003; SRC-005 | peer-reviewed plus secondary overview | Agreement | Medium | This project must not claim fluent readings. Medium until additional high-reliability state-of-field sources are registered. |
| C-002 | Accurate transcription is a prerequisite for decipherment attempts. | SRC-003 | peer-reviewed-open-access | Agreement within project method | High | This governs the Rust validation scope. |
| C-004 | The wood of Text D dates to 1438–1462 cal AD (95.4%), or 1483–1520 after the felling-date correction; a terminus post quem, not a date for the inscription. | SRC-006 | peer-reviewed-open-access | Single sample | High | The authors' inference that writing predates European contact is not promoted here. |

## 9. Current Conclusions

Only promote conclusions here after they are traceable and have survived competing explanations.

| Conclusion | Basis | Confidence | Open Risks |
|---|---|---|---|
| Production-ready v1 should validate research artifacts rather than claim decipherment. | Project scope plus C-001 and C-002 | High | A richer corpus may require revised validation rules. |
| Text D's Da3 contains a repeated structural template, and the ligature 200.52x is specific to it within Text D. | O-003, O-004, S-004 | High | Significance cannot be judged without other tablets (H-003). |
| No other tablet's transcription is currently available under redistributable terms, so corpus-wide tests are blocked. | Survey in `docs/project-conclusion.md` | High | Permission from SRC-007's authors or CEIPP would change this. |
