use std::fmt;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct ValidationReport {
    pub checked_files: usize,
    pub warning_count: usize,
    pub error_count: usize,
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

#[derive(Debug, Error)]
#[error("CSV {path} is missing required header(s): {missing:?}")]
struct MissingHeaders {
    path: PathBuf,
    missing: Vec<String>,
}

pub fn validate_workspace(root: &Path, strict: bool) -> Result<ValidationReport> {
    let mut report = ValidationReport {
        checked_files: 0,
        warning_count: 0,
        error_count: 0,
        messages: Vec::new(),
    };

    for required in [
        "README.md",
        "research-dossier.md",
        "decipherment-notebook.md",
        "data/corpus-index.csv",
        "data/source-registry.csv",
        "data/hypotheses.csv",
        "data/claims.csv",
        "data/observations.csv",
        "data/sequences.csv",
        "data/readings.csv",
    ] {
        check_required_file(&mut report, &root.join(required), required, strict)?;
    }

    check_csv_headers(
        &mut report,
        &root.join("data/corpus-index.csv"),
        &[
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
    )?;

    check_csv_records(
        &mut report,
        &root.join("data/corpus-index.csv"),
        strict,
        &["object_name", "catalog_id"],
        &["source_reliability", "inclusion_confidence"],
    )?;

    check_optional_csv_headers(
        &mut report,
        &root.join("data/source-registry.csv"),
        &[
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
    )?;

    if root.join("data/source-registry.csv").exists() {
        check_csv_records(
            &mut report,
            &root.join("data/source-registry.csv"),
            strict,
            &["source_id", "citation"],
            &["reliability"],
        )?;
    }

    check_optional_csv_headers(
        &mut report,
        &root.join("data/hypotheses.csv"),
        &[
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
    )?;

    if root.join("data/hypotheses.csv").exists() {
        check_csv_records(
            &mut report,
            &root.join("data/hypotheses.csv"),
            strict,
            &["hypothesis_id", "claim"],
            &["confidence"],
        )?;
    }

    check_csv_headers(
        &mut report,
        &root.join("data/claims.csv"),
        &[
            "claim_id",
            "claim",
            "claim_type",
            "evidence_refs",
            "corpus_refs",
            "confidence",
            "status",
            "notes",
        ],
    )?;

    check_csv_records(
        &mut report,
        &root.join("data/claims.csv"),
        strict,
        &["claim_id", "claim"],
        &["confidence"],
    )?;

    check_csv_headers(
        &mut report,
        &root.join("data/observations.csv"),
        &[
            "observation_id",
            "corpus_id",
            "source_refs",
            "observation",
            "reading_order_assumption",
            "confidence",
            "status",
            "notes",
        ],
    )?;

    check_csv_records(
        &mut report,
        &root.join("data/observations.csv"),
        strict,
        &["observation_id", "observation"],
        &["confidence"],
    )?;

    check_csv_headers(
        &mut report,
        &root.join("data/sequences.csv"),
        &[
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
    )?;

    check_csv_records(
        &mut report,
        &root.join("data/sequences.csv"),
        strict,
        &["sequence_id", "sequence"],
        &["confidence"],
    )?;

    check_csv_headers(
        &mut report,
        &root.join("data/readings.csv"),
        &[
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
    )?;

    check_csv_records(
        &mut report,
        &root.join("data/readings.csv"),
        strict,
        &["reading_id", "sign_or_sequence"],
        &["confidence"],
    )?;

    Ok(report)
}

fn check_required_file(
    report: &mut ValidationReport,
    path: &Path,
    label: &str,
    strict: bool,
) -> Result<()> {
    report.checked_files += 1;
    if path.is_file() {
        push(report, ValidationLevel::Ok, format!("found {label}"));
        let text = std::fs::read_to_string(path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        if text.contains("TBD") {
            let level = if strict {
                ValidationLevel::Error
            } else {
                ValidationLevel::Warning
            };
            push(
                report,
                level,
                format!("{label} still contains TBD placeholders"),
            );
        }
    } else {
        push(
            report,
            ValidationLevel::Error,
            format!("missing required file {label}"),
        );
    }
    Ok(())
}

fn check_csv_records(
    report: &mut ValidationReport,
    path: &Path,
    strict: bool,
    identity_columns: &[&str],
    enum_columns: &[&str],
) -> Result<()> {
    let mut reader = csv::Reader::from_path(path)
        .with_context(|| format!("failed to open CSV at {}", path.display()))?;
    let headers = reader
        .headers()
        .with_context(|| format!("failed to read CSV headers from {}", path.display()))?
        .clone();
    let mut seen_ids = std::collections::HashSet::new();

    for (index, record) in reader.records().enumerate() {
        let row_number = index + 2;
        let record = record.with_context(|| {
            format!(
                "failed to read CSV row {row_number} from {}",
                path.display()
            )
        })?;

        if record
            .iter()
            .any(|field| field.trim().eq_ignore_ascii_case("TBD"))
        {
            let level = if strict {
                ValidationLevel::Error
            } else {
                ValidationLevel::Warning
            };
            push(
                report,
                level,
                format!(
                    "{} row {row_number} contains TBD placeholder",
                    path.display()
                ),
            );
        }

        for column in identity_columns {
            if let Some(value) = value_for(&headers, &record, column) {
                let trimmed = value.trim();
                if trimmed.is_empty() {
                    push(
                        report,
                        ValidationLevel::Error,
                        format!("{} row {row_number} has empty {column}", path.display()),
                    );
                }
                if *column == "catalog_id" || column.ends_with("_id") {
                    let key = format!("{column}:{trimmed}");
                    if !trimmed.is_empty() && !seen_ids.insert(key) {
                        push(
                            report,
                            ValidationLevel::Error,
                            format!(
                                "{} row {row_number} duplicates {column}={trimmed}",
                                path.display()
                            ),
                        );
                    }
                }
            }
        }

        for column in enum_columns {
            if let Some(value) = value_for(&headers, &record, column) {
                let value = value.trim();
                if !value.is_empty()
                    && !value.eq_ignore_ascii_case("TBD")
                    && !["High", "Medium", "Low", "Mixed"].contains(&value)
                {
                    push(
                        report,
                        ValidationLevel::Error,
                        format!(
                            "{} row {row_number} has invalid {column}={value}; expected High, Medium, Low, or Mixed",
                            path.display()
                        ),
                    );
                }
            }
        }
    }

    Ok(())
}

fn value_for<'a>(
    headers: &csv::StringRecord,
    record: &'a csv::StringRecord,
    column: &str,
) -> Option<&'a str> {
    headers
        .iter()
        .position(|header| header == column)
        .and_then(|index| record.get(index))
}

fn check_optional_csv_headers(
    report: &mut ValidationReport,
    path: &Path,
    expected: &[&str],
) -> Result<()> {
    if path.exists() {
        check_csv_headers(report, path, expected)?;
    } else {
        push(
            report,
            ValidationLevel::Warning,
            format!("optional CSV is not present: {}", path.display()),
        );
    }
    Ok(())
}

fn check_csv_headers(report: &mut ValidationReport, path: &Path, expected: &[&str]) -> Result<()> {
    report.checked_files += 1;
    let mut reader = csv::Reader::from_path(path)
        .with_context(|| format!("failed to open CSV at {}", path.display()))?;
    let headers = reader
        .headers()
        .with_context(|| format!("failed to read CSV headers from {}", path.display()))?
        .clone();
    let missing: Vec<String> = expected
        .iter()
        .filter(|header| !headers.iter().any(|actual| actual == **header))
        .map(|header| (*header).to_string())
        .collect();

    if missing.is_empty() {
        push(
            report,
            ValidationLevel::Ok,
            format!("valid CSV headers in {}", path.display()),
        );
        Ok(())
    } else {
        push(
            report,
            ValidationLevel::Error,
            format!("invalid CSV headers in {}", path.display()),
        );
        Err(MissingHeaders {
            path: path.to_path_buf(),
            missing,
        }
        .into())
    }
}

fn push(report: &mut ValidationReport, level: ValidationLevel, message: String) {
    match level {
        ValidationLevel::Ok => {}
        ValidationLevel::Warning => report.warning_count += 1,
        ValidationLevel::Error => report.error_count += 1,
    }
    report.messages.push(ValidationMessage { level, message });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_current_workspace_shape() {
        let report = validate_workspace(Path::new(env!("CARGO_MANIFEST_DIR")), false).unwrap();
        assert_eq!(report.error_count, 0);
        assert!(report.checked_files >= 5);
    }

    #[test]
    fn strict_mode_rejects_placeholders() {
        let temp = tempfile::tempdir().unwrap();
        write_minimal_workspace(temp.path(), "TBD").unwrap();

        let report = validate_workspace(temp.path(), true).unwrap();
        assert!(report.error_count > 0);
    }

    fn write_minimal_workspace(root: &Path, object_name: &str) -> std::io::Result<()> {
        std::fs::create_dir_all(root.join("data"))?;
        std::fs::write(root.join("README.md"), "README\n")?;
        std::fs::write(root.join("research-dossier.md"), "Dossier\n")?;
        std::fs::write(root.join("decipherment-notebook.md"), "Notebook\n")?;
        std::fs::write(
            root.join("data/corpus-index.csv"),
            format!(
            "object_name,catalog_id,current_location,sides,transcription_source,source_reliability,inclusion_confidence,reference_url,notes\n{object_name},D,Rome,Da; Db,SRC-003,High,High,https://example.invalid,fixture\n"
            ),
        )?;
        std::fs::write(
            root.join("data/source-registry.csv"),
        "source_id,citation,year,source_type,contribution,limits,reliability,access,url,accessed,notes\nSRC-003,Fixture,2022,peer-reviewed-open-access,Fixture,Fixture,High,open-access,https://example.invalid,2026-05-19,fixture\n",
        )?;
        std::fs::write(
            root.join("data/hypotheses.csv"),
        "hypothesis_id,claim,evidence,evidence_refs,corpus_refs,observation_refs,test,status,confidence,notes\nH-001,Fixture,SRC-003,SRC-003,D,O-001,Fixture,active,High,fixture\n",
        )?;
        std::fs::write(
            root.join("data/claims.csv"),
            "claim_id,claim,claim_type,evidence_refs,corpus_refs,confidence,status,notes\nC-001,Fixture,method,SRC-003,D,High,active,fixture\n",
        )?;
        std::fs::write(
            root.join("data/observations.csv"),
            "observation_id,corpus_id,source_refs,observation,reading_order_assumption,confidence,status,notes\nO-001,D,SRC-003,Fixture,Fixture,High,active,fixture\n",
        )?;
        std::fs::write(
            root.join("data/sequences.csv"),
            "sequence_id,sequence,corpus_refs,source_refs,observation_refs,position_pattern,possible_function,confidence,status,notes\nS-001,fixture,D,SRC-003,O-001,fixture,fixture,High,active,fixture\n",
        )?;
        std::fs::write(
            root.join("data/readings.csv"),
            "reading_id,sign_or_sequence,proposed_meaning,evidence_refs,corpus_refs,observation_refs,alternative_explanations,confidence,status,notes\nR-001,fixture,fixture,SRC-003,D,O-001,fixture,High,active,fixture\n",
        )?;
        Ok(())
    }
}
