use crate::model::{
    ClaimRecord, CorpusObject, HypothesisRecord, ObservationRecord, ReadingRecord, SequenceRecord,
    SourceRecord,
};
use crate::validate::{ValidationLevel, ValidationMessage, ValidationReport};
use anyhow::{Context, Result};
use serde::Serialize;
use std::collections::HashSet;
use std::path::Path;

#[derive(Debug, Serialize)]
pub struct AuditSummary {
    pub corpus_objects: usize,
    pub sources: usize,
    pub claims: usize,
    pub observations: usize,
    pub sequences: usize,
    pub readings: usize,
    pub hypotheses: usize,
    pub promoted_claims: usize,
    pub active_hypotheses: usize,
}

#[derive(Debug)]
pub struct AuditReport {
    pub validation: ValidationReport,
    pub summary: AuditSummary,
}

pub fn audit_workspace(root: &Path, strict: bool) -> Result<AuditReport> {
    let mut validation = crate::validate::validate_workspace(root, strict)?;

    let corpus = read_csv::<CorpusObject>(&root.join("data/corpus-index.csv"))?;
    let sources = read_csv::<SourceRecord>(&root.join("data/source-registry.csv"))?;
    let hypotheses = read_csv::<HypothesisRecord>(&root.join("data/hypotheses.csv"))?;
    let claims = read_csv::<ClaimRecord>(&root.join("data/claims.csv"))?;
    let observations = read_csv::<ObservationRecord>(&root.join("data/observations.csv"))?;
    let sequences = read_csv::<SequenceRecord>(&root.join("data/sequences.csv"))?;
    let readings = read_csv::<ReadingRecord>(&root.join("data/readings.csv"))?;

    let source_ids: HashSet<&str> = sources
        .iter()
        .map(|source| source.source_id.as_str())
        .collect();
    let corpus_ids: HashSet<&str> = corpus
        .iter()
        .map(|object| object.catalog_id.as_str())
        .collect();
    let observation_ids: HashSet<&str> = observations
        .iter()
        .map(|observation| observation.observation_id.as_str())
        .collect();

    for object in &corpus {
        check_refs(
            &mut validation,
            "corpus transcription_source",
            &object.object_name,
            &object.transcription_source,
            &source_ids,
        );
    }

    for claim in &claims {
        check_refs(
            &mut validation,
            "claim evidence_refs",
            &claim.claim_id,
            &claim.evidence_refs,
            &source_ids,
        );
        check_optional_refs(
            &mut validation,
            "claim corpus_refs",
            &claim.claim_id,
            &claim.corpus_refs,
            &corpus_ids,
            true,
        );
    }

    for observation in &observations {
        check_refs(
            &mut validation,
            "observation source_refs",
            &observation.observation_id,
            &observation.source_refs,
            &source_ids,
        );
        check_refs(
            &mut validation,
            "observation corpus_id",
            &observation.observation_id,
            &observation.corpus_id,
            &corpus_ids,
        );
    }

    for hypothesis in &hypotheses {
        check_refs(
            &mut validation,
            "hypothesis evidence_refs",
            &hypothesis.hypothesis_id,
            &hypothesis.evidence_refs,
            &source_ids,
        );
        check_corpus_refs(
            &mut validation,
            "hypothesis corpus_refs",
            &hypothesis.hypothesis_id,
            &hypothesis.corpus_refs,
            &corpus_ids,
        );
        check_refs(
            &mut validation,
            "hypothesis observation_refs",
            &hypothesis.hypothesis_id,
            &hypothesis.observation_refs,
            &observation_ids,
        );
    }

    for sequence in &sequences {
        check_refs(
            &mut validation,
            "sequence source_refs",
            &sequence.sequence_id,
            &sequence.source_refs,
            &source_ids,
        );
        check_corpus_refs(
            &mut validation,
            "sequence corpus_refs",
            &sequence.sequence_id,
            &sequence.corpus_refs,
            &corpus_ids,
        );
        check_refs(
            &mut validation,
            "sequence observation_refs",
            &sequence.sequence_id,
            &sequence.observation_refs,
            &observation_ids,
        );
    }

    for reading in &readings {
        check_refs(
            &mut validation,
            "reading evidence_refs",
            &reading.reading_id,
            &reading.evidence_refs,
            &source_ids,
        );
        check_corpus_refs(
            &mut validation,
            "reading corpus_refs",
            &reading.reading_id,
            &reading.corpus_refs,
            &corpus_ids,
        );
        check_refs(
            &mut validation,
            "reading observation_refs",
            &reading.reading_id,
            &reading.observation_refs,
            &observation_ids,
        );
        if reading.alternative_explanations.trim().is_empty() {
            push_error(
                &mut validation,
                format!(
                    "reading {} is missing alternative explanations",
                    reading.reading_id
                ),
            );
        }
    }

    let summary = AuditSummary {
        corpus_objects: corpus.len(),
        sources: sources.len(),
        claims: claims.len(),
        observations: observations.len(),
        sequences: sequences.len(),
        readings: readings.len(),
        hypotheses: hypotheses.len(),
        promoted_claims: claims
            .iter()
            .filter(|claim| claim.status.eq_ignore_ascii_case("promoted"))
            .count(),
        active_hypotheses: hypotheses
            .iter()
            .filter(|hypothesis| {
                hypothesis.status.eq_ignore_ascii_case("active")
                    || hypothesis
                        .status
                        .eq_ignore_ascii_case("accepted-working-rule")
            })
            .count(),
    };

    Ok(AuditReport {
        validation,
        summary,
    })
}

fn read_csv<T>(path: &Path) -> Result<Vec<T>>
where
    T: for<'de> serde::Deserialize<'de>,
{
    let mut reader = csv::Reader::from_path(path)
        .with_context(|| format!("failed to open CSV at {}", path.display()))?;
    reader
        .deserialize()
        .collect::<Result<Vec<T>, csv::Error>>()
        .with_context(|| format!("failed to parse CSV at {}", path.display()))
}

fn check_refs(
    report: &mut ValidationReport,
    label: &str,
    subject: &str,
    refs: &str,
    known: &HashSet<&str>,
) {
    let values: Vec<&str> = refs
        .split(';')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .collect();

    if values.is_empty() {
        push_error(report, format!("{label} for {subject} is empty"));
        return;
    }

    for value in values {
        if !known.contains(value) {
            push_error(
                report,
                format!("{label} for {subject} references unknown id {value}"),
            );
        }
    }
}

fn check_corpus_refs(
    report: &mut ValidationReport,
    label: &str,
    subject: &str,
    refs: &str,
    known: &HashSet<&str>,
) {
    let values: Vec<&str> = refs
        .split(';')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .collect();

    if values.is_empty() {
        push_error(report, format!("{label} for {subject} is empty"));
        return;
    }

    for value in values {
        if value != "corpus-wide" && !known.contains(value) {
            push_error(
                report,
                format!("{label} for {subject} references unknown id {value}"),
            );
        }
    }
}

fn check_optional_refs(
    report: &mut ValidationReport,
    label: &str,
    subject: &str,
    refs: &str,
    known: &HashSet<&str>,
    allow_corpus_wide: bool,
) {
    if refs.trim().is_empty() {
        push_error(report, format!("{label} for {subject} is empty"));
        return;
    }
    if allow_corpus_wide && refs.trim() == "corpus-wide" {
        return;
    }
    check_refs(report, label, subject, refs, known);
}

fn push_error(report: &mut ValidationReport, message: String) {
    report.error_count += 1;
    report.messages.push(ValidationMessage {
        level: ValidationLevel::Error,
        message,
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audits_current_workspace_cross_references() {
        let report = audit_workspace(Path::new(env!("CARGO_MANIFEST_DIR")), true).unwrap();
        assert_eq!(report.validation.error_count, 0);
        assert_eq!(report.summary.sources, 5);
        assert_eq!(report.summary.claims, 3);
    }
}
