// src/main.rs

use clap::Parser;

use mdivide::{Cli, FileContext};
use mdivide::utils;

fn main() {
    let cli = Cli::parse();
    let context = match FileContext::from_cli(cli) {
        Ok(ctx) => ctx,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    // check if out_dir exists (if -o uses, go through)
    if let Some(dir) = &context.out_dir {
        if let Err(e) = utils::prepare::ensure_dir(dir) {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }

    // Pair Loop
    for pair in &context.pairs {
        // Output Process
        if let Err(e) = utils::processor::run_process(&pair, &context) {
            eprintln!("{}", e);
            continue;
        }
    }
}
