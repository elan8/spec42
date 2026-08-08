use std::process::ExitCode;

use clap::Parser;
use generator_conformance::{default_corpus_root, run_corpus};

#[derive(Debug, Parser)]
#[command(
    name = "generator-conformance",
    about = "Run the Spec42 generator ABI conformance corpus"
)]
struct Args {
    /// Corpus directory. Defaults to the in-tree `generator-tests`.
    #[arg(long = "corpus")]
    corpus: Option<std::path::PathBuf>,
    /// Run only cases whose id contains this substring.
    #[arg(long = "case")]
    case: Option<String>,
    /// Rewrite goldens from the current behaviour instead of comparing.
    #[arg(long = "bless", default_value_t = false)]
    bless: bool,
}

fn main() -> ExitCode {
    let args = Args::parse();
    let root = args.corpus.unwrap_or_else(default_corpus_root);

    match run_corpus(&root, args.case.as_deref(), args.bless) {
        Ok(results) => {
            let mut failed = 0;
            for result in &results {
                if result.passed() {
                    println!("ok    {} ({:?})", result.id, result.duration);
                } else {
                    failed += 1;
                    println!("FAIL  {}", result.id);
                    for failure in &result.failures {
                        for line in failure.lines() {
                            println!("        {line}");
                        }
                    }
                }
            }
            println!(
                "\n{} case(s): {} passed, {failed} failed{}",
                results.len(),
                results.len() - failed,
                if args.bless { " (blessed)" } else { "" }
            );
            if failed == 0 {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            }
        }
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}
