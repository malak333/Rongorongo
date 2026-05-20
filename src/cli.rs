use crate::audit::AuditReport;
use crate::model::{
    ClaimRecord, CorpusObject, HypothesisRecord, ObservationRecord, OutputFormat, SourceRecord,
};
use crate::validate::{self, ValidationReport};
use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand};
use serde::Serialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Parser)]
#[command(name = "rongorongo")]
#[command(about = "Audit-ready research tooling for the Rongorongo workspace")]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Run the full research-governance audit, including cross-reference checks.
    Audit(AuditArgs),
    /// Validate required files and supported CSV schemas.
    Validate(ValidateArgs),
    /// Work with the canonical corpus index.
    Corpus(CorpusCommand),
    /// Work with the source registry.
    Sources(SourceCommand),
    /// Work with the decipherment hypothesis register.
    Hypotheses(HypothesisCommand),
    /// Work with the evidence claim register.
    Claims(ClaimCommand),
    /// Work with corpus observation records.
    Observations(ObservationCommand),
    /// Generate public-safe intake templates.
    Intake(IntakeCommand),
    /// Check whether claims or hypotheses satisfy the promotion gate.
    Promote(PromoteCommand),
}

#[derive(Debug, Args)]
struct RootArgs {
    /// Repository root. Defaults to the current directory.
    #[arg(long, default_value = ".")]
    root: PathBuf,
}

#[derive(Debug, Args)]
struct ValidateArgs {
    /// Repository root. Defaults to the current directory.
    #[arg(long, default_value = ".")]
    root: PathBuf,
    /// Fail on placeholders and warnings. This is intended for CI and release gates.
    #[arg(long)]
    strict: bool,
}

#[derive(Debug, Args)]
struct AuditArgs {
    /// Repository root. Defaults to the current directory.
    #[arg(long, default_value = ".")]
    root: PathBuf,
    /// Fail on placeholders and warnings. This is intended for CI and release gates.
    #[arg(long)]
    strict: bool,
    #[command(flatten)]
    format: FormatArgs,
}

#[derive(Debug, Args)]
struct FormatArgs {
    /// Output format: table or json.
    #[arg(long, default_value = "table")]
    format: OutputFormat,
}

#[derive(Debug, Args)]
struct CorpusCommand {
    #[command(subcommand)]
    command: CorpusSubcommand,
}

#[derive(Debug, Subcommand)]
enum CorpusSubcommand {
    /// List canonical corpus rows from data/corpus-index.csv.
    List(ListArgs),
}

#[derive(Debug, Args)]
struct SourceCommand {
    #[command(subcommand)]
    command: SourceSubcommand,
}

#[derive(Debug, Subcommand)]
enum SourceSubcommand {
    /// List source rows from data/source-registry.csv.
    List(ListArgs),
}

#[derive(Debug, Args)]
struct HypothesisCommand {
    #[command(subcommand)]
    command: HypothesisSubcommand,
}

#[derive(Debug, Subcommand)]
enum HypothesisSubcommand {
    /// List hypothesis rows from data/hypotheses.csv.
    List(ListArgs),
}

#[derive(Debug, Args)]
struct ClaimCommand {
    #[command(subcommand)]
    command: ClaimSubcommand,
}

#[derive(Debug, Subcommand)]
enum ClaimSubcommand {
    /// List evidence claims from data/claims.csv.
    List(ListArgs),
}

#[derive(Debug, Args)]
struct ObservationCommand {
    #[command(subcommand)]
    command: ObservationSubcommand,
}

#[derive(Debug, Subcommand)]
enum ObservationSubcommand {
    /// List corpus observations from data/observations.csv.
    List(ListArgs),
}

#[derive(Debug, Args)]
struct IntakeCommand {
    #[command(subcommand)]
    command: IntakeSubcommand,
}

#[derive(Debug, Subcommand)]
enum IntakeSubcommand {
    /// Print a source-registry CSV row template.
    Source(SourceTemplateArgs),
}

#[derive(Debug, Args)]
struct SourceTemplateArgs {
    /// Suggested next source id, e.g. SRC-006.
    #[arg(long, default_value = "SRC-NEW")]
    next_id: String,
}

#[derive(Debug, Args)]
struct PromoteCommand {
    #[command(subcommand)]
    command: PromoteSubcommand,
}

#[derive(Debug, Subcommand)]
enum PromoteSubcommand {
    /// Check whether a claim is ready to promote into the dossier.
    Claim(PromoteArgs),
    /// Check whether a hypothesis is ready to promote into the dossier.
    Hypothesis(PromoteArgs),
}

#[derive(Debug, Args)]
struct PromoteArgs {
    /// Repository root. Defaults to the current directory.
    #[arg(long, default_value = ".")]
    root: PathBuf,
    /// Claim or hypothesis id.
    id: String,
    /// Emit JSON instead of a table.
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Args)]
struct ListArgs {
    /// Repository root. Defaults to the current directory.
    #[arg(long, default_value = ".")]
    root: PathBuf,
    #[command(flatten)]
    format: FormatArgs,
}

impl Cli {
    pub fn run(self) -> Result<()> {
        match self.command {
            Command::Audit(args) => print_audit(
                crate::audit::audit_workspace(&args.root, args.strict)?,
                args.format.format,
            ),
            Command::Validate(args) => {
                print_validation(validate::validate_workspace(&args.root, args.strict)?)
            }
            Command::Corpus(command) => match command.command {
                CorpusSubcommand::List(args) => {
                    let rows = read_csv::<CorpusObject>(&args.root.join("data/corpus-index.csv"))?;
                    print_rows(&rows, args.format.format, corpus_columns)
                }
            },
            Command::Sources(command) => match command.command {
                SourceSubcommand::List(args) => {
                    let rows =
                        read_csv::<SourceRecord>(&args.root.join("data/source-registry.csv"))?;
                    print_rows(&rows, args.format.format, source_columns)
                }
            },
            Command::Hypotheses(command) => match command.command {
                HypothesisSubcommand::List(args) => {
                    let rows =
                        read_csv::<HypothesisRecord>(&args.root.join("data/hypotheses.csv"))?;
                    print_rows(&rows, args.format.format, hypothesis_columns)
                }
            },
            Command::Claims(command) => match command.command {
                ClaimSubcommand::List(args) => {
                    let rows = read_csv::<ClaimRecord>(&args.root.join("data/claims.csv"))?;
                    print_rows(&rows, args.format.format, claim_columns)
                }
            },
            Command::Observations(command) => match command.command {
                ObservationSubcommand::List(args) => {
                    let rows =
                        read_csv::<ObservationRecord>(&args.root.join("data/observations.csv"))?;
                    print_rows(&rows, args.format.format, observation_columns)
                }
            },
            Command::Intake(command) => match command.command {
                IntakeSubcommand::Source(args) => {
                    print!("{}", crate::workflow::source_template(&args.next_id));
                    Ok(())
                }
            },
            Command::Promote(command) => match command.command {
                PromoteSubcommand::Claim(args) => {
                    let check = crate::workflow::check_claim_promotion(&args.root, &args.id)?;
                    crate::workflow::print_promotion_check(&check, args.json)
                }
                PromoteSubcommand::Hypothesis(args) => {
                    let check = crate::workflow::check_hypothesis_promotion(&args.root, &args.id)?;
                    crate::workflow::print_promotion_check(&check, args.json)
                }
            },
        }
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

fn print_validation(report: ValidationReport) -> Result<()> {
    for message in &report.messages {
        println!("[{}] {}", message.level, message.message);
    }

    println!(
        "validated {} files with {} warning(s) and {} error(s)",
        report.checked_files, report.warning_count, report.error_count
    );

    if report.error_count == 0 {
        Ok(())
    } else {
        anyhow::bail!("workspace validation failed")
    }
}

fn print_audit(report: AuditReport, format: OutputFormat) -> Result<()> {
    match format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&report.summary)?);
        }
        OutputFormat::Table => {
            println!("Audit summary");
            println!("corpus_objects\t{}", report.summary.corpus_objects);
            println!("sources\t{}", report.summary.sources);
            println!("claims\t{}", report.summary.claims);
            println!("observations\t{}", report.summary.observations);
            println!("hypotheses\t{}", report.summary.hypotheses);
            println!("promoted_claims\t{}", report.summary.promoted_claims);
            println!("active_hypotheses\t{}", report.summary.active_hypotheses);
        }
    }

    println!(
        "validated {} files with {} warning(s) and {} error(s)",
        report.validation.checked_files,
        report.validation.warning_count,
        report.validation.error_count
    );

    if report.validation.error_count == 0 {
        Ok(())
    } else {
        anyhow::bail!("workspace audit failed")
    }
}

fn print_rows<T, F>(rows: &[T], format: OutputFormat, columns: F) -> Result<()>
where
    T: Serialize,
    F: Fn(&T) -> Vec<(&'static str, String)>,
{
    match format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(rows)?);
            Ok(())
        }
        OutputFormat::Table => {
            if rows.is_empty() {
                println!("No rows found.");
                return Ok(());
            }

            let headers: Vec<&'static str> =
                columns(&rows[0]).into_iter().map(|(key, _)| key).collect();
            println!("{}", headers.join("\t"));
            for row in rows {
                let values: Vec<String> =
                    columns(row).into_iter().map(|(_, value)| value).collect();
                println!("{}", values.join("\t"));
            }
            Ok(())
        }
    }
}

fn corpus_columns(row: &CorpusObject) -> Vec<(&'static str, String)> {
    vec![
        ("object", row.object_name.clone()),
        ("catalog", row.catalog_id.clone()),
        ("location", row.current_location.clone()),
        ("confidence", row.inclusion_confidence.clone()),
        ("source", row.transcription_source.clone()),
    ]
}

fn source_columns(row: &SourceRecord) -> Vec<(&'static str, String)> {
    vec![
        ("source_id", row.source_id.clone()),
        ("citation", row.citation.clone()),
        ("year", row.year.clone()),
        ("type", row.source_type.clone()),
        ("reliability", row.reliability.clone()),
    ]
}

fn hypothesis_columns(row: &HypothesisRecord) -> Vec<(&'static str, String)> {
    vec![
        ("hypothesis_id", row.hypothesis_id.clone()),
        ("claim", row.claim.clone()),
        ("status", row.status.clone()),
        ("confidence", row.confidence.clone()),
    ]
}

fn claim_columns(row: &ClaimRecord) -> Vec<(&'static str, String)> {
    vec![
        ("claim_id", row.claim_id.clone()),
        ("claim", row.claim.clone()),
        ("type", row.claim_type.clone()),
        ("status", row.status.clone()),
        ("confidence", row.confidence.clone()),
    ]
}

fn observation_columns(row: &ObservationRecord) -> Vec<(&'static str, String)> {
    vec![
        ("observation_id", row.observation_id.clone()),
        ("corpus_id", row.corpus_id.clone()),
        ("status", row.status.clone()),
        ("confidence", row.confidence.clone()),
        ("source_refs", row.source_refs.clone()),
    ]
}
