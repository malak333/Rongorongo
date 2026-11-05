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
        .args(["promote", "claim", "C-002"])
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

#[test]
fn e2e_text_d_transcription_parses_and_analyzes() {
    let mut list = Command::cargo_bin("rongorongo").unwrap();
    list.args(["transcriptions", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Da1"))
        .stdout(predicate::str::contains("Db6"));

    let mut frequency = Command::cargo_bin("rongorongo").unwrap();
    frequency
        .args(["analyze", "frequency", "--corpus", "Text D / D"])
        .assert()
        .success()
        .stdout(predicate::str::contains("CC BY-NC 4.0"));

    let mut sequences = Command::cargo_bin("rongorongo").unwrap();
    sequences
        .args(["analyze", "sequences"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "sequence\tlength\tcount\tlocations",
        ));
}

/// Every element position Lastilla et al. 2022 cite in sections 4.1-4.3,
/// with the element the parser must find there. Their numbering is the
/// reference: if the parser drifts from it, this test fails.
#[test]
fn e2e_parser_matches_positions_cited_by_the_source() {
    let cited: &[(&str, usize, &str)] = &[
        ("Da1", 7, "381/741"),
        ("Da1", 18, "522f"),
        ("Da1", 19, "99"),
        ("Da2", 10, "441.95x.3"),
        ("Da2", 11, "445.107.1"),
        ("Da2", 15, "173(=5.225/325)"),
        ("Da3", 2, "42:200.41/11"),
        ("Da5", 8, "93"),
        ("Da5", 12, "93"),
        ("Da6", 3, "3.3.52"),
        ("Da6", 4, "50"),
        ("Da6", 9, "50"),
        ("Da6", 14, "•"),
        ("Da6", 15, "7"),
        ("Da6", 16, "60•"),
        ("Db2", 12, "520f"),
        ("Db3", 11, "280.63"),
        ("Db4", 12, "7"),
        ("Db4", 13, "600"),
        ("Db4", 14, "7.10"),
        ("Db4", 15, "•"),
        ("Db5", 1, "522f.700"),
        ("Db5", 3, "200.524f.78"),
        ("Db5", 13, "445"),
        ("Db5", 14, "445"),
        ("Db6", 2, "55"),
        ("Db6", 3, "9"),
        ("Db6", 6, "6/55"),
        ("Db6", 7, "7"),
        ("Db6", 8, "600"),
        ("Db6", 9, "7"),
        ("Db6", 14, "22f"),
    ];

    let mut command = Command::cargo_bin("rongorongo").unwrap();
    let assert = command
        .args(["transcriptions", "elements", "--format", "json"])
        .assert()
        .success();
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    let elements: Vec<serde_json::Value> = serde_json::from_str(&output).unwrap();

    for (line, position, expected) in cited {
        let found = elements
            .iter()
            .find(|e| e["line_id"] == *line && e["position"] == *position)
            .unwrap_or_else(|| panic!("{line}.{position} not found"));
        assert_eq!(found["raw"], *expected, "{line}.{position}");
    }

    // Per-line counts from the numbered drawings (Figs. 16 and 17), except
    // Da7: its printed transcription has 15 elements but Fig. 16 numbers 16.
    let figure_counts = [
        ("Da1", 19),
        ("Da2", 16),
        ("Da3", 15),
        ("Da4", 16),
        ("Da5", 20),
        ("Da6", 16),
        ("Da7", 15),
        ("Da8", 8),
        ("Db1", 15),
        ("Db2", 13),
        ("Db3", 15),
        ("Db4", 15),
        ("Db5", 14),
        ("Db6", 14),
    ];
    for (line, count) in figure_counts {
        let actual = elements.iter().filter(|e| e["line_id"] == line).count();
        assert_eq!(actual, count, "{line}");
    }
    assert_eq!(elements.len(), 211, "212 in the source, less Da7's gap");
}
