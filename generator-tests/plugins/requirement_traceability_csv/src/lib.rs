//! Exports authoritative workspace satisfy statements without reconstructing their semantics.

use spec42_generator_sdk::{export, model, Artifact, Guest};
use model::{RelationshipProvenance, SatisfyEndpoint, SatisfyPolarity};

struct RequirementTraceabilityCsv;

impl Guest for RequirementTraceabilityCsv {
    fn generate(_args: Vec<String>) -> Result<Vec<Artifact>, String> {
        let relationships = model::satisfy_relationships()?;
        let rows = relationships
            .into_iter()
            .map(|relationship| {
                let (requirement_qualified_name, requirement_status) = endpoint(relationship.requirement);
                let (satisfying_element_qualified_name, satisfying_element_status) =
                    endpoint(relationship.satisfying_element);
                Row {
                    requirement_qualified_name,
                    satisfying_element_qualified_name,
                    relationship_semantic_id: relationship.semantic_id,
                    polarity: match relationship.polarity {
                        SatisfyPolarity::Satisfied => "satisfied",
                        SatisfyPolarity::NotSatisfied => "not_satisfied",
                    },
                    provenance: match relationship.provenance {
                        RelationshipProvenance::Authored => "authored",
                        RelationshipProvenance::Implied => "implied",
                    },
                    requirement_status,
                    satisfying_element_status,
                    publication_status: if relationship.recovered { "recovered" } else { "resolved" },
                }
            })
            .collect::<Vec<_>>();
        Ok(vec![Artifact {
            file_path: "requirement_traceability.csv".to_owned(),
            contents: encode_csv(&rows).into_bytes(),
        }])
    }
}

fn endpoint(value: SatisfyEndpoint) -> (String, &'static str) {
    match value {
        SatisfyEndpoint::Resolved(value) => (value.qualified_name, "resolved"),
        SatisfyEndpoint::Ambiguous(values) => (
            values.into_iter().map(|value| value.qualified_name).collect::<Vec<_>>().join("|"),
            "ambiguous",
        ),
        SatisfyEndpoint::Unresolved => (String::new(), "unresolved"),
        SatisfyEndpoint::Unsupported => (String::new(), "unsupported"),
    }
}

struct Row {
    requirement_qualified_name: String,
    satisfying_element_qualified_name: String,
    relationship_semantic_id: String,
    polarity: &'static str,
    provenance: &'static str,
    requirement_status: &'static str,
    satisfying_element_status: &'static str,
    publication_status: &'static str,
}

fn encode_csv(rows: &[Row]) -> String {
    let mut output = String::from("requirement_qualified_name,satisfying_element_qualified_name,relationship_semantic_id,polarity,provenance,requirement_status,satisfying_element_status,publication_status\n");
    for row in rows {
        for (index, value) in [
            row.requirement_qualified_name.as_str(), row.satisfying_element_qualified_name.as_str(),
            row.relationship_semantic_id.as_str(), row.polarity, row.provenance,
            row.requirement_status, row.satisfying_element_status, row.publication_status,
        ].into_iter().enumerate() {
            if index != 0 { output.push(','); }
            write_field(&mut output, value);
        }
        output.push('\n');
    }
    output
}

fn write_field(output: &mut String, field: &str) {
    if !field.bytes().any(|byte| matches!(byte, b',' | b'"' | b'\r' | b'\n')) {
        output.push_str(field);
        return;
    }
    output.push('"');
    for character in field.chars() {
        if character == '"' { output.push('"'); }
        output.push(character);
    }
    output.push('"');
}

export!(RequirementTraceabilityCsv);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csv_quotes_special_characters_and_has_one_final_lf() {
        let output = encode_csv(&[Row {
            requirement_qualified_name: "R::Safe,Stop".into(),
            satisfying_element_qualified_name: "S::\"vehicle\"".into(),
            relationship_semantic_id: "id\r\nnext".into(), polarity: "satisfied",
            provenance: "authored", requirement_status: "resolved",
            satisfying_element_status: "resolved", publication_status: "resolved",
        }]);
        assert!(output.contains("\"R::Safe,Stop\",\"S::\"\"vehicle\"\"\",\"id\r\nnext\""));
        assert!(output.ends_with('\n'));
        assert!(!output.ends_with("\n\n"));
    }
}
