use crate::audit::AuditReport;
use crate::model::{
    ClaimRecord, CorpusObject, HypothesisRecord, ObservationRecord, OutputFormat, ReadingRecord,
    SequenceRecord, SourceRecord,
};
use crate::transcription::{self, Element, TranscriptionLine};
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
    /// Run the full research-governance audit: cross-references, confidence caps,
    /// promotion gates on promoted rows, and CSV/Markdown consistency.
    Audit(AuditArgs),
    /// Validate required files, CSV schemas, and controlled vocabularies.
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
    /// Work with repeated sequence records.
    Sequences(SequenceCommand),
    /// Work with tentative semantic reading records.
    Readings(ReadingCommand),
    /// Generate public-safe intake templates.
    Intake(IntakeCommand),
    /// Check whether claims or hypotheses satisfy the promotion gate.
    Promote(PromoteCommand),
    /// Work with verbatim transcription lines in data/transcriptions/lines.csv.
    Transcriptions(TranscriptionCommand),
    /// Derived structural analysis of the transcriptions (counts and repeats, not meanings).
    Analyze(AnalyzeCommand),
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
    /// Fail on placeholders and warnings. This is intended for local release gates.
    #[arg(long)]
    strict: bool,
}

#[derive(Debug, Args)]
struct AuditArgs {
    /// Repository root. Defaults to the current directory.
    #[arg(long, default_value = ".")]
    root: PathBuf,
    /// Fail on placeholders and warnings. This is intended for local release gates.
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
struct SequenceCommand {
    #[command(subcommand)]
    command: SequenceSubcommand,
}

#[derive(Debug, Subcommand)]
enum SequenceSubcommand {
    /// List repeated sequence records from data/sequences.csv.
    List(ListArgs),
}

#[derive(Debug, Args)]
struct ReadingCommand {
    #[command(subcommand)]
    command: ReadingSubcommand,
}

#[derive(Debug, Subcommand)]
enum ReadingSubcommand {
    /// List tentative semantic reading records from data/readings.csv.
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
struct TranscriptionCommand {
    #[command(subcommand)]
    command: TranscriptionSubcommand,
}

#[derive(Debug, Subcommand)]
enum TranscriptionSubcommand {
    /// List transcription lines with their element counts.
    List(ListArgs),
    /// Show how each line parses into graphic elements.
    Elements(ElementsArgs),
}

#[derive(Debug, Args)]
struct ElementsArgs {
    /// Repository root. Defaults to the current directory.
    #[arg(long, default_value = ".")]
    root: PathBuf,
    /// Only show this line, e.g. Da1.
    #[arg(long)]
    line: Option<String>,
    /// Only show elements that are doubtful, uncertain, ambiguous, unparsed, in a
    /// lacuna, or after a soft boundary.
    #[arg(long)]
    problems: bool,
    #[command(flatten)]
    format: FormatArgs,
}

#[derive(Debug, Args)]
struct AnalyzeCommand {
    #[command(subcommand)]
    command: AnalyzeSubcommand,
}

#[derive(Debug, Subcommand)]
enum AnalyzeSubcommand {
    /// Count signs by Barthel base number.
    Frequency(FrequencyArgs),
    /// Find repeated runs of certain elements (or signs) within lines.
    Sequences(SequencesArgs),
    /// Search for a sequence of elements, e.g. "600|610 * 200.52x.*".
    Find(FindArgs),
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum LevelArg {
    /// Compare whole graphic elements.
    Element,
    /// Compare individual signs, ignoring whether they are ligatured.
    Part,
}

#[derive(Debug, Args)]
struct FindArgs {
    /// Repository root. Defaults to the current directory.
    #[arg(long, default_value = ".")]
    root: PathBuf,
    /// Restrict to one corpus object, e.g. "Text D / D".
    #[arg(long)]
    corpus: Option<String>,
    /// Element patterns separated by spaces or dashes; `|` for alternatives,
    /// `*` as a wildcard. A lone `*` matches any one element.
    pattern: String,
    #[command(flatten)]
    format: FormatArgs,
}

#[derive(Debug, Args)]
struct FrequencyArgs {
    /// Repository root. Defaults to the current directory.
    #[arg(long, default_value = ".")]
    root: PathBuf,
    /// Restrict to one corpus object, e.g. "Text D / D".
    #[arg(long)]
    corpus: Option<String>,
    /// Count full codes (522f, 200 vs 200s) instead of base numbers.
    #[arg(long)]
    exact: bool,
    #[command(flatten)]
    format: FormatArgs,
}

#[derive(Debug, Args)]
struct SequencesArgs {
    /// Repository root. Defaults to the current directory.
    #[arg(long, default_value = ".")]
    root: PathBuf,
    /// Restrict to one corpus object, e.g. "Text D / D".
    #[arg(long)]
    corpus: Option<String>,
    /// Compare whole elements or individual signs.
    #[arg(long, value_enum, default_value = "element")]
    level: LevelArg,
    /// Minimum number of elements (or signs) in a run (at least 2).
    #[arg(long, default_value_t = 2)]
    min_length: usize,
    /// Minimum number of occurrences (at least 2).
    #[arg(long, default_value_t = 2)]
    min_count: usize,
    #[command(flatten)]
    format: FormatArgs,
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
            Command::Sequences(command) => match command.command {
                SequenceSubcommand::List(args) => {
                    let rows = read_csv::<SequenceRecord>(&args.root.join("data/sequences.csv"))?;
                    print_rows(&rows, args.format.format, sequence_columns)
                }
            },
            Command::Readings(command) => match command.command {
                ReadingSubcommand::List(args) => {
                    let rows = read_csv::<ReadingRecord>(&args.root.join("data/readings.csv"))?;
                    print_rows(&rows, args.format.format, reading_columns)
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
            Command::Transcriptions(command) => match command.command {
                TranscriptionSubcommand::List(args) => {
                    let lines = load_lines(&args.root, None)?;
                    print_rows(&lines, args.format.format, transcription_columns)
                }
                TranscriptionSubcommand::Elements(args) => {
                    let elements: Vec<Element> = load_elements(&args.root, None)?
                        .into_iter()
                        .filter(|e| args.line.as_ref().is_none_or(|line| &e.line_id == line))
                        .filter(|e| !args.problems || element_status(e) != "ok")
                        .collect();
                    print_rows(&elements, args.format.format, element_columns)?;
                    print_attribution(args.format.format);
                    Ok(())
                }
            },
            Command::Analyze(command) => match command.command {
                AnalyzeSubcommand::Frequency(args) => {
                    let elements = load_elements(&args.root, args.corpus.as_deref())?;
                    let report = transcription::frequency(&elements, args.exact);
                    print_frequency(&report, args.format.format)
                }
                AnalyzeSubcommand::Sequences(args) => {
                    let elements = load_elements(&args.root, args.corpus.as_deref())?;
                    let level = match args.level {
                        LevelArg::Element => transcription::Level::Element,
                        LevelArg::Part => transcription::Level::Part,
                    };
                    let repeats = transcription::repeated_sequences(
                        &elements,
                        level,
                        args.min_length,
                        args.min_count,
                    );
                    print_rows(&repeats, args.format.format, sequence_repeat_columns)?;
                    print_attribution(args.format.format);
                    Ok(())
                }
                AnalyzeSubcommand::Find(args) => {
                    let elements = load_elements(&args.root, args.corpus.as_deref())?;
                    let matches = transcription::find_pattern(&elements, &args.pattern)
                        .map_err(anyhow::Error::msg)?;
                    print_rows(&matches, args.format.format, pattern_match_columns)?;
                    print_attribution(args.format.format);
                    Ok(())
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
    match (&report.summary, format) {
        (Some(summary), OutputFormat::Json) => {
            println!("{}", serde_json::to_string_pretty(summary)?);
        }
        (Some(summary), OutputFormat::Table) => {
            println!("Audit summary");
            println!("corpus_objects\t{}", summary.corpus_objects);
            println!("sources\t{}", summary.sources);
            println!("claims\t{}", summary.claims);
            println!("observations\t{}", summary.observations);
            println!("sequences\t{}", summary.sequences);
            println!("readings\t{}", summary.readings);
            println!("hypotheses\t{}", summary.hypotheses);
            println!("promoted_claims\t{}", summary.promoted_claims);
            println!("active_hypotheses\t{}", summary.active_hypotheses);
            println!("transcription_lines\t{}", summary.transcription_lines);
        }
        (None, _) => {}
    }

    // In JSON mode, findings go to stderr so stdout stays parseable.
    let json = matches!(format, OutputFormat::Json);
    for message in &report.validation.messages {
        if !matches!(message.level, crate::validate::ValidationLevel::Ok) {
            let line = format!("[{}] {}", message.level, message.message);
            if json {
                eprintln!("{line}");
            } else {
                println!("{line}");
            }
        }
    }

    let totals = format!(
        "validated {} files with {} warning(s) and {} error(s)",
        report.validation.checked_files,
        report.validation.warning_count,
        report.validation.error_count
    );
    if json {
        eprintln!("{totals}");
    } else {
        println!("{totals}");
    }

    if report.validation.error_count == 0 {
        Ok(())
    } else {
        anyhow::bail!("workspace audit failed")
    }
}

fn load_lines(root: &Path, corpus: Option<&str>) -> Result<Vec<TranscriptionLine>> {
    let lines = crate::audit::read_transcriptions(root)?;
    if lines.is_empty() {
        anyhow::bail!(
            "no transcription lines found in {}",
            root.join(validate::TRANSCRIPTIONS_CSV).display()
        );
    }
    let lines: Vec<TranscriptionLine> = lines
        .into_iter()
        .filter(|line| corpus.is_none_or(|id| line.corpus_id == id))
        .collect();
    if lines.is_empty() {
        anyhow::bail!(
            "no transcription lines match corpus {}",
            corpus.unwrap_or("")
        );
    }
    Ok(lines)
}

fn load_elements(root: &Path, corpus: Option<&str>) -> Result<Vec<Element>> {
    Ok(load_lines(root, corpus)?
        .iter()
        .flat_map(|line| transcription::parse_line(&line.line_id, &line.marked_text))
        .collect())
}

const ATTRIBUTION: &str = "Derived from transcription data under its own license (Text D: Lastilla et al. 2022, CC BY-NC 4.0). See data/transcriptions/LICENSE.md.";

fn print_attribution(format: OutputFormat) {
    match format {
        OutputFormat::Table => println!("\n{ATTRIBUTION}"),
        OutputFormat::Json => eprintln!("{ATTRIBUTION}"),
    }
}

fn print_frequency(report: &transcription::FrequencyReport, format: OutputFormat) -> Result<()> {
    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(report)?),
        OutputFormat::Table => {
            println!("sign\tcount\tpossible\tlines");
            for row in &report.rows {
                println!(
                    "{}\t{}\t{}\t{}",
                    row.sign,
                    row.count,
                    row.possible,
                    row.lines.join(",")
                );
            }
            println!(
                "\n{} elements, {} parts. count = certain single readings; possible = underlined (doubtful), marked uncertain (•), or one of several (/) readings.",
                report.elements, report.parts
            );
            if !report.unparsed.is_empty() {
                println!(
                    "{} sign(s) did not match the notation and were excluded: {}",
                    report.unparsed.len(),
                    report.unparsed.join("; ")
                );
            }
        }
    }
    print_attribution(format);
    Ok(())
}

fn element_status(element: &Element) -> &'static str {
    let signs = || element.parts.iter().flat_map(|p| p.alternatives.iter());
    if element.lacuna {
        "lacuna"
    } else if signs().any(|s| s.unparsed) {
        "unparsed"
    } else if element.parts.iter().any(|p| p.alternatives.len() > 1) {
        "ambiguous"
    } else if element.doubtful_join || signs().any(|s| s.doubtful) {
        "doubtful"
    } else if element.is_uncertain() {
        "uncertain"
    } else if element.boundary_possible_join {
        "possible-join"
    } else if element.boundary_doubtful {
        "doubtful-boundary"
    } else if element.irregular_separator {
        "irregular-separator"
    } else {
        "ok"
    }
}

fn transcription_columns(row: &TranscriptionLine) -> Vec<(&'static str, String)> {
    vec![
        ("line_id", row.line_id.clone()),
        ("corpus_id", row.corpus_id.clone()),
        (
            "elements",
            transcription::parse_line(&row.line_id, &row.marked_text)
                .len()
                .to_string(),
        ),
        ("source", row.source_ref.clone()),
        ("license", row.license.clone()),
    ]
}

fn element_columns(row: &Element) -> Vec<(&'static str, String)> {
    vec![
        ("line", row.line_id.clone()),
        ("pos", row.position.to_string()),
        ("text", row.marked.clone()),
        ("status", element_status(row).to_string()),
        ("annotations", row.annotations.join("; ")),
    ]
}

fn sequence_repeat_columns(row: &transcription::RepeatedSequence) -> Vec<(&'static str, String)> {
    let mut columns = vec![
        ("sequence", row.sequence.clone()),
        ("length", row.length.to_string()),
        ("count", row.count.to_string()),
        ("locations", row.locations.join(", ")),
    ];
    if !row.forms.is_empty() {
        columns.push(("forms", row.forms.join(", ")));
    }
    columns
}

fn pattern_match_columns(row: &transcription::PatternMatch) -> Vec<(&'static str, String)> {
    vec![
        ("line", row.line_id.clone()),
        ("start", row.start.to_string()),
        ("end", row.end.to_string()),
        ("text", row.text.clone()),
        (
            "certain",
            if row.certain { "yes" } else { "no" }.to_string(),
        ),
    ]
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

fn sequence_columns(row: &SequenceRecord) -> Vec<(&'static str, String)> {
    vec![
        ("sequence_id", row.sequence_id.clone()),
        ("sequence", row.sequence.clone()),
        ("status", row.status.clone()),
        ("confidence", row.confidence.clone()),
        ("corpus_refs", row.corpus_refs.clone()),
    ]
}

fn reading_columns(row: &ReadingRecord) -> Vec<(&'static str, String)> {
    vec![
        ("reading_id", row.reading_id.clone()),
        ("sign_or_sequence", row.sign_or_sequence.clone()),
        ("meaning", row.proposed_meaning.clone()),
        ("status", row.status.clone()),
        ("confidence", row.confidence.clone()),
    ]
}
