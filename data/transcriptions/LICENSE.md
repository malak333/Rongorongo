# Transcription Data License

The files in this directory are **not** covered by the repository's CC BY 4.0
or MIT terms. Each row of `lines.csv` names its own source and license.

## Text D (Échancrée tablet), lines Da1–Da8 and Db1–Db6

- **Source:** Lorenzo Lastilla, Roberta Ravanelli, Miguel Valério, and Silvia
  Ferrara, "Modelling the Rongorongo tablets: A new transcription of the
  Échancrée tablet and the foundation for decipherment attempts," *Digital
  Scholarship in the Humanities* 37(2), 2022 (online 2021).
  https://academic.oup.com/dsh/article/37/2/497/6387816
- **Copyright:** © The Author(s) 2021. Published by Oxford University Press on
  behalf of EADH.
- **License:** Creative Commons Attribution-NonCommercial 4.0 International
  (CC BY-NC 4.0), https://creativecommons.org/licenses/by-nc/4.0/
- **Use restriction:** non-commercial use only. Anyone reusing these rows must
  credit the authors and the article above.
- **Changes made:** the 14 transcription lines from section 4.1 are copied
  verbatim, one row per line, and checked character by character against the
  typeset PDF (pp. 509–510). `raw_text` is the printed text. `marked_text` is
  the same text with the article's underlining (doubtful readings) wrapped in
  `{}`; this project added the braces, not the authors. Row metadata (line id,
  side, source id, license, notes) was added by this project. No readings
  were altered.

### Verification against the typeset article

- The first import came from the publisher's web page. Checking it against
  the PDF corrected three errors in the web text: Da6 ends `60•` (not `60.`),
  Db1 has `3••/40•` (not `3../40.`), and Db3 has an en dash where the web
  text showed `--` at a line break.
- Underlining was read from page images. It can cover a whole sign, single
  digits of a sign (`2{0}7`), a ligature join (`2V{.}3`), or an element
  separator (`{55–}9`). An underlined separator is read as a doubtful
  boundary; the authors' note on Db6.2–3 supports this.
- `(−)` is not defined in the article. The authors' element numbering
  (Da1.7, Da1.18–19) and the numbered drawings (Figs. 16–17) only agree if it
  separates two elements, so the parser treats it as a possible-join break.
- With that rule the parser counts 211 elements, and every line matches the
  drawings' numbering except Da7: its printed transcription has 15 elements,
  but Fig. 16 numbers 16. That is a discrepancy within the source. The
  authors report 212.
- All 32 element positions the authors cite in sections 4.2–4.3 match the
  parser's positions; `tests/e2e_cli.rs` checks them.

Derived output (sign frequencies, repeated sequences) produced by the
`rongorongo analyze` commands is derived from this data and carries the same
attribution and non-commercial terms.
