//! Keeps the Markdown dossier and notebook in step with the CSV registers.
//!
//! The CSVs are the source of truth. Every registered id must appear in a
//! Markdown table in `research-dossier.md` or `decipherment-notebook.md`, every
//! id a table mentions must exist in the CSVs, and the confidence,
//! reliability, and sequence columns must agree where a table shows them.

use crate::audit::Workspace;
use crate::validate::ValidationReport;
use anyhow::{Context, Result};
use std::collections::{BTreeMap, HashMap};
use std::path::Path;

pub const SYNCED_DOCS: &[&str] = &["research-dossier.md", "decipherment-notebook.md"];

/// Columns compared between Markdown tables and CSV rows. Free-text columns
/// are deliberately not compared: prose may paraphrase the register.
const COMPARED_COLUMNS: &[&str] = &["confidence", "reliability", "sequence"];

const ID_PREFIXES: &[&str] = &["SRC", "C", "O", "S", "H", "R"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableRow {
    pub line: usize,
    pub id: String,
    /// Normalized header (lowercase, spaces to `_`) -> cell value.
    pub cells: BTreeMap<String, String>,
}

pub fn check_markdown_sync(
    report: &mut ValidationReport,
    root: &Path,
    ws: &Workspace,
) -> Result<()> {
    let registered = register_fields(ws);
    let mut mentioned: HashMap<String, Vec<String>> = HashMap::new();

    for doc in SYNCED_DOCS {
        let path = root.join(doc);
        let text = std::fs::read_to_string(&path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        for row in parse_id_rows(&text) {
            let location = format!("{doc}:{}", row.line);
            mentioned
                .entry(row.id.clone())
                .or_default()
                .push(location.clone());
            let Some(fields) = registered.get(row.id.as_str()) else {
                report.error(format!(
                    "{location} mentions {} which is not in any CSV register",
                    row.id
                ));
                continue;
            };
            for column in COMPARED_COLUMNS {
                if let (Some(shown), Some(actual)) = (row.cells.get(*column), fields.get(*column)) {
                    if shown != actual {
                        report.error(format!(
                            "{location} shows {} {column}={shown} but the CSV has {column}={actual}",
                            row.id
                        ));
                    }
                }
            }
        }
    }

    let mut missing: Vec<&str> = registered
        .keys()
        .copied()
        .filter(|id| !mentioned.contains_key(*id))
        .collect();
    missing.sort_unstable();
    for id in missing {
        report.error(format!(
            "{id} is in the CSV registers but not in any table in {}",
            SYNCED_DOCS.join(" or ")
        ));
    }
    Ok(())
}

fn register_fields(ws: &Workspace) -> HashMap<&str, HashMap<&'static str, &str>> {
    let mut map: HashMap<&str, HashMap<&'static str, &str>> = HashMap::new();
    for s in &ws.sources {
        map.insert(
            &s.source_id,
            HashMap::from([("reliability", s.reliability.as_str())]),
        );
    }
    for c in &ws.claims {
        map.insert(
            &c.claim_id,
            HashMap::from([("confidence", c.confidence.as_str())]),
        );
    }
    for o in &ws.observations {
        map.insert(
            &o.observation_id,
            HashMap::from([("confidence", o.confidence.as_str())]),
        );
    }
    for h in &ws.hypotheses {
        map.insert(
            &h.hypothesis_id,
            HashMap::from([("confidence", h.confidence.as_str())]),
        );
    }
    for s in &ws.sequences {
        map.insert(
            &s.sequence_id,
            HashMap::from([
                ("confidence", s.confidence.as_str()),
                ("sequence", s.sequence.as_str()),
            ]),
        );
    }
    for r in &ws.readings {
        map.insert(
            &r.reading_id,
            HashMap::from([("confidence", r.confidence.as_str())]),
        );
    }
    map
}

/// Parse every Markdown table and return rows whose first cell starts with a
/// register id such as `C-001` or `SRC-003: Barthel ...`.
pub fn parse_id_rows(text: &str) -> Vec<TableRow> {
    let lines: Vec<&str> = text.lines().collect();
    let mut rows = Vec::new();
    let mut index = 0;
    while index + 1 < lines.len() {
        if is_table_line(lines[index]) && is_separator(lines[index + 1]) {
            let headers: Vec<String> = split_cells(lines[index])
                .into_iter()
                .map(|header| normalize_header(&header))
                .collect();
            index += 2;
            while index < lines.len() && is_table_line(lines[index]) {
                let cells = split_cells(lines[index]);
                if let Some(id) = cells.first().and_then(|cell| leading_id(cell)) {
                    rows.push(TableRow {
                        line: index + 1,
                        id,
                        cells: headers.iter().cloned().zip(cells).collect(),
                    });
                }
                index += 1;
            }
        } else {
            index += 1;
        }
    }
    rows
}

fn is_table_line(line: &str) -> bool {
    line.trim_start().starts_with('|')
}

fn is_separator(line: &str) -> bool {
    is_table_line(line)
        && split_cells(line)
            .iter()
            .all(|cell| !cell.is_empty() && cell.chars().all(|c| matches!(c, '-' | ':' | ' ')))
}

/// Split a table row into cells. As in GitHub-flavoured Markdown, `\|`
/// is a literal pipe inside a cell, not a cell boundary.
fn split_cells(line: &str) -> Vec<String> {
    let trimmed = line.trim();
    let inner = trimmed.strip_prefix('|').unwrap_or(trimmed);
    let inner = if inner.ends_with('|') && !inner.ends_with("\\|") {
        &inner[..inner.len() - 1]
    } else {
        inner
    };
    let mut cells = Vec::new();
    let mut current = String::new();
    let mut chars = inner.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\\' if chars.peek() == Some(&'|') => {
                current.push('|');
                chars.next();
            }
            '|' => cells.push(std::mem::take(&mut current)),
            _ => current.push(c),
        }
    }
    cells.push(current);
    cells
        .into_iter()
        .map(|cell| cell.trim().to_string())
        .collect()
}

fn normalize_header(header: &str) -> String {
    header.trim().to_lowercase().replace(' ', "_")
}

fn leading_id(cell: &str) -> Option<String> {
    let token: String = cell
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '-')
        .collect();
    let (prefix, digits) = token.split_once('-')?;
    (ID_PREFIXES.contains(&prefix)
        && !digits.is_empty()
        && digits.chars().all(|c| c.is_ascii_digit()))
    .then_some(token)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_id_rows_and_normalizes_headers() {
        let text = "\
intro
| Source | Reliability |
|---|---|
| SRC-001: Barthel, Grundlagen | High |
| not an id | Low |

| Sequence ID | Sequence | Confidence |
|---|---:|---|
| S-001 | none-recorded | Low |
";
        let rows = parse_id_rows(text);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].id, "SRC-001");
        assert_eq!(rows[0].line, 4);
        assert_eq!(rows[0].cells["reliability"], "High");
        assert_eq!(rows[1].id, "S-001");
        assert_eq!(rows[1].cells["sequence"], "none-recorded");
        assert_eq!(rows[1].cells["sequence_id"], "S-001");
    }

    #[test]
    fn ignores_non_register_tokens() {
        assert_eq!(leading_id("none"), None);
        assert_eq!(leading_id("522"), None);
        assert_eq!(leading_id("X-001"), None);
        assert_eq!(leading_id("C-"), None);
        assert_eq!(leading_id("H-002"), Some("H-002".to_string()));
    }

    #[test]
    fn escaped_pipes_stay_inside_a_cell() {
        let text = "| Sequence ID | Sequence | Confidence |\n|---|---|---|\n| S-004 | 600\\|610 * 200.52x.* | High |\n";
        let rows = parse_id_rows(text);
        assert_eq!(rows[0].cells["sequence"], "600|610 * 200.52x.*");
        assert_eq!(rows[0].cells["confidence"], "High");
    }

    #[test]
    fn requires_a_separator_row_to_start_a_table() {
        let text = "| C-001 | High |\n| C-002 | Low |\n";
        assert!(parse_id_rows(text).is_empty());
    }
}
