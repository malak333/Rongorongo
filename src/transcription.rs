//! Parsing and structural analysis of verbatim transcription lines.
//!
//! Lines are stored exactly as published (`data/transcriptions/lines.csv`).
//! Everything here is derived on demand, so a parser change never rewrites
//! the source data.
//!
//! Notation (Barthel-based, as used by Lastilla et al. 2022 for Text D):
//!
//! - `–` (en dash) separates graphic elements. `--` is treated the same way
//!   and flagged, because it appears once in the source and is not explained.
//! - `.` joins two shapes horizontally and `:` joins them vertically
//!   (ligatures); each joined shape is a *part*.
//! - `/` separates equally possible transcriptions of one part.
//! - `•` marks uncertainty about a glyph or part of a glyph.
//! - Letter suffixes (`a`–`e`, `f`, `s`, `x`, `y`, `V`) are palaeographic
//!   variants; a leading `s` or a bare `s` part is Barthel's ornament element.
//! - `(...)` content is kept as an annotation and excluded from counts, and
//!   acts as a part boundary; the source does not define `(−)` or `(=...)`.
//! - `[…]` is recorded as a lacuna. Leading `]` and trailing `[` are stripped;
//!   the source does not define them.
//!
//! The source marks doubtful readings with underlining, which does not
//! survive in plain text. Treat every element as possibly doubtful until it
//! has been checked against the typeset article.

use serde::Serialize;

#[derive(Debug, Clone, serde::Deserialize, Serialize)]
pub struct TranscriptionLine {
    pub line_id: String,
    pub corpus_id: String,
    pub side: String,
    pub source_ref: String,
    pub license: String,
    pub raw_text: String,
    pub notes: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Element {
    pub line_id: String,
    /// 1-based position within the line, as counted by this parser.
    pub position: usize,
    pub raw: String,
    pub parts: Vec<Part>,
    pub annotations: Vec<String>,
    pub lacuna: bool,
    /// Set when the element was split on the unexplained `--` separator.
    pub irregular_separator: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Part {
    pub raw: String,
    pub alternatives: Vec<Sign>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Sign {
    pub raw: String,
    /// Barthel base number, when the sign has one.
    pub base: Option<u32>,
    /// Full code without uncertainty marks, e.g. `522f`, `s594`, `s`.
    pub code: Option<String>,
    pub uncertain: bool,
    /// True when the sign does not match the notation grammar above.
    pub unparsed: bool,
}

impl Element {
    /// True when any part is marked uncertain, unparsed, or ambiguous, or the
    /// element sits in a lacuna. Such elements are excluded from sequence
    /// matching so that uncertainty cannot manufacture a repeat.
    pub fn is_uncertain(&self) -> bool {
        self.lacuna
            || self.parts.is_empty()
            || self.parts.iter().any(|part| {
                part.alternatives.len() != 1
                    || part.alternatives.iter().any(|s| s.uncertain || s.unparsed)
            })
    }

    /// Normalized element text used to compare elements: annotations,
    /// whitespace, and lacuna marks removed.
    pub fn key(&self) -> String {
        self.parts
            .iter()
            .map(|part| part.raw.as_str())
            .collect::<Vec<_>>()
            .join(".")
    }
}

pub fn parse_line(line_id: &str, raw_text: &str) -> Vec<Element> {
    let body = raw_text.trim();
    let body = body.strip_prefix(']').unwrap_or(body);
    let body = body.strip_suffix('[').unwrap_or(body).trim();

    let mut elements = Vec::new();
    for chunk in body.split('–') {
        let pieces: Vec<&str> = chunk.split("--").collect();
        let irregular = pieces.len() > 1;
        for piece in pieces {
            let piece = piece.trim();
            if piece.is_empty() {
                continue;
            }
            elements.push(parse_element(line_id, elements.len() + 1, piece, irregular));
        }
    }
    elements
}

fn parse_element(line_id: &str, position: usize, raw: &str, irregular: bool) -> Element {
    let lacuna = raw.contains("[…]") || raw.contains("[...]");
    let without_lacuna = raw.replace("[…]", "").replace("[...]", "");

    let mut annotations = Vec::new();
    let mut text = String::new();
    let mut depth = 0usize;
    let mut current = String::new();
    for c in without_lacuna.chars() {
        match c {
            '(' => {
                depth += 1;
                if depth == 1 {
                    current.clear();
                    continue;
                }
            }
            ')' if depth > 0 => {
                depth -= 1;
                if depth == 0 {
                    annotations.push(current.clone());
                    // An annotation separates what is on either side of it.
                    text.push('.');
                    continue;
                }
            }
            _ => {}
        }
        if depth > 0 {
            current.push(c);
        } else if !c.is_whitespace() {
            text.push(c);
        }
    }

    let parts = text
        .split(['.', ':'])
        .filter(|part| !part.is_empty())
        .map(|part| Part {
            raw: part.to_string(),
            alternatives: part.split('/').map(parse_sign).collect(),
        })
        .collect();

    Element {
        line_id: line_id.to_string(),
        position,
        raw: raw.to_string(),
        parts,
        annotations,
        lacuna,
        irregular_separator: irregular,
    }
}

/// Grammar: `[s]? digits letters* •*`, or a bare `s`, or a bare run of `•`.
pub fn parse_sign(raw: &str) -> Sign {
    let uncertain = raw.contains('•');
    let core = raw.trim_end_matches('•');

    if core.is_empty() && uncertain {
        // A bare `•`: an uncertain, unidentified glyph.
        return Sign {
            raw: raw.to_string(),
            base: None,
            code: None,
            uncertain: true,
            unparsed: false,
        };
    }
    if core == "s" {
        return Sign {
            raw: raw.to_string(),
            base: None,
            code: Some("s".to_string()),
            uncertain,
            unparsed: false,
        };
    }

    let rest = core.strip_prefix('s').unwrap_or(core);
    let digits: String = rest.chars().take_while(char::is_ascii_digit).collect();
    let suffix = &rest[digits.len()..];
    let valid = !digits.is_empty()
        && suffix.chars().all(|c| c.is_ascii_alphabetic())
        && !core.contains('•');

    if valid {
        Sign {
            raw: raw.to_string(),
            base: digits.parse().ok(),
            code: Some(core.to_string()),
            uncertain,
            unparsed: false,
        }
    } else {
        Sign {
            raw: raw.to_string(),
            base: None,
            code: None,
            uncertain,
            unparsed: true,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct FrequencyRow {
    pub sign: String,
    /// Occurrences as an unambiguous, certain reading.
    pub count: usize,
    /// Occurrences marked `•` (uncertain) or offered as one of several `/` alternatives.
    pub possible: usize,
    pub lines: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FrequencyReport {
    pub elements: usize,
    pub parts: usize,
    pub rows: Vec<FrequencyRow>,
    pub unparsed: Vec<String>,
}

/// Count signs by Barthel base number (or by full code when `exact`).
pub fn frequency(elements: &[Element], exact: bool) -> FrequencyReport {
    use std::collections::BTreeMap;
    let mut counts: BTreeMap<(u32, String), FrequencyRow> = BTreeMap::new();
    let mut unparsed = Vec::new();
    let mut parts = 0;

    for element in elements {
        for part in &element.parts {
            parts += 1;
            let ambiguous = part.alternatives.len() > 1;
            for sign in &part.alternatives {
                if sign.unparsed {
                    unparsed.push(format!(
                        "{}.{} {}",
                        element.line_id, element.position, part.raw
                    ));
                    continue;
                }
                let (Some(base), Some(code)) = (sign.base, sign.code.as_ref()) else {
                    continue;
                };
                let label = if exact {
                    code.clone()
                } else {
                    base.to_string()
                };
                let row = counts
                    .entry((base, label.clone()))
                    .or_insert_with(|| FrequencyRow {
                        sign: label,
                        count: 0,
                        possible: 0,
                        lines: Vec::new(),
                    });
                if ambiguous || sign.uncertain {
                    row.possible += 1;
                } else {
                    row.count += 1;
                }
                if !row.lines.contains(&element.line_id) {
                    row.lines.push(element.line_id.clone());
                }
            }
        }
    }

    let mut rows: Vec<FrequencyRow> = counts.into_values().collect();
    rows.sort_by(|a, b| {
        (b.count, b.possible)
            .cmp(&(a.count, a.possible))
            .then_with(|| natural_key(&a.sign).cmp(&natural_key(&b.sign)))
    });

    FrequencyReport {
        elements: elements.len(),
        parts,
        rows,
        unparsed,
    }
}

fn natural_key(sign: &str) -> (u32, String) {
    let digits: String = sign
        .trim_start_matches('s')
        .chars()
        .take_while(char::is_ascii_digit)
        .collect();
    (digits.parse().unwrap_or(u32::MAX), sign.to_string())
}

#[derive(Debug, Clone, Serialize)]
pub struct RepeatedSequence {
    pub sequence: String,
    pub length: usize,
    pub count: usize,
    /// `line.start-end`, using this parser's 1-based element positions.
    pub locations: Vec<String>,
}

/// Find maximal repeated runs of certain elements within lines.
///
/// Runs never cross a line boundary or an uncertain element. A run is dropped
/// when a longer reported run contains it at every one of its occurrences.
pub fn repeated_sequences(
    elements: &[Element],
    min_length: usize,
    min_count: usize,
) -> Vec<RepeatedSequence> {
    use std::collections::BTreeMap;

    // Split into runs of certain elements, never crossing lines.
    let mut runs: Vec<Vec<&Element>> = Vec::new();
    let mut current: Vec<&Element> = Vec::new();
    let mut current_line: Option<&str> = None;
    for element in elements {
        if current_line != Some(element.line_id.as_str()) || element.is_uncertain() {
            if !current.is_empty() {
                runs.push(std::mem::take(&mut current));
            }
            current_line = Some(element.line_id.as_str());
            if element.is_uncertain() {
                continue;
            }
        }
        current.push(element);
    }
    if !current.is_empty() {
        runs.push(current);
    }

    let min_length = min_length.max(2);
    let min_count = min_count.max(2);
    let mut found: BTreeMap<Vec<String>, Vec<(String, usize, usize)>> = BTreeMap::new();
    for run in &runs {
        for length in min_length..=run.len() {
            for window in run.windows(length) {
                let key: Vec<String> = window.iter().map(|e| e.key()).collect();
                found.entry(key).or_default().push((
                    window[0].line_id.clone(),
                    window[0].position,
                    window[length - 1].position,
                ));
            }
        }
    }
    found.retain(|_, locations| locations.len() >= min_count);

    // Drop runs absorbed by a longer repeat at every occurrence.
    let keys: Vec<Vec<String>> = found.keys().cloned().collect();
    let absorbed: Vec<Vec<String>> = keys
        .iter()
        .filter(|short| {
            let short_locs = &found[*short];
            keys.iter().any(|long| {
                long.len() > short.len()
                    && short_locs.iter().all(|(line, start, end)| {
                        found[long]
                            .iter()
                            .any(|(l, s, e)| l == line && s <= start && e >= end)
                    })
            })
        })
        .cloned()
        .collect();
    for key in absorbed {
        found.remove(&key);
    }

    let mut result: Vec<RepeatedSequence> = found
        .into_iter()
        .map(|(key, locations)| RepeatedSequence {
            sequence: key.join("–"),
            length: key.len(),
            count: locations.len(),
            locations: locations
                .into_iter()
                .map(|(line, start, end)| format!("{line}.{start}-{end}"))
                .collect(),
        })
        .collect();
    result.sort_by(|a, b| {
        (b.length, b.count)
            .cmp(&(a.length, a.count))
            .then_with(|| a.sequence.cmp(&b.sequence))
    });
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_ligatures_alternatives_and_suffixes() {
        let elements = parse_line("Da2", "]•–60–78/79–590V.s–445.107.1–s594–[");
        let keys: Vec<String> = elements.iter().map(Element::key).collect();
        assert_eq!(keys, ["•", "60", "78/79", "590V.s", "445.107.1", "s594"]);
        assert!(elements[0].is_uncertain());
        assert!(!elements[1].is_uncertain());
        assert!(elements[2].is_uncertain(), "alternatives are not certain");
        assert_eq!(elements[3].parts[0].alternatives[0].base, Some(590));
        assert_eq!(
            elements[3].parts[1].alternatives[0].code.as_deref(),
            Some("s")
        );
        assert_eq!(elements[5].parts[0].alternatives[0].base, Some(594));
        assert_eq!(elements[5].position, 6);
    }

    #[test]
    fn keeps_annotations_out_of_parts() {
        let elements = parse_line("Da2", "]173(=5.225/325)–371/379(.61)–306.56/74(−)41[");
        assert_eq!(elements[0].key(), "173");
        assert_eq!(
            elements[2].key(),
            "306.56/74.41",
            "annotation must not glue signs"
        );
        assert_eq!(elements[0].annotations, ["=5.225/325"]);
        assert_eq!(elements[1].annotations, [".61"]);
    }

    #[test]
    fn flags_lacunae_uncertainty_and_irregular_separators() {
        let elements = parse_line("Da8", "]•–• […]–2••–3•6–4.94a/116--2–3../40.[");
        assert!(elements[1].lacuna);
        let sign = &elements[2].parts[0].alternatives[0];
        assert_eq!(
            (sign.base, sign.uncertain, sign.unparsed),
            (Some(2), true, false)
        );
        assert!(elements[3].parts[0].alternatives[0].unparsed);
        assert!(elements[4].irregular_separator && elements[5].irregular_separator);
        assert_eq!(elements[5].key(), "2");
        let empty_alternative = &elements[6].parts[1].alternatives[0];
        assert!(
            empty_alternative.unparsed,
            "'/40' has an empty first reading"
        );
    }

    #[test]
    fn frequency_separates_certain_from_possible() {
        let elements = parse_line("Da1", "]600–600–400/600–60•–522f.600[");
        let report = frequency(&elements, false);
        let six_hundred = report.rows.iter().find(|r| r.sign == "600").unwrap();
        assert_eq!((six_hundred.count, six_hundred.possible), (3, 1));
        let exact = frequency(&elements, true);
        assert!(exact.rows.iter().any(|r| r.sign == "522f"));
    }

    #[test]
    fn repeated_sequences_are_maximal_and_skip_uncertain_elements() {
        let mut elements = parse_line("L1", "]1–2–3–•–1–2–3[");
        elements.extend(parse_line("L2", "]1–2–3–9[ "));
        let repeats = repeated_sequences(&elements, 2, 2);
        assert_eq!(repeats.len(), 1, "{repeats:?}");
        assert_eq!(repeats[0].sequence, "1–2–3");
        assert_eq!(repeats[0].count, 3);
        assert_eq!(repeats[0].locations, ["L1.1-3", "L1.5-7", "L2.1-3"]);
    }

    #[test]
    fn repeats_do_not_cross_lines() {
        let mut elements = parse_line("L1", "]5–1[");
        elements.extend(parse_line("L2", "]2–5–1–2[ "));
        let repeats = repeated_sequences(&elements, 2, 2);
        assert_eq!(repeats.len(), 1);
        assert_eq!(repeats[0].sequence, "5–1");
    }
}
