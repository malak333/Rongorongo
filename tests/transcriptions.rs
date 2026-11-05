//! Transcription register rules and derived analysis, against the fixture.

mod common;

use common::Fixture;
use predicates::prelude::*;

#[test]
fn transcriptions_are_optional() {
    let fixture = Fixture::new();
    fixture.remove("data/transcriptions/lines.csv");
    fixture.remove("data/transcriptions/LICENSE.md");
    fixture.run(&["audit", "--strict"]).success();
    fixture
        .run(&["analyze", "frequency"])
        .failure()
        .stderr(predicate::str::contains("no transcription lines found"));
}

#[test]
fn transcriptions_require_a_license_notice() {
    let fixture = Fixture::new();
    fixture.remove("data/transcriptions/LICENSE.md");
    fixture
        .run(&["validate", "--strict"])
        .failure()
        .stdout(predicate::str::contains(
            "data/transcriptions/LICENSE.md is missing",
        ));
}

#[test]
fn transcription_rows_require_a_license_value() {
    let fixture = Fixture::new();
    fixture.write(
        "data/transcriptions/lines.csv",
        &common::TRANSCRIPTIONS.replace("SRC-001,CC-BY-NC-4.0,]1", "SRC-001,,]1"),
    );
    fixture
        .run(&["validate", "--strict"])
        .failure()
        .stdout(predicate::str::contains("row 2 has empty license"));
}

#[test]
fn audit_rejects_transcription_side_not_on_the_object() {
    let fixture = Fixture::new();
    fixture.write(
        "data/transcriptions/lines.csv",
        &common::TRANSCRIPTIONS.replace("Db1,D,Db", "Dc1,D,Dc"),
    );
    fixture
        .run(&["audit", "--strict"])
        .failure()
        .stdout(predicate::str::contains(
            "transcription Dc1 side=Dc is not one of D's sides (Da; Db)",
        ));
}

#[test]
fn audit_rejects_line_id_that_does_not_match_side() {
    let fixture = Fixture::new();
    fixture.write(
        "data/transcriptions/lines.csv",
        &common::TRANSCRIPTIONS.replace("Db1,D,Db", "Line1,D,Db"),
    );
    fixture
        .run(&["audit", "--strict"])
        .failure()
        .stdout(predicate::str::contains(
            "line_id must be the side followed by a line number",
        ));
}

#[test]
fn audit_rejects_unknown_transcription_source() {
    let fixture = Fixture::new();
    fixture.write(
        "data/transcriptions/lines.csv",
        &common::TRANSCRIPTIONS.replace("Da1,D,Da,SRC-001", "Da1,D,Da,SRC-404"),
    );
    fixture
        .run(&["audit", "--strict"])
        .failure()
        .stdout(predicate::str::contains(
            "transcription Da1 source_ref references unknown id SRC-404",
        ));
}

#[test]
fn audit_rejects_duplicate_line_ids() {
    let fixture = Fixture::new();
    fixture.write(
        "data/transcriptions/lines.csv",
        &common::TRANSCRIPTIONS.replace("Db1,D,Db", "Da1,D,Da"),
    );
    fixture
        .run(&["validate", "--strict"])
        .failure()
        .stdout(predicate::str::contains("duplicates line_id=Da1"));
}

#[test]
fn frequency_separates_certain_possible_and_unparsed() {
    let fixture = Fixture::new();
    fixture
        .run(&["analyze", "frequency"])
        .success()
        .stdout(predicate::str::contains("1\t3\t0\tDa1,Db1"))
        .stdout(predicate::str::contains("2\t3\t1\tDa1,Db1"))
        .stdout(predicate::str::contains("600\t1\t1\tDb1"))
        .stdout(predicate::str::contains("excluded: Db1.4 3•6"))
        .stdout(predicate::str::contains("CC BY-NC 4.0"));
}

#[test]
fn sequences_report_maximal_repeats_with_locations() {
    let fixture = Fixture::new();
    let assert = fixture
        .run(&["analyze", "sequences", "--format", "json"])
        .success()
        .stderr(predicate::str::contains("CC BY-NC 4.0"));
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
    let rows = parsed.as_array().unwrap();
    // 1–2–3 repeats twice in Da1. 1–2 is not absorbed by it, because it
    // also occurs in Db1 where 3 does not follow.
    assert_eq!(rows.len(), 2, "{rows:?}");
    assert_eq!(rows[0]["sequence"], "1–2–3");
    assert_eq!(
        rows[0]["locations"],
        serde_json::json!(["Da1.1-3", "Da1.5-7"])
    );
    assert_eq!(rows[1]["sequence"], "1–2");
    assert_eq!(rows[1]["count"], 3);

    fixture
        .run(&["analyze", "sequences", "--min-length", "3"])
        .success()
        .stdout(predicate::str::contains("1–2–3\t3\t2"))
        .stdout(predicate::str::contains("1–2\t2").not());
}

#[test]
fn elements_problems_lists_only_flagged_elements() {
    let fixture = Fixture::new();
    fixture
        .run(&["transcriptions", "elements", "--problems"])
        .success()
        .stdout(predicate::str::contains("Da1\t4\t•\tuncertain"))
        .stdout(predicate::str::contains("Db1\t2\t400/600\tambiguous"))
        .stdout(predicate::str::contains("Db1\t4\t3•6\tunparsed"))
        .stdout(predicate::str::contains("Da1\t1\t").not());
}
