use clap::Parser;
use std::fs::{create_dir_all, File};
use std::io::{BufRead, BufReader, Write};
use std::path::{PathBuf};
use std::process::{Command, Stdio};
use std::collections::HashMap;

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

    create_dir_all(&args.output).context("Failed to create output directory")?;

    let anarci_cmd = args.anarci_path.clone().unwrap_or_else(|| PathBuf::from("ANARCI"));
    let output_path = args.output.join("anarci.tsv");

    if which::which(&anarci_cmd).is_err() {
        eprintln!(
            "Error: Could not find ANARCI at '{}'.\n\
            Make sure it is installed and in your PATH,\n\
            or specify the path using --anarci-path.",
            anarci_cmd.display()
        );
        std::process::exit(1);
    }

    // FASTAの配列名から chain (H/L) へのマップを作成
    let fasta_file = File::open(&args.input).context("Failed to open input FASTA file")?;
    let fasta_reader = BufReader::new(fasta_file);
    let mut chain_name_map = HashMap::new();
    let mut current_id = String::new();
    for line in fasta_reader.lines() {
        let line = line?;
        if line.starts_with('>') {
            current_id = line.trim_start_matches('>').to_string();
        } else if current_id.ends_with("_H") {
            chain_name_map.insert("H".to_string(), current_id.clone());
        } else if current_id.ends_with("_L") {
            chain_name_map.insert("L".to_string(), current_id.clone());
        }
    }

    // ANARCI実行
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

    println!("ANARCI finished. Parsing output for CDR extraction...");

    // ANARCI出力のパースとCDR抽出
    let file = File::open(&output_path).context("Failed to open ANARCI output file")?;
    let reader = BufReader::new(file);

    // chainごとにIMGT番号→AA のHashMapを作る
    let mut chains: HashMap<String, HashMap<usize, char>> = HashMap::new();

    for line_res in reader.lines() {
        let line = line_res?;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // TSVの形式: chain_name position amino_acid ...
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 {
            continue;
        }

        let chain = parts[0].to_string(); // "H" または "L"
        let pos: usize = match parts[1].parse() {
            Ok(n) => n,
            Err(_) => continue,
        };
        let aa = parts[2].chars().next().unwrap_or('-');

        chains.entry(chain).or_insert_with(HashMap::new).insert(pos, aa);
    }

    // CDR領域定義 (IMGT番号)
    let cdr_ranges = vec![
        ("cdr1", 27, 38),
        ("cdr2", 56, 65),
        ("cdr3", 105, 117),
    ];

    // chainごとにcdr配列を抽出
    let mut cdr_results = HashMap::new();
    for (chain_id, imgt_map) in &chains {
        let mut cdr_map = HashMap::new();
        for &(cdr_name, start, end) in &cdr_ranges {
            let cdr_seq: String = (start..=end)
                .map(|pos| imgt_map.get(&pos).copied().unwrap_or('-'))
                .collect();
            cdr_map.insert(cdr_name.to_string(), cdr_seq);
        }

        // 配列名（Trastuzumab_H/Lなど）をキーに
        let fasta_id = chain_name_map.get(chain_id).unwrap_or(chain_id);
        cdr_results.insert(fasta_id.clone(), cdr_map);
    }

    if args.json {
        let json_path = args.output.join("results.json");
        let mut json_file = File::create(&json_path).context("Failed to create JSON output file")?;
        let json_output = json!(cdr_results);  // ←ここでcdr_resultsをそのまま使う
        writeln!(json_file, "{}", json_output.to_string())?;
        println!("Saved CDR JSON output to: {}", json_path.display());
    } else {
        for (fasta_id, cdr_map) in &cdr_results {
            println!("[{}]", fasta_id);
            for cdr_name in &["cdr1", "cdr2", "cdr3"] {
                if let Some(seq) = cdr_map.get(*cdr_name) {
                    println!("{}: {}", cdr_name, seq);
                }
            }
            println!();
        }
    }

    Ok(())
}
