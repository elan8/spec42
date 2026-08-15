//! Generates a catalogue of workspace-authored requirement definitions.
//!
//! The rows deliberately retain the order returned by `find`: ordering is part of the
//! semantic query contract, and sorting here would conceal host nondeterminism.

use spec42_generator_sdk::{export, model, Artifact, Guest};

struct RequirementsCsv;

impl Guest for RequirementsCsv {
    fn generate(_args: Vec<String>) -> Result<Vec<Artifact>, String> {
        let requirements = model::find(Some("RequirementDefinition"))?;
        let mut rows = Vec::new();
        for requirement in requirements {
            // Libraries take part in resolution but are not authored catalogue entries.
            if requirement.library_element {
                continue;
            }
            let detail = model::element(&requirement.handle)?;
            rows.push(RequirementRow {
                qualified_name: detail.summary.qualified_name,
                name: detail.summary.name.unwrap_or_default(),
                documentation: detail.documentation.unwrap_or_default(),
            });
        }

        Ok(vec![Artifact {
            file_path: "requirements.csv".to_owned(),
            contents: encode_csv(&rows).into_bytes(),
        }])
    }
}

struct RequirementRow {
    qualified_name: String,
    name: String,
    documentation: String,
}

fn encode_csv(rows: &[RequirementRow]) -> String {
    let mut output = String::from("qualified_name,name,documentation\n");
    for row in rows {
        write_field(&mut output, &row.qualified_name);
        output.push(',');
        write_field(&mut output, &row.name);
        output.push(',');
        write_field(&mut output, &row.documentation);
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

export!(RequirementsCsv);

#[cfg(test)]
mod tests {
    use super::{encode_csv, RequirementRow};

    #[test]
    fn emits_header_rows_in_input_order_and_one_final_lf() {
        let output = encode_csv(&[
            RequirementRow {
                qualified_name: "Requirements::Second".to_owned(),
                name: "Second".to_owned(),
                documentation: "second".to_owned(),
            },
            RequirementRow {
                qualified_name: "Requirements::First".to_owned(),
                name: "First".to_owned(),
                documentation: "first".to_owned(),
            },
        ]);

        assert_eq!(
            output,
            "qualified_name,name,documentation\n\
             Requirements::Second,Second,second\n\
             Requirements::First,First,first\n"
        );
        assert!(!output.ends_with("\n\n"));
    }

    #[test]
    fn quotes_csv_special_characters_and_doubles_quotes() {
        let output = encode_csv(&[RequirementRow {
            qualified_name: "Requirements::Safe,Stop".to_owned(),
            name: "Safe \"Stop\"".to_owned(),
            documentation: "first line\r\nsecond line".to_owned(),
        }]);

        assert_eq!(
            output,
            "qualified_name,name,documentation\n\
             \"Requirements::Safe,Stop\",\"Safe \"\"Stop\"\"\",\"first line\r\nsecond line\"\n"
        );
    }

    #[test]
    fn empty_catalogue_still_has_a_final_lf() {
        assert_eq!(encode_csv(&[]), "qualified_name,name,documentation\n");
    }
}
