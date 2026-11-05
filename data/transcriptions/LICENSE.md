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
  verbatim, one row per line. Row metadata (line id, side, source id, license,
  notes) was added by this project. No readings were altered.

### Known losses from plain-text capture

- The article marks doubtful readings with **underlining**. Underlining is not
  preserved in `raw_text`. Before relying on any element, check it against
  the typeset article.
- The project parser counts 208 graphic elements. The authors report 212 in
  their conservative count. This gap is unresolved.

Derived output (sign frequencies, repeated sequences) produced by the
`rongorongo analyze` commands is derived from this data and carries the same
attribution and non-commercial terms.
