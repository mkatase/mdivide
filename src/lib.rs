// src/lib.rs

use clap::Parser;
use std::collections::HashMap;

use crate::error::AppError;

#[derive(Parser)]
#[clap(
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    group(clap::ArgGroup::new("mode")
        .required(true).args(&["input", "file"]), ))]
pub struct Cli {
    #[clap(short, long, value_name = "Lang", help = "Input Lang")]
    pub lang: Option<String>,
    #[clap(short, long, value_name = "Input", help = "Input File", conflicts_with = "file")]
    pub input: Option<String>,
    #[clap(short, long, value_name = "Output", help = "Output File")]
    pub output: Option<String>,
    #[clap(short, long, value_name = "File List", help = "List File", conflicts_with = "input")]
    pub file: Option<String>,
    #[clap(short, long, value_name = "Dir", help = "Output Directory")]
    pub dir: Option<String>,
    #[clap(short, long, help = "Skip mode")]
    pub skip: bool,
    #[clap(short, long, help = "Check mode (dry-run)")]
    pub check: bool,

}

#[derive(Debug, Clone)]
pub struct FilePair {
    pub input: String,
    pub output: String,
}

#[derive(Debug, Clone)]
pub struct FileContext {
    pub lang: Option<String>,
    pub pairs: Vec<FilePair>,
    pub skip: bool,
    pub check: bool,
    pub out_dir: Option<String>,
}

impl FileContext {
    pub fn from_cli(cli: Cli) -> Result<Self, AppError> {
        let is_check = cli.check;

        // --- 1. 実働モード (!check) の時の厳格な検問 ---
        if !is_check {
            if cli.lang.is_none() {
                return Err(AppError::Config(
                        "Error: -l (language) is REQUIRED for output mode. Use --check for dry-run.".into()));
            }
            // -l common check
            if cli.lang.as_ref().unwrap() == "common" {
                return Err(AppError::Config(
                    "'-l common' is NOT allowed. 'common' is a reserved tag for global content.".into()));
            }
            // for -i and -o pair check
            if cli.input.is_some() && cli.output.is_none() && cli.dir.is_none() {
                return Err(AppError::Config(
                    "Error: -o (output) is REQUIRED when using -i in output mode.".into()));
            }
            // for -f and -d pair check
            if cli.file.is_some() && cli.dir.is_none() {
                return Err(AppError::Config(
                    "Error: -d (directory) is REQUIRED when using -f in output mode.".into()));
            }
        }

        // Gen pairs
        let mut pairs = Vec::new();

        // for single file mode
        if let Some(input_file) = cli.input {
            pairs.push(FilePair {
                input: input_file,
                output: cli.output.clone().unwrap_or_default(),
            });
        } else if let Some(list_file) = cli.file {
            // for file list mode
            let content = std::fs::read_to_string(&list_file).map_err(AppError::Io)?;
            for line in content.lines().map(|l| l.trim()).filter(|l| !l.is_empty()) {
                if line.starts_with('#') {
                    continue;
                }
                pairs.push(FilePair {
                    input: line.to_string(),
                    output: String::new(), // processor側で動的に生成される
                });
            }
        }

        Ok(FileContext {
            lang: cli.lang,
            pairs,
            skip: cli.skip,
            check: is_check,
            out_dir: cli.dir,
        })
    }
}

pub type TagMap = HashMap<String, usize>;

pub mod error;
pub mod utils;
