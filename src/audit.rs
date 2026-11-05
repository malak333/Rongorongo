use crate::model::{
    ClaimRecord, CorpusObject, HypothesisRecord, ObservationRecord, ReadingRecord, SequenceRecord,
    SourceRecord,
};
use crate::transcription::{self, TranscriptionLine};
use crate::validate::{ValidationReport, TRANSCRIPTIONS_CSV};
use crate::workflow::{self, KnownIds};
use anyhow::{Context, Result};
use serde::Serialize;
use std::collections::{HashMap, HashSet};
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
    pub transcription_lines: usize,
    pub promoted_claims: usize,
    pub active_hypotheses: usize,
}

#[derive(Debug)]
pub struct AuditReport {
    pub validation: ValidationReport,
    pub summary: Option<AuditSummary>,
}

/// Sentinel used in `readings.proposed_meaning` for an explicit non-claim.
pub const NO_READING: &str = "none-proposed";

pub struct Workspace {
    pub corpus: Vec<CorpusObject>,
    pub sources: Vec<SourceRecord>,
    pub hypotheses: Vec<HypothesisRecord>,
    pub claims: Vec<ClaimRecord>,
    pub observations: Vec<ObservationRecord>,
    pub sequences: Vec<SequenceRecord>,
    pub readings: Vec<ReadingRecord>,
    pub transcriptions: Vec<TranscriptionLine>,
}

impl Workspace {
    pub fn load(root: &Path) -> Result<Self> {
        Ok(Self {
            corpus: read_csv(&root.join("data/corpus-index.csv"))?,
            sources: read_csv(&root.join("data/source-registry.csv"))?,
            hypotheses: read_csv(&root.join("data/hypotheses.csv"))?,
            claims: read_csv(&root.join("data/claims.csv"))?,
            observations: read_csv(&root.join("data/observations.csv"))?,
            sequences: read_csv(&root.join("data/sequences.csv"))?,
            readings: read_csv(&root.join("data/readings.csv"))?,
            transcriptions: read_transcriptions(root)?,
        })
    }
}

pub fn audit_workspace(root: &Path, strict: bool) -> Result<AuditReport> {
    let mut validation = crate::validate::validate_workspace(root, strict)?;
    if !validation.schema_valid {
        validation.error(
            "cross-reference audit skipped because required files or headers are invalid"
                .to_string(),
        );
        return Ok(AuditReport {
            validation,
            summary: None,
        });
    }

    let ws = Workspace::load(root)?;
    let report = &mut validation;

    let known = KnownIds {
        sources: ws.sources.iter().map(|s| s.source_id.as_str()).collect(),
        corpus: ws.corpus.iter().map(|c| c.catalog_id.as_str()).collect(),
        observations: ws
            .observations
            .iter()
            .map(|o| o.observation_id.as_str())
            .collect(),
    };
    let reliability: HashMap<&str, &str> = ws
        .sources
        .iter()
        .map(|s| (s.source_id.as_str(), s.reliability.as_str()))
        .collect();
    let observations: HashMap<&str, &ObservationRecord> = ws
        .observations
        .iter()
        .map(|o| (o.observation_id.as_str(), o))
        .collect();

    for object in &ws.corpus {
        let subject = format!("corpus {}", object.catalog_id);
        check_refs(
            report,
            &subject,
            "transcription_source",
            &object.transcription_source,
            &known.sources,
            false,
        );
        // The corpus row restates its transcription source's reliability; keep them in sync.
        if let Some(actual) = reliability.get(object.transcription_source.trim()) {
            if object.source_reliability != *actual {
                report.error(format!(
                    "{subject} source_reliability={} but {} has reliability={actual}",
                    object.source_reliability, object.transcription_source
                ));
            }
        }
        check_confidence_cap(
            report,
            &subject,
            "inclusion_confidence",
            &object.inclusion_confidence,
            &object.transcription_source,
            &reliability,
        );
    }

    for claim in &ws.claims {
        let subject = format!("claim {}", claim.claim_id);
        check_refs(
            report,
            &subject,
            "evidence_refs",
            &claim.evidence_refs,
            &known.sources,
            false,
        );
        check_refs(
            report,
            &subject,
            "corpus_refs",
            &claim.corpus_refs,
            &known.corpus,
            true,
        );
        check_confidence_cap(
            report,
            &subject,
            "confidence",
            &claim.confidence,
            &claim.evidence_refs,
            &reliability,
        );
        if claim.status == "promoted" {
            require_gate(
                report,
                &subject,
                workflow::claim_promotion_checks(claim, &known),
            );
        }
    }

    for observation in &ws.observations {
        let subject = format!("observation {}", observation.observation_id);
        check_refs(
            report,
            &subject,
            "source_refs",
            &observation.source_refs,
            &known.sources,
            false,
        );
        check_refs(
            report,
            &subject,
            "corpus_id",
            &observation.corpus_id,
            &known.corpus,
            false,
        );
        check_confidence_cap(
            report,
            &subject,
            "confidence",
            &observation.confidence,
            &observation.source_refs,
            &reliability,
        );
        if observation.status == "promoted" {
            require_promotable_confidence(report, &subject, &observation.confidence);
        }
    }

    for hypothesis in &ws.hypotheses {
        let subject = format!("hypothesis {}", hypothesis.hypothesis_id);
        check_refs(
            report,
            &subject,
            "evidence_refs",
            &hypothesis.evidence_refs,
            &known.sources,
            false,
        );
        check_refs(
            report,
            &subject,
            "corpus_refs",
            &hypothesis.corpus_refs,
            &known.corpus,
            true,
        );
        check_refs(
            report,
            &subject,
            "observation_refs",
            &hypothesis.observation_refs,
            &known.observations,
            false,
        );
        check_confidence_cap(
            report,
            &subject,
            "confidence",
            &hypothesis.confidence,
            &hypothesis.evidence_refs,
            &reliability,
        );
        if hypothesis.status == "accepted-working-rule" {
            require_gate(
                report,
                &subject,
                workflow::hypothesis_promotion_checks(hypothesis, &known),
            );
        }
    }

    for sequence in &ws.sequences {
        let subject = format!("sequence {}", sequence.sequence_id);
        check_refs(
            report,
            &subject,
            "source_refs",
            &sequence.source_refs,
            &known.sources,
            false,
        );
        check_refs(
            report,
            &subject,
            "corpus_refs",
            &sequence.corpus_refs,
            &known.corpus,
            true,
        );
        check_refs(
            report,
            &subject,
            "observation_refs",
            &sequence.observation_refs,
            &known.observations,
            false,
        );
        check_confidence_cap(
            report,
            &subject,
            "confidence",
            &sequence.confidence,
            &sequence.source_refs,
            &reliability,
        );
        if sequence.status == "promoted" {
            require_promotable_confidence(report, &subject, &sequence.confidence);
        }
    }

    for reading in &ws.readings {
        let subject = format!("reading {}", reading.reading_id);
        check_refs(
            report,
            &subject,
            "evidence_refs",
            &reading.evidence_refs,
            &known.sources,
            false,
        );
        check_refs(
            report,
            &subject,
            "corpus_refs",
            &reading.corpus_refs,
            &known.corpus,
            true,
        );
        check_refs(
            report,
            &subject,
            "observation_refs",
            &reading.observation_refs,
            &known.observations,
            false,
        );
        check_confidence_cap(
            report,
            &subject,
            "confidence",
            &reading.confidence,
            &reading.evidence_refs,
            &reliability,
        );
        if reading.alternative_explanations.trim().is_empty() {
            report.error(format!("{subject} is missing alternative explanations"));
        }
        if reading.proposed_meaning.trim() != NO_READING {
            check_semantic_reading(report, &subject, reading, &observations);
        }
        if reading.status == "promoted" {
            require_promotable_confidence(report, &subject, &reading.confidence);
        }
    }

    check_transcriptions(report, &ws, &known);

    crate::docs_sync::check_markdown_sync(report, root, &ws)?;

    let summary = AuditSummary {
        corpus_objects: ws.corpus.len(),
        sources: ws.sources.len(),
        claims: ws.claims.len(),
        observations: ws.observations.len(),
        sequences: ws.sequences.len(),
        readings: ws.readings.len(),
        hypotheses: ws.hypotheses.len(),
        transcription_lines: ws.transcriptions.len(),
        promoted_claims: ws.claims.iter().filter(|c| c.status == "promoted").count(),
        active_hypotheses: ws
            .hypotheses
            .iter()
            .filter(|h| h.status == "active" || h.status == "accepted-working-rule")
            .count(),
    };

    Ok(AuditReport {
        validation,
        summary: Some(summary),
    })
}

/// Transcription rows must point at a registered object and source, use one of
/// the object's sides, carry a license, and parse into at least one element.
fn check_transcriptions(report: &mut ValidationReport, ws: &Workspace, known: &KnownIds) {
    let sides: HashMap<&str, Vec<&str>> = ws
        .corpus
        .iter()
        .map(|c| (c.catalog_id.as_str(), split_refs(&c.sides).collect()))
        .collect();
    for line in &ws.transcriptions {
        let subject = format!("transcription {}", line.line_id);
        check_refs(
            report,
            &subject,
            "source_ref",
            &line.source_ref,
            &known.sources,
            false,
        );
        check_refs(
            report,
            &subject,
            "corpus_id",
            &line.corpus_id,
            &known.corpus,
            false,
        );
        if let Some(object_sides) = sides.get(line.corpus_id.as_str()) {
            if !object_sides.contains(&line.side.as_str()) {
                report.error(format!(
                    "{subject} side={} is not one of {}'s sides ({})",
                    line.side,
                    line.corpus_id,
                    object_sides.join("; ")
                ));
            }
        }
        let number = line.line_id.strip_prefix(line.side.as_str());
        if !number.is_some_and(|n| !n.is_empty() && n.chars().all(|c| c.is_ascii_digit())) {
            report.error(format!(
                "{subject} line_id must be the side followed by a line number (e.g. {}1)",
                line.side
            ));
        }
        if transcription::parse_line(&line.line_id, &line.raw_text).is_empty() {
            report.error(format!("{subject} raw_text contains no graphic elements"));
        }
        // marked_text is raw_text plus `{}` around underlined (doubtful) spans.
        if !transcription::marks_are_balanced(&line.marked_text) {
            report.error(format!(
                "{subject} marked_text has unbalanced or nested {{}} marks"
            ));
        } else {
            let unmarked: String = transcription::unmark(&line.marked_text)
                .into_iter()
                .map(|(c, _)| c)
                .collect();
            if unmarked != line.raw_text {
                report.error(format!(
                    "{subject} marked_text without {{}} marks does not equal raw_text"
                ));
            }
        }
    }
}

/// Rank used to compare confidence and reliability labels. `Mixed` sources
/// count as Medium: they can support Medium conclusions but not High ones.
fn rank(label: &str) -> Option<u8> {
    match label {
        "High" => Some(3),
        "Medium" | "Mixed" => Some(2),
        "Low" => Some(1),
        _ => None,
    }
}

/// A row's confidence may not exceed the reliability of the strongest source it cites.
fn check_confidence_cap(
    report: &mut ValidationReport,
    subject: &str,
    column: &str,
    confidence: &str,
    refs: &str,
    reliability: &HashMap<&str, &str>,
) {
    let Some(claimed) = rank(confidence) else {
        return;
    };
    let strongest = split_refs(refs)
        .filter_map(|id| reliability.get(id).copied())
        .max_by_key(|label| rank(label).unwrap_or(0));
    let Some(strongest) = strongest else {
        return;
    };
    if claimed > rank(strongest).unwrap_or(0) {
        report.error(format!(
            "{subject} {column}={confidence} exceeds the strongest cited source reliability ({strongest})"
        ));
    }
}

/// A semantic reading must rest on promoted structural observations, and may
/// not be more confident than the weakest observation it depends on.
fn check_semantic_reading(
    report: &mut ValidationReport,
    subject: &str,
    reading: &ReadingRecord,
    observations: &HashMap<&str, &ObservationRecord>,
) {
    for id in split_refs(&reading.observation_refs) {
        let Some(observation) = observations.get(id) else {
            continue; // unknown ids are already reported by check_refs
        };
        if observation.status != "promoted" {
            report.error(format!(
                "{subject} proposes a meaning but cites observation {id} with status={}; semantic readings require promoted observations",
                observation.status
            ));
        }
        if let (Some(claimed), Some(basis)) =
            (rank(&reading.confidence), rank(&observation.confidence))
        {
            if claimed > basis {
                report.error(format!(
                    "{subject} confidence={} exceeds the confidence of observation {id} ({})",
                    reading.confidence, observation.confidence
                ));
            }
        }
    }
}

fn require_gate(
    report: &mut ValidationReport,
    subject: &str,
    checks: Vec<workflow::PromotionCheckItem>,
) {
    for item in checks.into_iter().filter(|item| !item.passed) {
        report.error(format!(
            "{subject} is marked promoted but fails promotion gate '{}': {}",
            item.name, item.detail
        ));
    }
}

fn require_promotable_confidence(report: &mut ValidationReport, subject: &str, confidence: &str) {
    if !workflow::is_promotable_confidence(confidence) {
        report.error(format!(
            "{subject} is marked promoted but has confidence={confidence}; promotion requires Medium or High"
        ));
    }
}

pub fn split_refs(refs: &str) -> impl Iterator<Item = &str> {
    refs.split(';')
        .map(str::trim)
        .filter(|value| !value.is_empty())
}

fn check_refs(
    report: &mut ValidationReport,
    subject: &str,
    column: &str,
    refs: &str,
    known: &HashSet<&str>,
    allow_corpus_wide: bool,
) {
    let mut any = false;
    for value in split_refs(refs) {
        any = true;
        if allow_corpus_wide && value == "corpus-wide" {
            continue;
        }
        if !known.contains(value) {
            report.error(format!("{subject} {column} references unknown id {value}"));
        }
    }
    if !any {
        report.error(format!("{subject} {column} is empty"));
    }
}

pub fn read_transcriptions(root: &Path) -> Result<Vec<TranscriptionLine>> {
    let path = root.join(TRANSCRIPTIONS_CSV);
    if path.exists() {
        read_csv(&path)
    } else {
        Ok(Vec::new())
    }
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
