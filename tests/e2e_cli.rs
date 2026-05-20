use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn e2e_strict_workspace_validation_passes() {
    let mut command = Command::cargo_bin("rongorongo").unwrap();
    command
        .args(["validate", "--strict"])
        .assert()
        .success()
        .stdout(predicate::str::contains("0 warning(s) and 0 error(s)"));
}

#[test]
fn e2e_lists_seed_corpus_source_and_hypothesis() {
    let mut corpus = Command::cargo_bin("rongorongo").unwrap();
    corpus
        .args(["corpus", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Echancree tablet"));

    let mut sources = Command::cargo_bin("rongorongo").unwrap();
    sources
        .args(["sources", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("SRC-003"));

    let mut hypotheses = Command::cargo_bin("rongorongo").unwrap();
    hypotheses
        .args(["hypotheses", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("H-001"));

    let mut claims = Command::cargo_bin("rongorongo").unwrap();
    claims
        .args(["claims", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("C-001"));

    let mut observations = Command::cargo_bin("rongorongo").unwrap();
    observations
        .args(["observations", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("O-001"));

    let mut sequences = Command::cargo_bin("rongorongo").unwrap();
    sequences
        .args(["sequences", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("S-001"));

    let mut readings = Command::cargo_bin("rongorongo").unwrap();
    readings
        .args(["readings", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("R-001"));
}

#[test]
fn e2e_json_output_is_available_for_ci_consumers() {
    for (command_name, key) in [
        ("corpus", "object_name"),
        ("sources", "source_id"),
        ("hypotheses", "hypothesis_id"),
        ("claims", "claim_id"),
        ("observations", "observation_id"),
        ("sequences", "sequence_id"),
        ("readings", "reading_id"),
    ] {
        let mut command = Command::cargo_bin("rongorongo").unwrap();
        let assert = command
            .args([command_name, "list", "--format", "json"])
            .assert()
            .success();
        let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
        assert!(parsed.as_array().unwrap()[0].get(key).is_some());
    }
}

#[test]
fn e2e_audit_reports_cross_reference_summary() {
    let mut command = Command::cargo_bin("rongorongo").unwrap();
    command
        .args(["audit", "--strict"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Audit summary"))
        .stdout(predicate::str::contains("promoted_claims"));
}

#[test]
fn e2e_intake_template_and_promotion_gates_work() {
    let mut intake = Command::cargo_bin("rongorongo").unwrap();
    intake
        .args(["intake", "source", "--next-id", "SRC-006"])
        .assert()
        .success()
        .stdout(predicate::str::contains("SRC-006"))
        .stdout(predicate::str::contains("source_id,citation"));

    let mut claim = Command::cargo_bin("rongorongo").unwrap();
    claim
        .args(["promote", "claim", "C-003"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ready=true"));

    let mut hypothesis = Command::cargo_bin("rongorongo").unwrap();
    hypothesis
        .args(["promote", "hypothesis", "H-002", "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"ready\": true"));
}
