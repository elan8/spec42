use spec42_generator_sdk::{artifacts, diagnostics, export, model, Guest};

struct ExampleGenerator;

impl Guest for ExampleGenerator {
    fn generate(args: Vec<String>) -> Result<(), String> {
        let info = model::info();
        let parts = model::find(Some("PartDefinition"))?;

        let mut markdown = format!(
            "# Generated model\n\nModel: `{}`\n\nArguments: `{}`\n\n",
            info.model_digest,
            args.join(" ")
        );
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
        artifacts::emit("README.md", markdown.as_bytes())?;

        let names = parts
            .iter()
            .map(|part| format!("\"{}\"", part.qualified_name.replace('"', "\\\"")))
            .collect::<Vec<_>>()
            .join(",\n  ");
        artifacts::emit("model/parts.json", format!("[\n  {names}\n]\n").as_bytes())?;
        diagnostics::log(
            diagnostics::Level::Info,
            &format!("emitted {} part definition(s)", parts.len()),
        );
        if args.iter().any(|arg| arg == "error-after-emit") {
            return Err("requested failure after emission".to_owned());
        }
        if args.iter().any(|arg| arg == "trap-after-emit") {
            panic!("requested trap after emission");
        }
        Ok(())
    }
}

export!(ExampleGenerator with_types_in spec42_generator_sdk::bindings);
