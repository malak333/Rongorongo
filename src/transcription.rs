//! Parsing and structural analysis of verbatim transcription lines.
//!
//! Each line is stored twice in `data/transcriptions/lines.csv`:
//! `raw_text` is the printed text, and `marked_text` is the same text with
//! the source's underlining (doubtful readings) wrapped in `{` `}`. Removing
//! the braces from `marked_text` must give `raw_text` exactly; the audit
//! checks this. Everything here is derived on demand, so a parser change
//! never rewrites the source data.
//!
//! Notation (Barthel-based, as used by Lastilla et al. 2022 for Text D):
//!
//! - `–` (en dash) separates graphic elements. `--` is treated the same way
//!   and flagged as irregular.
//! - `(−)` separates two elements that may have been written jointly. The
//!   source does not define it, but the authors' element numbering (e.g.
//!   Da1.7, Da1.18–19) and the numbered figures only match if it is a break.
//! - `.` joins two shapes horizontally and `:` joins them vertically
//!   (ligatures); each joined shape is a *part*.
//! - `/` separates equally possible transcriptions of one part.
//! - `•` marks uncertainty about a glyph or part of a glyph.
//! - Underlining (`{...}` in `marked_text`) marks a doubtful reading. It can
//!   cover a whole sign, single digits of a sign, or a separator; an
//!   underlined separator marks a doubtful boundary or join.
//! - Letter suffixes (`a`–`e`, `f`, `s`, `x`, `y`, `V`) are palaeographic
//!   variants; a leading `s` or a bare `s` part is Barthel's ornament element.
//! - Other `(...)` content (`(=...)`, `(.61)`, `(6x.)`) is kept as an
//!   annotation, excluded from counts, and treated as a part boundary.
//! - `[...]` or `[…]` is recorded as a lacuna. Leading `]` and trailing `[` are
//!   stripped; the source does not define them.

use serde::Serialize;

#[derive(Debug, Clone, serde::Deserialize, Serialize)]
pub struct TranscriptionLine {
    pub line_id: String,
    pub corpus_id: String,
    pub side: String,
    pub source_ref: String,
    pub license: String,
    pub raw_text: String,
    pub marked_text: String,
    pub notes: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Element {
    pub line_id: String,
    /// 1-based position within the line, as counted by this parser.
    pub position: usize,
    /// Element text as printed, without underline marks.
    pub raw: String,
    /// Element text with underlined (doubtful) spans wrapped in `{}`.
    pub marked: String,
    pub parts: Vec<Part>,
    pub annotations: Vec<String>,
    pub lacuna: bool,
    /// True when a ligature join inside the element is underlined.
    pub doubtful_join: bool,
    /// True when the separator before this element is underlined.
    pub boundary_doubtful: bool,
    /// True when this element follows a `(−)` possible-join separator.
    pub boundary_possible_join: bool,
    /// True when this element follows the irregular `--` separator.
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
    /// Marked `•`: the glyph or part of it cannot be identified.
    pub uncertain: bool,
    /// Underlined in the source (wholly or in part): a doubtful reading.
    pub doubtful: bool,
    /// True when the sign does not match the notation grammar above.
    pub unparsed: bool,
}

impl Element {
    /// True when the element cannot be read with confidence: a lacuna, an
    /// unparsed or `•`-uncertain sign, `/` alternatives, or any underlined
    /// sign or join. Such elements are excluded from sequence matching so
    /// that uncertainty cannot manufacture a repeat.
    pub fn is_uncertain(&self) -> bool {
        self.lacuna
            || self.doubtful_join
            || self.parts.is_empty()
            || self.parts.iter().any(|part| {
                part.alternatives.len() != 1
                    || part
                        .alternatives
                        .iter()
                        .any(|s| s.uncertain || s.unparsed || s.doubtful)
            })
    }

    /// True when the boundary before this element is not a clean separator.
    pub fn has_soft_boundary(&self) -> bool {
        self.boundary_doubtful || self.boundary_possible_join || self.irregular_separator
    }

    /// Normalized element text used to compare elements: annotations,
    /// whitespace, underline marks, and lacuna marks removed.
    pub fn key(&self) -> String {
        self.parts
            .iter()
            .map(|part| part.raw.as_str())
            .collect::<Vec<_>>()
            .join(".")
    }
}

/// A character of marked text and whether it is underlined.
type Marked = (char, bool);

/// Remove `{}` underline marks, keeping a per-character doubtful flag.
pub fn unmark(text: &str) -> Vec<Marked> {
    let mut depth = 0usize;
    let mut out = Vec::new();
    for c in text.chars() {
        match c {
            '{' => depth += 1,
            '}' => depth = depth.saturating_sub(1),
            _ => out.push((c, depth > 0)),
        }
    }
    out
}

/// Check that `{}` marks are balanced and not nested.
pub fn marks_are_balanced(text: &str) -> bool {
    let mut open = false;
    for c in text.chars() {
        match c {
            '{' if open => return false,
            '{' => open = true,
            '}' if !open => return false,
            '}' => open = false,
            _ => {}
        }
    }
    !open
}

fn plain(chars: &[Marked]) -> String {
    chars.iter().map(|(c, _)| *c).collect()
}

fn remark(chars: &[Marked]) -> String {
    let mut out = String::new();
    let mut open = false;
    for (c, doubtful) in chars {
        if *doubtful != open {
            out.push(if *doubtful { '{' } else { '}' });
            open = *doubtful;
        }
        out.push(*c);
    }
    if open {
        out.push('}');
    }
    out
}

#[derive(Default, Clone, Copy)]
struct Boundary {
    doubtful: bool,
    possible_join: bool,
    irregular: bool,
}

/// Parse a line. `text` may be plain or carry `{}` underline marks.
pub fn parse_line(line_id: &str, text: &str) -> Vec<Element> {
    let mut chars = unmark(text);
    while chars.first().is_some_and(|(c, _)| c.is_whitespace()) {
        chars.remove(0);
    }
    while chars.last().is_some_and(|(c, _)| c.is_whitespace()) {
        chars.pop();
    }
    if chars.first().is_some_and(|(c, _)| *c == ']') {
        chars.remove(0);
    }
    if chars.last().is_some_and(|(c, _)| *c == '[') {
        chars.pop();
    }

    let mut elements = Vec::new();
    let mut current: Vec<Marked> = Vec::new();
    let mut boundary = Boundary::default();
    let flush = |current: &mut Vec<Marked>, boundary: Boundary, elements: &mut Vec<Element>| {
        if current.iter().any(|(c, _)| !c.is_whitespace()) {
            let position = elements.len() + 1;
            elements.push(parse_element(line_id, position, current, boundary));
        }
        current.clear();
    };

    let mut i = 0;
    while i < chars.len() {
        let c = chars[i].0;
        let next = |k: usize| chars.get(i + k).map(|(c, _)| *c);
        if c == '(' && next(1) == Some('−') && next(2) == Some(')') {
            flush(&mut current, boundary, &mut elements);
            boundary = Boundary {
                possible_join: true,
                doubtful: chars[i..i + 3].iter().any(|(_, d)| *d),
                ..Boundary::default()
            };
            i += 3;
        } else if c == '–' {
            flush(&mut current, boundary, &mut elements);
            boundary = Boundary {
                doubtful: chars[i].1,
                ..Boundary::default()
            };
            i += 1;
        } else if c == '-' && next(1) == Some('-') {
            flush(&mut current, boundary, &mut elements);
            boundary = Boundary {
                irregular: true,
                doubtful: chars[i].1 || chars[i + 1].1,
                ..Boundary::default()
            };
            i += 2;
        } else {
            current.push(chars[i]);
            i += 1;
        }
    }
    flush(&mut current, boundary, &mut elements);
    elements
}

fn remove_lacunae(chars: &[Marked]) -> (Vec<Marked>, bool) {
    let text: Vec<char> = chars.iter().map(|(c, _)| *c).collect();
    let mut out = Vec::new();
    let mut lacuna = false;
    let mut i = 0;
    while i < chars.len() {
        let rest = &text[i..];
        let len = if rest.starts_with(&['[', '…', ']']) {
            3
        } else if rest.starts_with(&['[', '.', '.', '.', ']']) {
            5
        } else {
            0
        };
        if len > 0 {
            lacuna = true;
            i += len;
        } else {
            out.push(chars[i]);
            i += 1;
        }
    }
    (out, lacuna)
}

fn parse_element(line_id: &str, position: usize, chars: &[Marked], boundary: Boundary) -> Element {
    let trimmed: Vec<Marked> = {
        let start = chars
            .iter()
            .position(|(c, _)| !c.is_whitespace())
            .unwrap_or(0);
        let end = chars
            .iter()
            .rposition(|(c, _)| !c.is_whitespace())
            .map_or(0, |e| e + 1);
        chars[start..end].to_vec()
    };
    let (without_lacuna, lacuna) = remove_lacunae(&trimmed);

    let mut annotations = Vec::new();
    let mut text: Vec<Marked> = Vec::new();
    let mut depth = 0usize;
    let mut current = String::new();
    for &(c, doubtful) in &without_lacuna {
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
                    text.push(('.', false));
                    continue;
                }
            }
            _ => {}
        }
        if depth > 0 {
            current.push(c);
        } else if !c.is_whitespace() {
            text.push((c, doubtful));
        }
    }

    let mut parts = Vec::new();
    let mut doubtful_join = false;
    for chunk in text.split(|(c, _)| *c == '.' || *c == ':') {
        if chunk.is_empty() {
            continue;
        }
        let alternatives = chunk
            .split(|(c, _)| *c == '/')
            .map(|alt| {
                let mut sign = parse_sign(&plain(alt));
                sign.doubtful = alt.iter().any(|(_, d)| *d);
                sign
            })
            .collect();
        parts.push(Part {
            raw: plain(chunk),
            alternatives,
        });
    }
    // A join is doubtful when the `.` or `:` itself is underlined.
    for (index, (c, doubtful)) in text.iter().enumerate() {
        let interior = index > 0 && index + 1 < text.len();
        if (*c == '.' || *c == ':') && *doubtful && interior {
            doubtful_join = true;
        }
    }

    Element {
        line_id: line_id.to_string(),
        position,
        raw: plain(&trimmed),
        marked: remark(&trimmed),
        parts,
        annotations,
        lacuna,
        doubtful_join,
        boundary_doubtful: boundary.doubtful,
        boundary_possible_join: boundary.possible_join,
        irregular_separator: boundary.irregular,
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
            doubtful: false,
            unparsed: false,
        };
    }
    if core == "s" {
        return Sign {
            raw: raw.to_string(),
            base: None,
            code: Some("s".to_string()),
            uncertain,
            doubtful: false,
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
            doubtful: false,
            unparsed: false,
        }
    } else {
        Sign {
            raw: raw.to_string(),
            base: None,
            code: None,
            uncertain,
            doubtful: false,
            unparsed: true,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct FrequencyRow {
    pub sign: String,
    /// Occurrences as an unambiguous, certain reading.
    pub count: usize,
    /// Occurrences marked `•` (uncertain), underlined (doubtful), or offered
    /// as one of several `/` alternatives.
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
                if ambiguous || sign.uncertain || sign.doubtful {
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
    /// Element level: `line.start-end` (1-based element positions).
    /// Part level: `line.element.part-element.part`.
    pub locations: Vec<String>,
    /// Part level only: how the signs were written at each occurrence, with
    /// `.` for a ligature join and `–` for separate elements, e.g. `200.52x`.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub forms: Vec<String>,
}

/// Unit of comparison for repeat detection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    /// Whole graphic elements (ligatures compared as wholes).
    Element,
    /// Individual signs (parts), ignoring whether they are ligatured.
    Part,
}

struct Token<'a> {
    line_id: &'a str,
    label: String,
    key: String,
    /// False for anything that must not take part in a repeat.
    usable: bool,
    /// True when a run must not continue across the gap before this token.
    break_before: bool,
    /// `–` between elements, `.` inside a ligature.
    joiner_before: char,
}

fn element_tokens(elements: &[Element]) -> Vec<Token<'_>> {
    elements
        .iter()
        .map(|e| Token {
            line_id: &e.line_id,
            label: e.position.to_string(),
            key: e.key(),
            usable: !e.is_uncertain(),
            break_before: e.has_soft_boundary(),
            joiner_before: '–',
        })
        .collect()
}

fn part_tokens(elements: &[Element]) -> Vec<Token<'_>> {
    let mut tokens = Vec::new();
    for e in elements {
        if e.lacuna || e.parts.is_empty() {
            // Keep a placeholder so a run cannot bridge the gap.
            tokens.push(Token {
                line_id: &e.line_id,
                label: format!("{}.0", e.position),
                key: String::new(),
                usable: false,
                break_before: true,
                joiner_before: '–',
            });
            continue;
        }
        for (index, part) in e.parts.iter().enumerate() {
            let certain = part.alternatives.len() == 1
                && part
                    .alternatives
                    .iter()
                    .all(|s| !s.uncertain && !s.doubtful && !s.unparsed);
            tokens.push(Token {
                line_id: &e.line_id,
                label: format!("{}.{}", e.position, index + 1),
                key: part.raw.clone(),
                usable: certain,
                break_before: false,
                joiner_before: if index == 0 { '–' } else { '.' },
            });
        }
    }
    tokens
}

/// Find maximal repeated runs within lines, at element or part level.
///
/// Element level: runs never cross a line, an uncertain or doubtful element,
/// or a soft boundary (underlined separator, `(−)`, or `--`).
/// Part level: runs follow the sign stream and ignore whether neighbouring
/// signs are ligatured, since the source says that is often unclear; they
/// still never cross a line, a lacuna, or an uncertain, doubtful,
/// ambiguous, or unparsed sign.
///
/// A run is dropped when a longer reported run contains it at every one of
/// its occurrences.
pub fn repeated_sequences(
    elements: &[Element],
    level: Level,
    min_length: usize,
    min_count: usize,
) -> Vec<RepeatedSequence> {
    use std::collections::BTreeMap;

    let tokens = match level {
        Level::Element => element_tokens(elements),
        Level::Part => part_tokens(elements),
    };

    // Split token indices into runs of usable tokens.
    let mut runs: Vec<Vec<usize>> = Vec::new();
    let mut current: Vec<usize> = Vec::new();
    let mut current_line: Option<&str> = None;
    for (index, token) in tokens.iter().enumerate() {
        if current_line != Some(token.line_id) || !token.usable || token.break_before {
            if !current.is_empty() {
                runs.push(std::mem::take(&mut current));
            }
            current_line = Some(token.line_id);
        }
        if token.usable {
            current.push(index);
        }
    }
    if !current.is_empty() {
        runs.push(current);
    }

    let min_length = min_length.max(2);
    let min_count = min_count.max(2);
    let mut found: BTreeMap<Vec<String>, Vec<(usize, usize)>> = BTreeMap::new();
    for run in &runs {
        for length in min_length..=run.len() {
            for window in run.windows(length) {
                let key: Vec<String> = window.iter().map(|&i| tokens[i].key.clone()).collect();
                found
                    .entry(key)
                    .or_default()
                    .push((window[0], window[length - 1]));
            }
        }
    }
    found.retain(|_, occurrences| occurrences.len() >= min_count);

    // Drop runs absorbed by a longer repeat at every occurrence. Runs never
    // cross lines, so containment of token index ranges implies same line.
    let keys: Vec<Vec<String>> = found.keys().cloned().collect();
    let absorbed: Vec<Vec<String>> = keys
        .iter()
        .filter(|short| {
            keys.iter().any(|long| {
                long.len() > short.len()
                    && found[*short].iter().all(|&(start, end)| {
                        found[long].iter().any(|&(s, e)| s <= start && e >= end)
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
        .map(|(key, occurrences)| {
            let locations = occurrences
                .iter()
                .map(|&(start, end)| {
                    let line = tokens[start].line_id;
                    format!("{line}.{}-{}", tokens[start].label, tokens[end].label)
                })
                .collect();
            let forms = match level {
                Level::Element => Vec::new(),
                Level::Part => {
                    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
                    for &(start, end) in &occurrences {
                        let mut form = tokens[start].key.clone();
                        for token in &tokens[start + 1..=end] {
                            form.push(token.joiner_before);
                            form.push_str(&token.key);
                        }
                        *counts.entry(form).or_default() += 1;
                    }
                    counts
                        .into_iter()
                        .map(|(form, n)| format!("{form} ×{n}"))
                        .collect()
                }
            };
            RepeatedSequence {
                sequence: key.join(match level {
                    Level::Element => "–",
                    Level::Part => " ",
                }),
                length: key.len(),
                count: occurrences.len(),
                locations,
                forms,
            }
        })
        .collect();
    result.sort_by(|a, b| {
        (b.length, b.count)
            .cmp(&(a.length, a.count))
            .then_with(|| a.sequence.cmp(&b.sequence))
    });
    result
}

#[derive(Debug, Clone, Serialize)]
pub struct PatternMatch {
    pub line_id: String,
    pub start: usize,
    pub end: usize,
    /// Matched elements with underlined spans in `{}`.
    pub text: String,
    /// False when any matched element is doubtful, uncertain, ambiguous, or
    /// unparsed, or a soft boundary falls inside the match.
    pub certain: bool,
}

/// Search for a sequence of elements within lines.
///
/// The pattern is a list of element patterns separated by spaces, `–`, or
/// `-`. Each element pattern is one or more alternatives joined by `|`.
/// An alternative is matched against the element's normalized key (parts
/// joined by `.`; `:` ligatures also become `.`), where `*` matches any run of
/// characters. A lone `*` therefore matches any one element.
///
/// Example: `600|610 * 200.52x.*`
pub fn find_pattern(elements: &[Element], pattern: &str) -> Result<Vec<PatternMatch>, String> {
    let slots: Vec<Vec<String>> = pattern
        .split(|c: char| c.is_whitespace() || c == '–' || c == '-')
        .filter(|slot| !slot.is_empty())
        .map(|slot| slot.split('|').map(str::to_string).collect())
        .collect();
    if slots.is_empty() {
        return Err("pattern is empty".to_string());
    }
    if slots.iter().flatten().any(|alt| alt.is_empty()) {
        return Err(format!("pattern has an empty alternative: {pattern}"));
    }

    let mut matches = Vec::new();
    let mut line_start = 0;
    while line_start < elements.len() {
        let line_id = &elements[line_start].line_id;
        let line_end = elements[line_start..]
            .iter()
            .position(|e| &e.line_id != line_id)
            .map_or(elements.len(), |offset| line_start + offset);
        let line = &elements[line_start..line_end];
        for window in line.windows(slots.len()) {
            let hit = window
                .iter()
                .zip(&slots)
                .all(|(e, alts)| alts.iter().any(|alt| glob(alt, &e.key())));
            if hit {
                let certain = window.iter().all(|e| !e.is_uncertain())
                    && window.iter().skip(1).all(|e| !e.has_soft_boundary());
                matches.push(PatternMatch {
                    line_id: line_id.clone(),
                    start: window[0].position,
                    end: window[window.len() - 1].position,
                    text: window
                        .iter()
                        .map(|e| e.marked.as_str())
                        .collect::<Vec<_>>()
                        .join("–"),
                    certain,
                });
            }
        }
        line_start = line_end;
    }
    Ok(matches)
}

/// Glob match where `*` matches any run of characters (including none).
fn glob(pattern: &str, text: &str) -> bool {
    let p: Vec<char> = pattern.chars().collect();
    let t: Vec<char> = text.chars().collect();
    let (mut pi, mut ti) = (0, 0);
    let (mut star, mut mark) = (None, 0);
    while ti < t.len() {
        if pi < p.len() && p[pi] != '*' && p[pi] == t[ti] {
            pi += 1;
            ti += 1;
        } else if pi < p.len() && p[pi] == '*' {
            star = Some(pi);
            mark = ti;
            pi += 1;
        } else if let Some(s) = star {
            pi = s + 1;
            mark += 1;
            ti = mark;
        } else {
            return false;
        }
    }
    while pi < p.len() && p[pi] == '*' {
        pi += 1;
    }
    pi == p.len()
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
        let elements = parse_line("Da2", "]173(=5.225/325)–371/379(.61)–(6x.)206s[");
        assert_eq!(elements[0].key(), "173");
        assert_eq!(elements[0].annotations, ["=5.225/325"]);
        assert_eq!(elements[1].annotations, [".61"]);
        assert_eq!(elements[2].key(), "206s");
    }

    #[test]
    fn possible_join_separator_splits_elements() {
        // Matches the authors' numbering: 381/741 is Da1.7 and 99 is Da1.19.
        let da1 = "]•–•–•–•–2•1(−)6–381/741–700–208–7y–7–94–207–63.207s–734–204–5f–522f–99[";
        let elements = parse_line("Da1", da1);
        assert_eq!(elements.len(), 19);
        assert_eq!(elements[6].key(), "381/741");
        assert_eq!(elements[18].key(), "99");
        assert!(elements[5].boundary_possible_join);
        assert_eq!(elements[5].key(), "6");

        let db1 = parse_line("Db1", "]306.56/74(−)41–3•2[");
        assert_eq!(db1[0].key(), "306.56/74");
        assert_eq!(db1[1].key(), "41");
    }

    #[test]
    fn flags_lacunae_uncertainty_and_irregular_separators() {
        let elements = parse_line("Da8", "]•–• [...]–2••–3•6–4.94a/116--2–• […]–3••/40•[");
        assert!(elements[1].lacuna);
        assert!(elements[6].lacuna, "unicode ellipsis form");
        let sign = &elements[2].parts[0].alternatives[0];
        assert_eq!(
            (sign.base, sign.uncertain, sign.unparsed),
            (Some(2), true, false)
        );
        assert!(elements[3].parts[0].alternatives[0].unparsed);
        assert!(!elements[4].irregular_separator);
        assert!(elements[5].irregular_separator, "flag sits after the `--`");
        assert_eq!(elements[5].key(), "2");
        let alternatives = &elements[7].parts[0].alternatives;
        assert_eq!(alternatives.len(), 2);
        assert!(alternatives.iter().all(|s| s.uncertain && !s.unparsed));
    }

    #[test]
    fn underline_marks_doubtful_signs_joins_and_boundaries() {
        let elements = parse_line("Da6", "]{59}–2{0}7–2V{.}3–7{00–}•–{3.3}.52–600[");
        assert_eq!(elements[0].raw, "59");
        assert_eq!(elements[0].marked, "{59}");
        assert!(elements[0].parts[0].alternatives[0].doubtful);
        assert!(
            elements[1].parts[0].alternatives[0].doubtful,
            "one doubtful digit"
        );
        assert_eq!(elements[1].key(), "207");
        assert!(elements[2].doubtful_join);
        assert!(!elements[2].parts[0].alternatives[0].doubtful);
        assert!(elements[3].parts[0].alternatives[0].doubtful);
        assert!(elements[4].boundary_doubtful, "underlined dash before •");
        assert!(elements[5].doubtful_join);
        assert!(!elements[5].parts[2].alternatives[0].doubtful);
        assert!(!elements[6].is_uncertain());
        assert!(elements.iter().take(6).all(Element::is_uncertain));
    }

    #[test]
    fn marks_must_be_balanced() {
        assert!(marks_are_balanced("]{59}–2{0}7["));
        assert!(!marks_are_balanced("]{59–2"));
        assert!(!marks_are_balanced("]59}–2"));
        assert!(!marks_are_balanced("]{{59}}"));
        assert_eq!(plain(&unmark("]{59}–7{00–}•[")), "]59–700–•[");
    }

    #[test]
    fn glob_matches_character_runs() {
        assert!(glob("*", "200.52x.s"));
        assert!(glob("200.52x.*", "200.52x.69"));
        assert!(!glob("200.52x.*", "200.52x"), "needs a part after the dot");
        assert!(glob("200.52*", "200.524f.78"), "`*` is character-level");
        assert!(glob("6*", "600"));
        assert!(!glob("600", "6000"));
    }

    #[test]
    fn find_pattern_matches_template_with_alternatives_and_wildcards() {
        let mut elements = parse_line(
            "Da3",
            "]600–42:200.41/11–610–2–200.52x.s–600–2–200.52x.52–600–460–200.52x.69[",
        );
        elements.extend(parse_line("Db1", "]600–1–200.52x.8[ "));
        let matches = find_pattern(&elements, "600|610 * 200.52x.*").unwrap();
        let found: Vec<(String, usize, usize)> = matches
            .iter()
            .map(|m| (m.line_id.clone(), m.start, m.end))
            .collect();
        assert_eq!(
            found,
            [
                ("Da3".to_string(), 3, 5),
                ("Da3".to_string(), 6, 8),
                ("Da3".to_string(), 9, 11),
                ("Db1".to_string(), 1, 3),
            ]
        );
        assert!(matches.iter().all(|m| m.certain));
        let ligature = find_pattern(&elements, "42.200.*").unwrap();
        assert_eq!(ligature.len(), 1, "`:` ligatures normalize to `.`");
        assert!(!ligature[0].certain, "41/11 is ambiguous");
        assert!(find_pattern(&elements, "  ").is_err());
        assert!(find_pattern(&elements, "600||610").is_err());
    }

    #[test]
    fn find_pattern_never_crosses_lines() {
        let mut elements = parse_line("L1", "]5–1[");
        elements.extend(parse_line("L2", "]2–9[ "));
        assert!(find_pattern(&elements, "1 2").unwrap().is_empty());
    }

    #[test]
    fn part_level_repeats_ignore_ligature_status_and_report_forms() {
        let elements = parse_line("L1", "]2–200.52x.s–600–2–200.52x.69–{200}.52x–7–200–52x[");
        let repeats = repeated_sequences(&elements, Level::Part, 2, 2);
        let core = repeats.iter().find(|r| r.sequence == "200 52x").unwrap();
        assert_eq!(core.count, 3, "{repeats:?}");
        assert_eq!(core.forms, ["200.52x ×2", "200–52x ×1"]);
        assert_eq!(core.locations[2], "L1.8.1-9.1");
        let longer = repeats.iter().find(|r| r.sequence == "2 200 52x").unwrap();
        assert_eq!(longer.count, 2);
        // The doubtful {200} never takes part.
        assert!(repeats
            .iter()
            .all(|r| r.locations.iter().all(|l| !l.contains(".6."))));
    }

    #[test]
    fn part_level_runs_do_not_bridge_lacunae() {
        let elements = parse_line("L1", "]1–2–• [...]–1–2–1–• […]–2[");
        let repeats = repeated_sequences(&elements, Level::Part, 2, 2);
        assert_eq!(repeats.len(), 1, "{repeats:?}");
        assert_eq!(repeats[0].sequence, "1 2");
        assert_eq!(repeats[0].count, 2);
    }

    #[test]
    fn soft_boundaries_break_repeats() {
        let elements = parse_line("L1", "]1–2–5–1{–}2–6–1(−)2[");
        let repeats = repeated_sequences(&elements, Level::Element, 2, 2);
        assert!(repeats.is_empty(), "{repeats:?}");
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
        let repeats = repeated_sequences(&elements, Level::Element, 2, 2);
        assert_eq!(repeats.len(), 1, "{repeats:?}");
        assert_eq!(repeats[0].sequence, "1–2–3");
        assert_eq!(repeats[0].count, 3);
        assert_eq!(repeats[0].locations, ["L1.1-3", "L1.5-7", "L2.1-3"]);
    }

    #[test]
    fn repeats_do_not_cross_lines() {
        let mut elements = parse_line("L1", "]5–1[");
        elements.extend(parse_line("L2", "]2–5–1–2[ "));
        let repeats = repeated_sequences(&elements, Level::Element, 2, 2);
        assert_eq!(repeats.len(), 1);
        assert_eq!(repeats[0].sequence, "5–1");
    }
}
