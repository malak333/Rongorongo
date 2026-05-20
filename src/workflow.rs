use crate::model::{ClaimRecord, HypothesisRecord, SourceRecord};
use anyhow::{bail, Context, Result};
use serde::Serialize;
use std::path::Path;

#[derive(Debug, Serialize)]
pub struct PromotionCheck {
    pub id: String,
    pub kind: String,
    pub ready: bool,
    pub checks: Vec<PromotionCheckItem>,
}

#[derive(Debug, Serialize)]
pub struct PromotionCheckItem {
    pub name: String,
    pub passed: bool,
    pub detail: String,
}

pub fn source_template(next_id: &str) -> String {
    format!(
        "source_id,citation,year,source_type,contribution,limits,reliability,access,notes\n{next_id},\"Author, Title\",YYYY,peer-reviewed-open-access,\"What this source contributes\",\"Known limits\",Medium,open-access,\"Public-safe notes only\"\n"
    )
}

pub fn check_claim_promotion(root: &Path, claim_id: &str) -> Result<PromotionCheck> {
    let claims = read_csv::<ClaimRecord>(&root.join("data/claims.csv"))?;
    let sources = read_csv::<SourceRecord>(&root.join("data/source-registry.csv"))?;
    let claim = claims
        .iter()
        .find(|claim| claim.claim_id == claim_id)
        .with_context(|| format!("claim not found: {claim_id}"))?;
    let source_ids: std::collections::HashSet<&str> = sources
        .iter()
        .map(|source| source.source_id.as_str())
        .collect();

    let checks = vec![
        check(
            "has evidence refs",
            !claim.evidence_refs.trim().is_empty(),
            "claim must cite at least one source id",
        ),
        check(
            "evidence refs exist",
            refs_exist(&claim.evidence_refs, &source_ids),
            "all evidence_refs must exist in data/source-registry.csv",
        ),
        check(
            "confidence is not Low",
            !claim.confidence.eq_ignore_ascii_case("Low"),
            "promoted claims require Medium or High confidence",
        ),
        check(
            "status is active or promoted",
            ["active", "promoted"].contains(&claim.status.as_str()),
            "claim status must be active before promotion or already promoted",
        ),
    ];

    Ok(PromotionCheck {
        id: claim.claim_id.clone(),
        kind: "claim".to_string(),
        ready: checks.iter().all(|item| item.passed),
        checks,
    })
}

pub fn check_hypothesis_promotion(root: &Path, hypothesis_id: &str) -> Result<PromotionCheck> {
    let hypotheses = read_csv::<HypothesisRecord>(&root.join("data/hypotheses.csv"))?;
    let sources = read_csv::<SourceRecord>(&root.join("data/source-registry.csv"))?;
    let hypothesis = hypotheses
        .iter()
        .find(|hypothesis| hypothesis.hypothesis_id == hypothesis_id)
        .with_context(|| format!("hypothesis not found: {hypothesis_id}"))?;
    let source_ids: std::collections::HashSet<&str> = sources
        .iter()
        .map(|source| source.source_id.as_str())
        .collect();

    let checks = vec![
        check(
            "has evidence",
            !hypothesis.evidence.trim().is_empty(),
            "hypothesis must describe supporting evidence",
        ),
        check(
            "evidence cites known source",
            source_ids
                .iter()
                .any(|source_id| hypothesis.evidence.contains(*source_id)),
            "hypothesis evidence must cite at least one known source id",
        ),
        check(
            "has test",
            !hypothesis.test.trim().is_empty(),
            "hypothesis must include a falsifiable test",
        ),
        check(
            "confidence is not Low",
            !hypothesis.confidence.eq_ignore_ascii_case("Low"),
            "promotion requires Medium or High confidence",
        ),
    ];

    Ok(PromotionCheck {
        id: hypothesis.hypothesis_id.clone(),
        kind: "hypothesis".to_string(),
        ready: checks.iter().all(|item| item.passed),
        checks,
    })
}

pub fn print_promotion_check(check: &PromotionCheck, json: bool) -> Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(check)?);
    } else {
        println!("{} {}\tready={}", check.kind, check.id, check.ready);
        for item in &check.checks {
            println!(
                "{}\t{}\t{}",
                if item.passed { "ok" } else { "fail" },
                item.name,
                item.detail
            );
        }
    }

    if check.ready {
        Ok(())
    } else {
        bail!("promotion gate failed")
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

fn refs_exist(refs: &str, known: &std::collections::HashSet<&str>) -> bool {
    let refs: Vec<&str> = refs
        .split(';')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .collect();
    !refs.is_empty() && refs.iter().all(|value| known.contains(value))
}

fn check(name: &str, passed: bool, detail: &str) -> PromotionCheckItem {
    PromotionCheckItem {
        name: name.to_string(),
        passed,
        detail: detail.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn claim_promotion_gate_accepts_active_medium_claim() {
        let check = check_claim_promotion(Path::new(env!("CARGO_MANIFEST_DIR")), "C-003").unwrap();
        assert!(check.ready);
    }

    #[test]
    fn hypothesis_promotion_gate_accepts_seed_hypothesis() {
        let check =
            check_hypothesis_promotion(Path::new(env!("CARGO_MANIFEST_DIR")), "H-002").unwrap();
        assert!(check.ready);
    }
}
