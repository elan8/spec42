use spec42_generator_sdk::{diagnostics, export, model, Artifact, Guest};

struct ExampleGenerator;

impl Guest for ExampleGenerator {
    fn generate(args: Vec<String>) -> Result<Vec<Artifact>, String> {
        let info = model::info()?;
        let parts = model::find(Some("PartDefinition"))?;

        // Deliberately not written into an artifact: `model_digest` includes the engine
        // version, so embedding it makes generated output differ on every Spec42 release
        // and defeats `--check`. Provenance belongs in the manifest, which records it.
        diagnostics::log(
            diagnostics::Level::Debug,
            &format!("model {}", info.model_digest),
        );

        let mut markdown = format!("# Generated model\n\nArguments: `{}`\n\n", args.join(" "));
        for part in &parts {
            markdown.push_str(&format!("- `{}`\n", part.qualified_name));
            for feature in model::effective_features(&part.handle)? {
                let detail = model::element(&feature.handle)?;
                let typed_by = model::typed_by(&feature.handle)?
                    .map(|target| target.qualified_name)
                    .unwrap_or_else(|| "<untyped>".to_owned());
                let multiplicity = detail
                    .multiplicity
                    .map(|value| {
                        format!(
                            "{}..{}",
                            value.lower.unwrap_or_else(|| "?".to_owned()),
                            value.upper.unwrap_or_else(|| "?".to_owned())
                        )
                    })
                    .unwrap_or_else(|| "unspecified".to_owned());
                markdown.push_str(&format!(
                    "  - `{}`: `{typed_by}` ({multiplicity})\n",
                    feature.qualified_name
                ));
            }
        }
        let names = parts
            .iter()
            .map(|part| format!("\"{}\"", part.qualified_name.replace('"', "\\\"")))
            .collect::<Vec<_>>()
            .join(",\n  ");
        let parts_json = format!("[\n  {names}\n]\n");
        diagnostics::log(
            diagnostics::Level::Info,
            &format!("generated {} part definition(s)", parts.len()),
        );
        if args.iter().any(|arg| arg == "error-after-build") {
            return Err("requested failure after building artifacts".to_owned());
        }
        if args.iter().any(|arg| arg == "trap-after-build") {
            panic!("requested trap after building artifacts");
        }
        Ok(vec![
            Artifact {
                file_path: "README.md".to_owned(),
                contents: markdown.into_bytes(),
            },
            Artifact {
                file_path: "model/parts.json".to_owned(),
                contents: parts_json.into_bytes(),
            },
        ])
    }
}

export!(ExampleGenerator);
