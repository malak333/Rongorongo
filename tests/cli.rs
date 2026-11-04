//! Rule tests against a temporary fixture workspace. Each test breaks the
//! fixture in one way and checks that the CLI reports it.

mod common;

use common::Fixture;
use predicates::prelude::*;

fn audit_fails_with(fixture: &Fixture, expected: &str) {
    fixture
        .run(&["audit", "--strict"])
        .failure()
        .stdout(predicate::str::contains(expected));
}

#[test]
fn fixture_passes_strict_validate_and_audit() {
    let fixture = Fixture::new();
    fixture
        .run(&["validate", "--strict"])
        .success()
        .stdout(predicate::str::contains("0 warning(s) and 0 error(s)"));
    fixture
        .run(&["audit", "--strict"])
        .success()
        .stdout(predicate::str::contains("0 warning(s) and 0 error(s)"));
}

#[test]
fn strict_validate_rejects_template_placeholders() {
    let fixture = Fixture::new();
    fixture.write(
        "data/corpus-index.csv",
        &common::CORPUS.replace("Fixture tablet", "TBD"),
    );
    fixture
        .run(&["validate", "--strict"])
        .failure()
        .stdout(predicate::str::contains("TBD placeholder"));
}

#[test]
fn non_strict_validate_only_warns_on_placeholders() {
    let fixture = Fixture::new();
    fixture.write("README.md", "TBD\n");
    fixture
        .run(&["validate"])
        .success()
        .stdout(predicate::str::contains("1 warning(s) and 0 error(s)"));
}

#[test]
fn validate_rejects_unknown_status() {
    let fixture = Fixture::new();
    fixture.write(
        "data/claims.csv",
        &common::CLAIMS.replace("promoted", "banana"),
    );
    fixture
        .run(&["validate", "--strict"])
        .failure()
        .stdout(predicate::str::contains("invalid status=banana"));
}

#[test]
fn validate_rejects_mixed_as_row_confidence() {
    let fixture = Fixture::new();
    fixture.write(
        "data/hypotheses.csv",
        &common::HYPOTHESES.replace("active,Medium", "active,Mixed"),
    );
    fixture
        .run(&["validate", "--strict"])
        .failure()
        .stdout(predicate::str::contains("invalid confidence=Mixed"));
}

#[test]
fn validate_requires_reading_order_assumption_on_observations() {
    let fixture = Fixture::new();
    fixture.write(
        "data/observations.csv",
        &common::OBSERVATIONS.replace("Reverse boustrophedon", ""),
    );
    fixture
        .run(&["validate", "--strict"])
        .failure()
        .stdout(predicate::str::contains("empty reading_order_assumption"));
}

#[test]
fn missing_headers_are_reported_without_aborting_other_checks() {
    let fixture = Fixture::new();
    fixture.write(
        "data/corpus-index.csv",
        "object_name,catalog_id\nFixture,D\n",
    );
    fixture.write(
        "data/claims.csv",
        &common::CLAIMS.replace("promoted", "banana"),
    );
    fixture
        .run(&["validate", "--strict"])
        .failure()
        .stdout(predicate::str::contains(
            "data/corpus-index.csv is missing required header(s): current_location",
        ))
        .stdout(predicate::str::contains("invalid status=banana"));
    fixture
        .run(&["audit", "--strict"])
        .failure()
        .stdout(predicate::str::contains("cross-reference audit skipped"));
}

#[test]
fn missing_register_is_a_validation_error() {
    let fixture = Fixture::new();
    fixture.remove("data/source-registry.csv");
    fixture
        .run(&["validate", "--strict"])
        .failure()
        .stdout(predicate::str::contains(
            "missing required file data/source-registry.csv",
        ));
    fixture
        .run(&["sources", "list"])
        .failure()
        .stderr(predicate::str::contains("data/source-registry.csv"));
}

#[test]
fn audit_rejects_unknown_cross_reference() {
    let fixture = Fixture::new();
    fixture.write(
        "data/claims.csv",
        &common::CLAIMS.replace("SRC-001", "SRC-999"),
    );
    audit_fails_with(&fixture, "references unknown id SRC-999");
}

#[test]
fn audit_caps_confidence_at_strongest_cited_source() {
    let fixture = Fixture::new();
    fixture.write(
        "data/claims.csv",
        &common::CLAIMS.replace("SRC-001,D,High", "SRC-002,D,High"),
    );
    audit_fails_with(
        &fixture,
        "claim C-001 confidence=High exceeds the strongest cited source reliability (Low)",
    );
}

#[test]
fn audit_allows_a_weak_source_alongside_a_strong_one() {
    let fixture = Fixture::new();
    fixture.write(
        "data/claims.csv",
        &common::CLAIMS.replace("SRC-001,D,High", "SRC-001;SRC-002,D,High"),
    );
    fixture.run(&["audit", "--strict"]).success();
}

#[test]
fn audit_rejects_corpus_reliability_that_disagrees_with_source() {
    let fixture = Fixture::new();
    fixture.write(
        "data/corpus-index.csv",
        &common::CORPUS.replace("SRC-001,High,High", "SRC-001,Medium,Medium"),
    );
    audit_fails_with(
        &fixture,
        "corpus D source_reliability=Medium but SRC-001 has reliability=High",
    );
}

#[test]
fn audit_applies_promotion_gate_to_rows_already_marked_promoted() {
    let fixture = Fixture::new();
    fixture.write(
        "data/claims.csv",
        &common::CLAIMS.replace("High,promoted", "Low,promoted"),
    );
    fixture.write(
        "research-dossier.md",
        &common::DOSSIER.replace("| Fixture | High |", "| Fixture | Low |"),
    );
    audit_fails_with(
        &fixture,
        "claim C-001 is marked promoted but fails promotion gate 'confidence is not Low'",
    );
}

#[test]
fn audit_applies_gate_to_accepted_hypotheses() {
    let fixture = Fixture::new();
    fixture.write(
        "data/hypotheses.csv",
        &common::HYPOTHESES.replace("active,Medium", "accepted-working-rule,Low"),
    );
    fixture.write(
        "decipherment-notebook.md",
        &common::NOTEBOOK.replace("| H-001 | Fixture | Medium |", "| H-001 | Fixture | Low |"),
    );
    audit_fails_with(
        &fixture,
        "hypothesis H-001 is marked promoted but fails promotion gate 'confidence is not Low'",
    );
}

#[test]
fn audit_rejects_semantic_reading_without_promoted_observation() {
    let fixture = Fixture::new();
    fixture.write(
        "data/observations.csv",
        &common::OBSERVATIONS.replace("Medium,promoted", "Medium,active"),
    );
    fixture.append(
        "data/readings.csv",
        "R-002,522,the moon,SRC-001,D,O-001,Could be a name,Low,draft,fixture\n",
    );
    fixture.append(
        "decipherment-notebook.md",
        "\n| Reading ID | Sign | Confidence |\n|---|---|---|\n| R-002 | 522 | Low |\n",
    );
    audit_fails_with(
        &fixture,
        "reading R-002 proposes a meaning but cites observation O-001 with status=active",
    );
}

#[test]
fn audit_rejects_reading_more_confident_than_its_observation() {
    let fixture = Fixture::new();
    fixture.append(
        "data/readings.csv",
        "R-002,522,the moon,SRC-001,D,O-001,Could be a name,High,draft,fixture\n",
    );
    fixture.append(
        "decipherment-notebook.md",
        "\n| Reading ID | Sign | Confidence |\n|---|---|---|\n| R-002 | 522 | High |\n",
    );
    audit_fails_with(
        &fixture,
        "reading R-002 confidence=High exceeds the confidence of observation O-001 (Medium)",
    );
}

#[test]
fn audit_rejects_markdown_confidence_drift() {
    let fixture = Fixture::new();
    fixture.write(
        "research-dossier.md",
        &common::DOSSIER.replace("| C-001 | Fixture | High |", "| C-001 | Fixture | Medium |"),
    );
    audit_fails_with(
        &fixture,
        "shows C-001 confidence=Medium but the CSV has confidence=High",
    );
}

#[test]
fn audit_rejects_markdown_sequence_drift() {
    let fixture = Fixture::new();
    fixture.write(
        "decipherment-notebook.md",
        &common::NOTEBOOK.replace("| S-001 | none-recorded |", "| S-001 | 522-99 pattern |"),
    );
    audit_fails_with(
        &fixture,
        "shows S-001 sequence=522-99 pattern but the CSV has sequence=none-recorded",
    );
}

#[test]
fn audit_rejects_register_id_missing_from_markdown() {
    let fixture = Fixture::new();
    fixture.write(
        "decipherment-notebook.md",
        &common::NOTEBOOK.replace("| O-001 | Fixture | Medium |\n", ""),
    );
    audit_fails_with(
        &fixture,
        "O-001 is in the CSV registers but not in any table",
    );
}

#[test]
fn audit_rejects_markdown_id_missing_from_registers() {
    let fixture = Fixture::new();
    fixture.append("research-dossier.md", "| C-009 | Ghost | Low |\n");
    audit_fails_with(&fixture, "mentions C-009 which is not in any CSV register");
}

#[test]
fn audit_json_keeps_stdout_parseable_on_failure() {
    let fixture = Fixture::new();
    fixture.write(
        "data/claims.csv",
        &common::CLAIMS.replace("SRC-001", "SRC-999"),
    );
    let assert = fixture
        .run(&["audit", "--strict", "--format", "json"])
        .failure()
        .stderr(predicate::str::contains("unknown id SRC-999"));
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
    assert_eq!(parsed["claims"], 1);
}

#[test]
fn promotion_gate_rejects_low_confidence_claim() {
    let fixture = Fixture::new();
    fixture.write(
        "data/claims.csv",
        &common::CLAIMS.replace("High,promoted", "Low,active"),
    );
    fixture
        .run(&["promote", "claim", "C-001"])
        .failure()
        .stdout(predicate::str::contains("fail\tconfidence is not Low"));
}

#[test]
fn promotion_gate_rejects_invalid_confidence_and_unknown_source_ref() {
    let fixture = Fixture::new();
    fixture.write(
        "data/hypotheses.csv",
        &common::HYPOTHESES
            .replace("SRC-001", "SRC-0010")
            .replace("active,Medium", "active,Nonsense"),
    );
    fixture
        .run(&["promote", "hypothesis", "H-001"])
        .failure()
        .stdout(predicate::str::contains(
            "fail\tevidence cites known source",
        ))
        .stdout(predicate::str::contains("fail\tconfidence is not Low"));
}

#[test]
fn promotion_gate_accepts_ready_items() {
    let fixture = Fixture::new();
    fixture
        .run(&["promote", "claim", "C-001"])
        .success()
        .stdout(predicate::str::contains("ready=true"));
    fixture
        .run(&["promote", "hypothesis", "H-001", "--json"])
        .success()
        .stdout(predicate::str::contains("\"ready\": true"));
}

#[test]
fn list_commands_read_fixture_rows() {
    let fixture = Fixture::new();
    fixture
        .run(&["corpus", "list"])
        .success()
        .stdout(predicate::str::contains("Fixture tablet"));
}
