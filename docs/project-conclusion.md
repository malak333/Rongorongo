# Project Conclusion (v1)

Status as of 2026-11-05 (release v1.0.0). This report closes the first phase of the project:
what it set out to do, what it established, why it stops here, and what
would reopen it. Every finding cites a register id, so it can be checked with
`cargo run -- audit --strict` and the commands listed below.

## 1. What the project set out to do

Build an evidence-based workspace for Rongorongo: first a traceable research
dossier, then a cautious structural analysis, with semantic readings allowed
only where structural evidence is strong (see `README.md`).

## 2. What was built

- **Governance tooling.** `audit --strict` enforces controlled vocabularies,
  caps confidence at the strongest cited source, re-applies promotion gates to
  promoted rows, restricts semantic readings to promoted observations, and
  keeps the Markdown dossier and notebook in step with the CSV registers
  (`docs/evidence-rules.md`).
- **One verified transcription.** Text D (Échancrée), 14 lines from SRC-003,
  is stored verbatim under CC BY-NC 4.0. It was checked character by character
  against the typeset article, with the authors' underlined doubtful readings
  captured (`data/transcriptions/`). All 32 element positions the authors cite
  match the parser.
- **Structural analysis commands.** Sign frequencies, exact repeats at element
  and sign level, and pattern search (`docs/cli-usage.md`). They report
  structure only and never propose meanings.

## 3. What the evidence supports

| Id | Finding | Confidence |
|---|---|---|
| C-001 | Rongorongo remains undeciphered. | Medium |
| C-004 | The wood of Text D dates to 1438–1462 cal AD (95.4%), or 1483–1520 after the felling-date correction. This is a terminus post quem for the tablet, not a date for its inscription. | High |
| O-003, S-004 | Line Da3 repeats a template four times: 600 or 610, a varying element, then a `200.52x` ligature. This confirms SRC-003 §4.3 against the transcription. | High |
| O-004 | The ligature `200.52x` occurs five times, all in Da3. Inverted `52x` occurs elsewhere only in Da4. | High |
| O-005, S-002, S-003 | Among certain readings, Text D has only two exact whole-element repeats (`200.52x.s–600`, `7–600`); at sign level `7–600–7` recurs in Db4 and Db6. | Medium |
| O-006 | Da1 ends `522f–99`, but the `99` is a doubtful reading (Barthel read 522). | Medium |
| O-007 | The transcription has 211 elements against the authors' 212; the gap is an inconsistency in the source's Da7. | High |

## 4. What the evidence does not support

- **No semantic reading.** R-001 records the explicit non-claim, and nothing
  in this project changes it.
- **No claim that the Da3 template means anything.** H-003, that it is a
  structured, list-like formula, stays at Low confidence because it cannot be
  tested on one tablet.
- **No claim about when writing began.** C-004 dates wood. The SRC-006
  authors argue that the result supports writing before European contact,
  but they also note the inscription may be later than the wood. This project
  does not promote that inference.

## 5. Why the project stops here

Every open question needs a second tablet:
- the 522–99 ending needs Text R;
- the `7–600–7.10` parallel needs Text S;
- the Text C parallels SRC-003 mentions need Text C;
- H-003 needs the whole corpus.

A survey on 2026-11-05 found no other transcription that is both scholarly
and redistributable:

| Source | Covers | Terms | Usable here |
|---|---|---|---|
| SRC-003, Lastilla et al. 2022 | Text D | CC BY-NC 4.0 | Yes, imported |
| SRC-007, Valério et al. 2022 | Text C | Publisher paywall; repository copy restricted | No |
| Lastilla et al. 2019, ISPRS Archives | Text C (3D model) | CC BY 4.0 | No transcription in the paper |
| SRC-006, Ferrara et al. 2024 | Dating of A–D | CC BY 4.0 | No transcription; dating registered as C-004 |
| SRC-001, Barthel 1958 | Whole corpus | In copyright | No |
| SRC-004, CEIPP / Kohaumotu digitized corpus | Barthel encodings | No license stated; offered to bona fide researchers | No, without permission |
| rongopy (GitHub) | Corpus, Barthel and Horley codes | GPL-3.0 on the repository | No: derived from Barthel, which the repository cannot relicense |
| Self-published Zenodo deposits | Various | Various Creative Commons | No: not peer-reviewed, and data provenance traces to Barthel |
| Van Vyve 2026, ULiège master's thesis (promotor Ravanelli) | Glyph extraction from 3D models of Texts A–D (INSCRIBE corpus) | Embargoed until 2027-06-30; copyright, no open license | No: no transcription; sign inventory named as future work |

Daily re-checks of the same sources found nothing new that qualifies (last
check 2026-11-05). The INSCRIBE glyph-extraction work (Van Vyve 2026) is the
most likely path to an A–D sign inventory; recheck it after its embargo ends.

So the project has done what can be done with redistributable material. The
remaining work is not more code.

## 6. What would reopen it

1. **Permission or an open release** (draft request: `docs/permission-request-draft.md`). Ask the SRC-003 and SRC-007 authors
   (ERC project INSCRIBE, University of Bologna) whether their Tablet C and
   other transcriptions can be shared under an open license. Or ask CEIPP for
   permission to redistribute its digitized corpus. Any of these would let
   the existing pipeline import more tablets in hours.
2. **Private analysis.** The tools run on any `--root`. A researcher with
   lawful access to other transcriptions can keep them outside this
   repository and run `analyze find "600|610 * 200.52x.*"` against them to
   test H-003, then commit only the derived observations.
3. **Original transcription.** A reading made from public 3D models or
   photographs would be the transcriber's own work and could be committed.
   It needs palaeographic expertise and should be checked against published
   readings.

## 7. How to check this report

```sh
./scripts/ci.sh
cargo run -- audit --strict
cargo run -- analyze find "600|610 * 200.52x.*"
cargo run -- analyze sequences --level part
cargo run -- transcriptions elements --problems
```
