use crate::model::{ClaimRecord, CorpusObject, HypothesisRecord, ObservationRecord, SourceRecord};
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

    let source_ids: HashSet<&str> = sources
        .iter()
        .map(|source| source.source_id.as_str())
        .collect();
    let corpus_ids: HashSet<&str> = corpus
        .iter()
        .map(|object| object.catalog_id.as_str())
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
        check_text_has_known_ref(
            &mut validation,
            "hypothesis evidence",
            &hypothesis.hypothesis_id,
            &hypothesis.evidence,
            &source_ids,
        );
    }

    let summary = AuditSummary {
        corpus_objects: corpus.len(),
        sources: sources.len(),
        claims: claims.len(),
        observations: observations.len(),
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

fn check_optional_refs(
    report: &mut ValidationReport,
    label: &str,
    subject: &str,
    refs: &str,
    known: &HashSet<&str>,
) {
    if refs.trim().is_empty() {
        return;
    }
    check_refs(report, label, subject, refs, known);
}

fn check_text_has_known_ref(
    report: &mut ValidationReport,
    label: &str,
    subject: &str,
    text: &str,
    known: &HashSet<&str>,
) {
    if known.iter().any(|source_id| text.contains(*source_id)) {
        return;
    }
    push_error(
        report,
        format!("{label} for {subject} does not cite a known source id"),
    );
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
