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

    /// Named source roots as PREFIX=PATH (repeatable). Example: method=/path/to/library
    #[arg(long = "named-source", value_name = "PREFIX=PATH")]
    named_sources: Vec<String>,
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

    let mut named_source_roots = Vec::new();
    for entry in &args.named_sources {
        let (prefix, path) = entry.split_once('=').ok_or_else(|| {
            kpar::error::KparError::InvalidArchive(format!(
                "invalid --named-source '{entry}' (expected PREFIX=PATH)"
            ))
        })?;
        let prefix = prefix.trim();
        let path = PathBuf::from(path.trim());
        if prefix.is_empty() || !path.is_dir() {
            return Err(kpar::error::KparError::InvalidArchive(format!(
                "invalid --named-source '{entry}' (empty prefix or missing directory)"
            )));
        }
        named_source_roots.push((prefix.to_string(), path));
    }

    let mut options = if args.sources.is_empty() {
        PackOptions::domain_libraries_defaults(project, &args.root)
    } else {
        PackOptions {
            project,
            source_roots: args.sources,
            named_source_roots: Vec::new(),
            excludes: kpar::pack::default_domain_excludes(),
        }
    };
    options.named_source_roots.extend(named_source_roots);

    let root_count = options.source_roots.len() + options.named_source_roots.len();
    build_kpar(&options, &args.output)?;
    println!(
        "Wrote {} ({} source roots)",
        args.output.display(),
        root_count
    );
    Ok(())
}
