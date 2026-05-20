use assert_cmd::Command;
use predicates::prelude::*;
use std::path::Path;

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
    let temp = tempfile::tempdir().unwrap();
    write_minimal_workspace(temp.path(), "TBD");

    let mut command = Command::cargo_bin("rongorongo").unwrap();
    command
        .args([
            "validate",
            "--strict",
            "--root",
            temp.path().to_str().unwrap(),
        ])
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
        .stdout(predicate::str::contains("Echancree tablet"));
}

#[test]
fn missing_optional_source_registry_returns_useful_error() {
    let temp = tempfile::tempdir().unwrap();
    write_minimal_workspace(temp.path(), "Fixture object");

    let mut command = Command::cargo_bin("rongorongo").unwrap();
    command
        .args(["sources", "list", "--root", temp.path().to_str().unwrap()])
        .assert()
        .failure()
        .stderr(predicate::str::contains("data/source-registry.csv"));
}

fn write_minimal_workspace(root: &Path, object_name: &str) {
    std::fs::create_dir_all(root.join("data")).unwrap();
    std::fs::write(root.join("README.md"), "README\n").unwrap();
    std::fs::write(root.join("research-dossier.md"), "Dossier\n").unwrap();
    std::fs::write(root.join("decipherment-notebook.md"), "Notebook\n").unwrap();
    std::fs::write(
        root.join("data/corpus-index.csv"),
        format!(
            "object_name,catalog_id,current_location,sides,transcription_source,source_reliability,inclusion_confidence,notes\n{object_name},D,Rome,Da; Db,SRC-003,High,High,fixture\n"
        ),
    )
    .unwrap();
}
