use std::path::PathBuf;

use clap::Parser;
use kpar::pack::{build_kpar, PackOptions};
use kpar::schema::Project;

#[derive(Debug, Parser)]
#[command(
    name = "kpar-pack",
    about = "Pack SysML/KerML sources into a KPAR archive"
)]
struct Args {
    /// Repository root containing domain/, technical/, generic/ (or use --source).
    #[arg(long, default_value = ".")]
    root: PathBuf,

    /// Project name for .project.json
    #[arg(long, default_value = "elan8-domain-libraries")]
    name: String,

    /// Project version for .project.json
    #[arg(long)]
    version: String,

    /// Publisher field in .project.json
    #[arg(long, default_value = "elan8")]
    publisher: String,

    /// Output .kpar file path
    #[arg(long)]
    output: PathBuf,

    /// Additional source root directories (repeatable)
    #[arg(long = "source")]
    sources: Vec<PathBuf>,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("kpar-pack: {err}");
        std::process::exit(1);
    }
}

fn run() -> kpar::Result<()> {
    let args = Args::parse();
    let project = Project {
        name: args.name,
        version: args.version.clone(),
        description: Some("Elan8 SysML v2 domain libraries".to_string()),
        license: Some("MIT".to_string()),
        publisher: Some(args.publisher),
        maintainer: vec![],
        website: None,
        topic: vec![],
        usage: vec![],
    };

    let options = if args.sources.is_empty() {
        PackOptions::domain_libraries_defaults(project, &args.root)
    } else {
        PackOptions {
            project,
            source_roots: args.sources,
            excludes: kpar::pack::default_domain_excludes(),
        }
    };

    build_kpar(&options, &args.output)?;
    println!(
        "Wrote {} ({} source roots)",
        args.output.display(),
        options.source_roots.len()
    );
    Ok(())
}
