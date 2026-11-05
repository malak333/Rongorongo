#![allow(dead_code)]

use assert_cmd::Command;
use std::path::Path;
use tempfile::TempDir;

/// A small, internally consistent workspace that passes `audit --strict`.
/// Tests break it in one specific way and assert on the resulting message,
/// so they do not depend on the live research data.
pub struct Fixture {
    dir: TempDir,
}

pub const CORPUS: &str = "object_name,catalog_id,current_location,sides,transcription_source,source_reliability,inclusion_confidence,reference_url,notes\nFixture tablet,D,Rome,Da; Db,SRC-001,High,High,https://example.invalid,fixture\n";
pub const SOURCES: &str = "source_id,citation,year,source_type,contribution,limits,reliability,access,url,accessed,notes\nSRC-001,Fixture paper,2022,peer-reviewed-open-access,Fixture,Fixture,High,open-access,https://example.invalid,2026-05-19,fixture\nSRC-002,Fixture overview,,expert-secondary-web,Fixture,Fixture,Low,public-web,https://example.invalid,2026-05-19,fixture\n";
pub const HYPOTHESES: &str = "hypothesis_id,claim,evidence,evidence_refs,corpus_refs,observation_refs,test,status,confidence,notes\nH-001,Fixture,Fixture evidence,SRC-001,D,O-001,Fixture test,active,Medium,fixture\n";
pub const CLAIMS: &str = "claim_id,claim,claim_type,evidence_refs,corpus_refs,confidence,status,notes\nC-001,Fixture,method,SRC-001,D,High,promoted,fixture\n";
pub const OBSERVATIONS: &str = "observation_id,corpus_id,source_refs,observation,reading_order_assumption,confidence,status,notes\nO-001,D,SRC-001,Fixture,Reverse boustrophedon,Medium,promoted,fixture\n";
pub const SEQUENCES: &str = "sequence_id,sequence,corpus_refs,source_refs,observation_refs,position_pattern,possible_function,confidence,status,notes\nS-001,none-recorded,D,SRC-001,O-001,fixture,fixture,Low,draft,fixture\n";
pub const READINGS: &str = "reading_id,sign_or_sequence,proposed_meaning,evidence_refs,corpus_refs,observation_refs,alternative_explanations,confidence,status,notes\nR-001,none,none-proposed,SRC-001,corpus-wide,O-001,Structural explanations remain prior,Low,draft,fixture\n";

pub const TRANSCRIPTIONS: &str = "line_id,corpus_id,side,source_ref,license,raw_text,marked_text,notes\nDa1,D,Da,SRC-001,CC-BY-NC-4.0,]1–2–3–•–1–2–3[,]1–2–3–•–1–2–3[,fixture\nDb1,D,Db,SRC-001,CC-BY-NC-4.0,]600–400/600–2••–3•6–1–2–9–8[,]600–400/600–2••–3•6–1–2–{9}–8[,fixture\n";
pub const TRANSCRIPTIONS_LICENSE: &str = "# Fixture license\n";

pub const DOSSIER: &str = "\
# Dossier

| Source | Reliability |
|---|---|
| SRC-001: Fixture paper | High |
| SRC-002: Fixture overview | Low |

| Claim ID | Claim | Confidence |
|---|---|---|
| C-001 | Fixture | High |
";

pub const NOTEBOOK: &str = "\
# Notebook

| Observation ID | Observation | Confidence |
|---|---|---|
| O-001 | Fixture | Medium |

| Sequence ID | Sequence | Confidence |
|---|---|---|
| S-001 | none-recorded | Low |

| Hypothesis ID | Hypothesis | Confidence |
|---|---|---|
| H-001 | Fixture | Medium |

| Reading ID | Sign / Sequence | Confidence |
|---|---|---|
| R-001 | none | Low |
";

impl Fixture {
    pub fn new() -> Self {
        let dir = tempfile::tempdir().unwrap();
        let fixture = Self { dir };
        fixture.write("README.md", "README\n");
        fixture.write("research-dossier.md", DOSSIER);
        fixture.write("decipherment-notebook.md", NOTEBOOK);
        fixture.write("data/corpus-index.csv", CORPUS);
        fixture.write("data/source-registry.csv", SOURCES);
        fixture.write("data/hypotheses.csv", HYPOTHESES);
        fixture.write("data/claims.csv", CLAIMS);
        fixture.write("data/observations.csv", OBSERVATIONS);
        fixture.write("data/sequences.csv", SEQUENCES);
        fixture.write("data/readings.csv", READINGS);
        fixture.write("data/transcriptions/lines.csv", TRANSCRIPTIONS);
        fixture.write("data/transcriptions/LICENSE.md", TRANSCRIPTIONS_LICENSE);
        fixture
    }

    pub fn root(&self) -> &Path {
        self.dir.path()
    }

    pub fn root_str(&self) -> &str {
        self.dir.path().to_str().unwrap()
    }

    pub fn write(&self, relative: &str, contents: &str) {
        let path = self.dir.path().join(relative);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, contents).unwrap();
    }

    pub fn append(&self, relative: &str, contents: &str) {
        let path = self.dir.path().join(relative);
        let mut existing = std::fs::read_to_string(&path).unwrap();
        existing.push_str(contents);
        std::fs::write(path, existing).unwrap();
    }

    pub fn remove(&self, relative: &str) {
        std::fs::remove_file(self.dir.path().join(relative)).unwrap();
    }

    /// Run the CLI with `--root <fixture>` appended.
    pub fn run(&self, args: &[&str]) -> assert_cmd::assert::Assert {
        let mut command = Command::cargo_bin("rongorongo").unwrap();
        command.args(args).args(["--root", self.root_str()]);
        command.assert()
    }
}
