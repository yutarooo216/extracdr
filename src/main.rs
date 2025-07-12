use clap::Parser;
use std::fs::{create_dir_all, File};
use std::io::{BufRead, BufReader, Write};
use std::path::{PathBuf};
use std::process::{Command, Stdio};

use anyhow::{Context, Result};
use serde_json::json;

/// CLI argument definition
#[derive(Parser)]
#[command(author, version, about = "CDR extraction tool using ANARCI")]
struct Args {
    /// Input FASTA file
    #[arg(short, long)]
    input: PathBuf,

    /// Output directory (default: outputs)
    #[arg(short, long, default_value = "outputs")]
    output: PathBuf,

    /// Output in JSON format (creates outputs/results.json)
    #[arg(long)]
    json: bool,

    /// Path to ANARCI executable (default: use `anarci` from PATH)
    #[arg(long)]
    anarci_path: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Create output directory if it doesn't exist
    create_dir_all(&args.output)
        .context("Failed to create output directory")?;

    let anarci_cmd = args.anarci_path.clone().unwrap_or_else(|| PathBuf::from("anarci"));
    let output_path = args.output.join("anarci.tsv");

    // Check if ANARCI is available
    if which::which(&anarci_cmd).is_err() {
        eprintln!(
            "Error: Could not find ANARCI at '{}'.\n\
            Make sure it is installed and in your PATH,\n\
            or specify the path using --anarci-path.",
            anarci_cmd.display()
        );
        std::process::exit(1);
    }

    // Run ANARCI subprocess
    let status = Command::new(&anarci_cmd)
        .args(&[
            "-i", args.input.to_str().unwrap(),
            "-o", output_path.to_str().unwrap(),
            "--scheme", "imgt",
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .with_context(|| format!("Failed to execute ANARCI at {}", anarci_cmd.display()))?;

    if !status.success() {
        eprintln!("Error: ANARCI failed (exit code: {}).", status.code().unwrap_or(-1));
        std::process::exit(1);
    }

    println!("CDR extraction completed. Output directory: {}", args.output.display());

    // If JSON flag is set, convert output to JSON
    if args.json {
        let tsv_file = File::open(&output_path)
            .context("Failed to open ANARCI output file")?;
        let reader = BufReader::new(tsv_file);

        let mut results = vec![];

        for line in reader.lines() {
            let line = line?;
            if line.starts_with('#') || line.trim().is_empty() {
                continue;
            }
            results.push(line);
        }

        let json_path = args.output.join("results.json");
        let mut json_file = File::create(&json_path)
            .context("Failed to create results.json")?;
        let json_output = json!({ "cdr_results": results });
        writeln!(json_file, "{}", json_output.to_string())?;

        println!("Saved JSON output to: {}", json_path.display());
    }

    Ok(())
}
