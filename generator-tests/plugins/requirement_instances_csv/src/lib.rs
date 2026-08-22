//! Exports workspace-authored requirement usages and their authoritative typing state.

use spec42_generator_sdk::{export, model, Artifact, Guest};

struct RequirementInstancesCsv;

impl Guest for RequirementInstancesCsv {
    fn generate(_args: Vec<String>) -> Result<Vec<Artifact>, String> {
        let usages = model::find(Some("RequirementUsage"))?;
        let mut rows = Vec::with_capacity(usages.len());
        for usage in usages {
            let (definition, status, provenance) =
                match model::requirement_usage_typing(&usage.handle)? {
                    model::RequirementUsageTyping::Resolved {
                        definition,
                        provenance,
                    } => (
                        definition.qualified_name,
                        "resolved",
                        match provenance {
                            model::TypingProvenance::Authored => "authored",
                            model::TypingProvenance::Implied => "implied",
                        },
                    ),
                    model::RequirementUsageTyping::RecoveredResolved {
                        definition,
                        provenance,
                    } => (
                        definition.qualified_name,
                        "recovered",
                        match provenance {
                            model::TypingProvenance::Authored => "authored",
                            model::TypingProvenance::Implied => "implied",
                        },
                    ),
                    model::RequirementUsageTyping::RecoveredMissing => {
                        (String::new(), "missing-recovery", "")
                    }
                    model::RequirementUsageTyping::RecoveredUnresolved => {
                        (String::new(), "unresolved-recovery", "")
                    }
                    model::RequirementUsageTyping::RecoveredAmbiguous { .. } => {
                        (String::new(), "ambiguous-recovery", "")
                    }
                    model::RequirementUsageTyping::RecoveredUnsupported => {
                        (String::new(), "unsupported-recovery", "")
                    }
                    model::RequirementUsageTyping::Missing => (String::new(), "missing", ""),
                    model::RequirementUsageTyping::Unresolved => (String::new(), "unresolved", ""),
                    model::RequirementUsageTyping::Ambiguous { .. } => {
                        (String::new(), "ambiguous", "")
                    }
                    model::RequirementUsageTyping::Unsupported => {
                        (String::new(), "unsupported", "")
                    }
                    model::RequirementUsageTyping::Recovery => (String::new(), "recovery", ""),
                    model::RequirementUsageTyping::Incomplete => (String::new(), "incomplete", ""),
                };
            rows.push(Row {
                qualified_name: usage.qualified_name,
                name: usage.name.unwrap_or_default(),
                definition,
                status,
                provenance,
            });
        }
        Ok(vec![Artifact {
            file_path: "requirement_instances.csv".into(),
            contents: encode_csv(&rows).into_bytes(),
        }])
    }
}

struct Row {
    qualified_name: String,
    name: String,
    definition: String,
    status: &'static str,
    provenance: &'static str,
}

fn encode_csv(rows: &[Row]) -> String {
    let mut output = String::from("qualified_name,name,requirement_definition_qualified_name,typing_status,typing_provenance\n");
    for row in rows {
        for (index, field) in [
            row.qualified_name.as_str(),
            row.name.as_str(),
            row.definition.as_str(),
            row.status,
            row.provenance,
        ]
        .into_iter()
        .enumerate()
        {
            if index != 0 {
                output.push(',');
            }
            write_field(&mut output, field);
        }
        output.push('\n');
    }
    output
}

fn write_field(output: &mut String, field: &str) {
    if !field
        .bytes()
        .any(|byte| matches!(byte, b',' | b'"' | b'\r' | b'\n'))
    {
        output.push_str(field);
        return;
    }
    output.push('"');
    for character in field.chars() {
        if character == '"' {
            output.push('"');
        }
        output.push(character);
    }
    output.push('"');
}

export!(RequirementInstancesCsv);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_order_quotes_special_characters_and_emits_one_final_lf() {
        let output = encode_csv(&[Row {
            qualified_name: "P::Second,\nusage".into(),
            name: "Second \"usage\"".into(),
            definition: "P::SecondDefinition".into(),
            status: "resolved",
            provenance: "authored",
        }]);
        assert_eq!(output, "qualified_name,name,requirement_definition_qualified_name,typing_status,typing_provenance\n\"P::Second,\nusage\",\"Second \"\"usage\"\"\",P::SecondDefinition,resolved,authored\n");
        assert!(!output.ends_with("\n\n"));
    }

    #[test]
    fn empty_output_is_a_header_with_one_final_lf() {
        assert_eq!(encode_csv(&[]), "qualified_name,name,requirement_definition_qualified_name,typing_status,typing_provenance\n");
    }
}
