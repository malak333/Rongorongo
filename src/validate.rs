use std::fmt;
use std::path::Path;

use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct ValidationReport {
    pub checked_files: usize,
    pub warning_count: usize,
    pub error_count: usize,
    /// False when any required file is missing or any CSV has missing headers.
    /// Cross-reference auditing is skipped when the schema is not valid.
    pub schema_valid: bool,
    pub messages: Vec<ValidationMessage>,
}

#[derive(Debug, Clone)]
pub struct ValidationMessage {
    pub level: ValidationLevel,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationLevel {
    Ok,
    Warning,
    Error,
}

impl fmt::Display for ValidationLevel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Ok => "ok",
            Self::Warning => "warning",
            Self::Error => "error",
        };
        formatter.write_str(value)
    }
}

impl ValidationReport {
    pub fn error(&mut self, message: String) {
        self.push(ValidationLevel::Error, message);
    }

    pub fn push(&mut self, level: ValidationLevel, message: String) {
        match level {
            ValidationLevel::Ok => {}
            ValidationLevel::Warning => self.warning_count += 1,
            ValidationLevel::Error => self.error_count += 1,
        }
        self.messages.push(ValidationMessage { level, message });
    }
}

/// Confidence labels allowed on claims, observations, sequences, hypotheses,
/// readings, and corpus inclusion.
pub const CONFIDENCE_LABELS: &[&str] = &["High", "Medium", "Low"];

/// Reliability labels allowed on sources. `Mixed` marks a source with
/// separable strong and weak parts (see docs/source-intake.md).
pub const RELIABILITY_LABELS: &[&str] = &["High", "Medium", "Low", "Mixed"];

pub const CLAIM_STATUSES: &[&str] = &["draft", "active", "promoted", "rejected", "superseded"];
pub const HYPOTHESIS_STATUSES: &[&str] = &[
    "draft",
    "active",
    "accepted-working-rule",
    "rejected",
    "superseded",
];
pub const RECORD_STATUSES: &[&str] = &["draft", "active", "promoted", "rejected", "superseded"];

struct CsvSpec {
    path: &'static str,
    headers: &'static [&'static str],
    /// Columns that must be non-empty.
    required_columns: &'static [&'static str],
    /// Primary-key column; values must be unique within the file.
    id_column: &'static str,
    confidence_columns: &'static [&'static str],
    reliability_columns: &'static [&'static str],
    statuses: Option<&'static [&'static str]>,
}

const CSV_SPECS: &[CsvSpec] = &[
    CsvSpec {
        path: "data/corpus-index.csv",
        headers: &[
            "object_name",
            "catalog_id",
            "current_location",
            "sides",
            "transcription_source",
            "source_reliability",
            "inclusion_confidence",
            "reference_url",
            "notes",
        ],
        required_columns: &["object_name", "catalog_id", "transcription_source"],
        id_column: "catalog_id",
        confidence_columns: &["inclusion_confidence"],
        reliability_columns: &["source_reliability"],
        statuses: None,
    },
    CsvSpec {
        path: "data/source-registry.csv",
        headers: &[
            "source_id",
            "citation",
            "year",
            "source_type",
            "contribution",
            "limits",
            "reliability",
            "access",
            "url",
            "accessed",
            "notes",
        ],
        required_columns: &["source_id", "citation", "contribution", "limits", "access"],
        id_column: "source_id",
        confidence_columns: &[],
        reliability_columns: &["reliability"],
        statuses: None,
    },
    CsvSpec {
        path: "data/hypotheses.csv",
        headers: &[
            "hypothesis_id",
            "claim",
            "evidence",
            "evidence_refs",
            "corpus_refs",
            "observation_refs",
            "test",
            "status",
            "confidence",
            "notes",
        ],
        required_columns: &["hypothesis_id", "claim", "test"],
        id_column: "hypothesis_id",
        confidence_columns: &["confidence"],
        reliability_columns: &[],
        statuses: Some(HYPOTHESIS_STATUSES),
    },
    CsvSpec {
        path: "data/claims.csv",
        headers: &[
            "claim_id",
            "claim",
            "claim_type",
            "evidence_refs",
            "corpus_refs",
            "confidence",
            "status",
            "notes",
        ],
        required_columns: &["claim_id", "claim"],
        id_column: "claim_id",
        confidence_columns: &["confidence"],
        reliability_columns: &[],
        statuses: Some(CLAIM_STATUSES),
    },
    CsvSpec {
        path: "data/observations.csv",
        headers: &[
            "observation_id",
            "corpus_id",
            "source_refs",
            "observation",
            "reading_order_assumption",
            "confidence",
            "status",
            "notes",
        ],
        required_columns: &["observation_id", "observation", "reading_order_assumption"],
        id_column: "observation_id",
        confidence_columns: &["confidence"],
        reliability_columns: &[],
        statuses: Some(RECORD_STATUSES),
    },
    CsvSpec {
        path: "data/sequences.csv",
        headers: &[
            "sequence_id",
            "sequence",
            "corpus_refs",
            "source_refs",
            "observation_refs",
            "position_pattern",
            "possible_function",
            "confidence",
            "status",
            "notes",
        ],
        required_columns: &["sequence_id", "sequence"],
        id_column: "sequence_id",
        confidence_columns: &["confidence"],
        reliability_columns: &[],
        statuses: Some(RECORD_STATUSES),
    },
    CsvSpec {
        path: "data/readings.csv",
        headers: &[
            "reading_id",
            "sign_or_sequence",
            "proposed_meaning",
            "evidence_refs",
            "corpus_refs",
            "observation_refs",
            "alternative_explanations",
            "confidence",
            "status",
            "notes",
        ],
        required_columns: &["reading_id", "sign_or_sequence"],
        id_column: "reading_id",
        confidence_columns: &["confidence"],
        reliability_columns: &[],
        statuses: Some(RECORD_STATUSES),
    },
];

pub const TRANSCRIPTIONS_CSV: &str = "data/transcriptions/lines.csv";
pub const TRANSCRIPTIONS_LICENSE: &str = "data/transcriptions/LICENSE.md";

const TRANSCRIPTION_SPEC: CsvSpec = CsvSpec {
    path: TRANSCRIPTIONS_CSV,
    headers: &[
        "line_id",
        "corpus_id",
        "side",
        "source_ref",
        "license",
        "raw_text",
        "marked_text",
        "notes",
    ],
    required_columns: &[
        "line_id",
        "corpus_id",
        "side",
        "source_ref",
        "license",
        "raw_text",
        "marked_text",
    ],
    id_column: "line_id",
    confidence_columns: &[],
    reliability_columns: &[],
    statuses: None,
};

const MARKDOWN_FILES: &[&str] = &[
    "README.md",
    "research-dossier.md",
    "decipherment-notebook.md",
];

pub fn validate_workspace(root: &Path, strict: bool) -> Result<ValidationReport> {
    let mut report = ValidationReport {
        checked_files: 0,
        warning_count: 0,
        error_count: 0,
        schema_valid: true,
        messages: Vec::new(),
    };

    for label in MARKDOWN_FILES {
        check_markdown_file(&mut report, root, label, strict)?;
    }

    for spec in CSV_SPECS {
        check_csv(&mut report, root, spec, strict)?;
    }

    // Transcriptions are optional, but if present they must carry a license notice.
    if root.join(TRANSCRIPTIONS_CSV).exists() {
        check_csv(&mut report, root, &TRANSCRIPTION_SPEC, strict)?;
        report.checked_files += 1;
        if !root.join(TRANSCRIPTIONS_LICENSE).is_file() {
            report.error(format!(
                "{TRANSCRIPTIONS_CSV} is present but {TRANSCRIPTIONS_LICENSE} is missing"
            ));
        }
    }

    Ok(report)
}

fn placeholder_level(strict: bool) -> ValidationLevel {
    if strict {
        ValidationLevel::Error
    } else {
        ValidationLevel::Warning
    }
}

fn check_markdown_file(
    report: &mut ValidationReport,
    root: &Path,
    label: &str,
    strict: bool,
) -> Result<()> {
    report.checked_files += 1;
    let path = root.join(label);
    if !path.is_file() {
        report.schema_valid = false;
        report.error(format!("missing required file {label}"));
        return Ok(());
    }
    report.push(ValidationLevel::Ok, format!("found {label}"));
    let text = std::fs::read_to_string(&path)
        .with_context(|| format!("failed to read {}", path.display()))?;
    if text.contains("TBD") {
        report.push(
            placeholder_level(strict),
            format!("{label} still contains TBD placeholders"),
        );
    }
    Ok(())
}

fn check_csv(
    report: &mut ValidationReport,
    root: &Path,
    spec: &CsvSpec,
    strict: bool,
) -> Result<()> {
    report.checked_files += 1;
    let label = spec.path;
    let path = root.join(label);
    if !path.is_file() {
        report.schema_valid = false;
        report.error(format!("missing required file {label}"));
        return Ok(());
    }

    let mut reader = csv::Reader::from_path(&path)
        .with_context(|| format!("failed to open CSV at {}", path.display()))?;
    let headers = reader
        .headers()
        .with_context(|| format!("failed to read CSV headers from {}", path.display()))?
        .clone();

    let missing: Vec<&str> = spec
        .headers
        .iter()
        .copied()
        .filter(|expected| !headers.iter().any(|actual| actual == *expected))
        .collect();
    if !missing.is_empty() {
        report.schema_valid = false;
        report.error(format!(
            "{label} is missing required header(s): {}",
            missing.join(", ")
        ));
        return Ok(());
    }
    report.push(ValidationLevel::Ok, format!("valid CSV headers in {label}"));

    let mut seen_ids = std::collections::HashSet::new();
    for (index, record) in reader.records().enumerate() {
        let row = index + 2;
        let record = match record {
            Ok(record) => record,
            Err(error) => {
                report.schema_valid = false;
                report.error(format!("{label} row {row} could not be parsed: {error}"));
                continue;
            }
        };
        let value = |column: &str| -> &str {
            headers
                .iter()
                .position(|header| header == column)
                .and_then(|index| record.get(index))
                .map(str::trim)
                .unwrap_or("")
        };

        if record
            .iter()
            .any(|field| field.trim().eq_ignore_ascii_case("TBD"))
        {
            report.push(
                placeholder_level(strict),
                format!("{label} row {row} contains TBD placeholder"),
            );
        }

        for column in spec.required_columns {
            let field = value(column);
            if field.is_empty() {
                report.error(format!("{label} row {row} has empty {column}"));
                continue;
            }
            if *column == spec.id_column && !seen_ids.insert(field.to_string()) {
                report.error(format!("{label} row {row} duplicates {column}={field}"));
            }
        }

        for column in spec.confidence_columns {
            check_enum(report, label, row, column, value(column), CONFIDENCE_LABELS);
        }
        for column in spec.reliability_columns {
            check_enum(
                report,
                label,
                row,
                column,
                value(column),
                RELIABILITY_LABELS,
            );
        }
        if let Some(statuses) = spec.statuses {
            check_enum(report, label, row, "status", value("status"), statuses);
        }
    }

    Ok(())
}

fn check_enum(
    report: &mut ValidationReport,
    label: &str,
    row: usize,
    column: &str,
    value: &str,
    allowed: &[&str],
) {
    // Placeholders are reported separately by the TBD check.
    if value.eq_ignore_ascii_case("TBD") {
        return;
    }
    if !allowed.contains(&value) {
        let shown = if value.is_empty() { "<empty>" } else { value };
        report.error(format!(
            "{label} row {row} has invalid {column}={shown}; expected one of: {}",
            allowed.join(", ")
        ));
    }
}
