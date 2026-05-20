use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn validate_command_reports_workspace_status() {
    let mut command = Command::cargo_bin("rongorongo").unwrap();
    command
        .arg("validate")
        .assert()
        .success()
        .stdout(predicate::str::contains("validated"));
}

#[test]
fn strict_validate_rejects_template_placeholders() {
    let mut command = Command::cargo_bin("rongorongo").unwrap();
    command
        .args(["validate", "--strict"])
        .assert()
        .failure()
        .stdout(predicate::str::contains("TBD placeholder"));
}

#[test]
fn corpus_list_outputs_existing_rows() {
    let mut command = Command::cargo_bin("rongorongo").unwrap();
    command
        .args(["corpus", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("object"))
        .stdout(predicate::str::contains("TBD"));
}

#[test]
fn missing_optional_source_registry_returns_useful_error() {
    let mut command = Command::cargo_bin("rongorongo").unwrap();
    command
        .args(["sources", "list"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("data/source-registry.csv"));
}
