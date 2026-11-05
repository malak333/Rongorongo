# Draft: Request to Share Transcriptions (not sent)

This is a draft for the project owner to review, edit, and send if they
choose. Nothing has been sent. It supports option 1 in
`docs/project-conclusion.md` section 6.

**To:** the corresponding authors of SRC-003 and SRC-007 (ERC project
INSCRIBE, University of Bologna). Take the current address from the
articles or the project site; do not use one copied from a third-party page.

**Subject:** Request: open licensing of the INSCRIBE Rongorongo transcriptions

---

Dear Professor Ferrara, Dr Valério, and colleagues,

I maintain a small open-source research workspace for structural analysis of
Rongorongo. It is built on your 2022 transcription of the Échancrée tablet
(Text D, *Digital Scholarship in the Humanities* 37(2)), which it uses under
the article's CC BY-NC 4.0 licence with full attribution.

The tools check the transcription against the published article and run
frequency counts, repeat detection, and pattern searches. They report
structure only and propose no readings. Against your text they reproduce
every element position cited in sections 4.2–4.3, including the four
`600/610 – x – 200.52x` repetitions in Da3.

Every remaining question needs a second text: the Da1 ending against Text R,
the `7–600–7` parallel against Text S, and whether the Da3 template recurs
anywhere in the corpus. The only other recent scholarly transcription I know
of is your Tablet C edition (Valério et al. 2022), which I cannot
redistribute under its current terms.

Would you consider releasing the Tablet C transcription, or other INSCRIBE
transcriptions, under an open licence such as CC BY or CC BY-NC? If that is
not possible, would you permit them to be used in this repository with
attribution? If you prefer to keep the data private, I would be glad to run
the analysis on your behalf and share only the derived results with you.

The repository and its evidence rules are here: [repository URL].

With thanks for your work and for publishing Text D openly,

[your name]

---

## Before sending

- Replace `[repository URL]` and `[your name]`.
- Check the claims against the current registers (`cargo run -- audit --strict`).
- If they agree, import per `docs/public-safe-sources.md` and record the
  licence in `data/transcriptions/LICENSE.md`.
