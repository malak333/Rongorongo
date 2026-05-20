use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Table,
    Json,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "table" => Ok(Self::Table),
            "json" => Ok(Self::Json),
            other => Err(format!("unsupported output format: {other}")),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CorpusObject {
    pub object_name: String,
    pub catalog_id: String,
    pub current_location: String,
    pub sides: String,
    pub transcription_source: String,
    pub source_reliability: String,
    pub inclusion_confidence: String,
    pub notes: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SourceRecord {
    pub source_id: String,
    pub citation: String,
    pub year: String,
    pub source_type: String,
    pub contribution: String,
    pub limits: String,
    pub reliability: String,
    pub access: String,
    pub notes: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HypothesisRecord {
    pub hypothesis_id: String,
    pub claim: String,
    pub evidence: String,
    pub test: String,
    pub status: String,
    pub confidence: String,
    pub notes: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClaimRecord {
    pub claim_id: String,
    pub claim: String,
    pub claim_type: String,
    pub evidence_refs: String,
    pub corpus_refs: String,
    pub confidence: String,
    pub status: String,
    pub notes: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ObservationRecord {
    pub observation_id: String,
    pub corpus_id: String,
    pub source_refs: String,
    pub observation: String,
    pub reading_order_assumption: String,
    pub confidence: String,
    pub status: String,
    pub notes: String,
}
