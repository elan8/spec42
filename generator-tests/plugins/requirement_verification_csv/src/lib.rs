//! Exports authoritative verification links and explicit unsupported outcome state.

use spec42_generator_sdk::{export, model, Artifact, Guest};
use model::{RelationshipProvenance, VerificationOutcome, VerificationRequirement};

struct RequirementVerificationCsv;

impl Guest for RequirementVerificationCsv {
    fn generate(_args: Vec<String>) -> Result<Vec<Artifact>, String> {
        let rows = model::requirement_verifications()?.into_iter().map(|link| {
            let (requirement_qualified_name, requirement_status) = match link.requirement {
                VerificationRequirement::Resolved(value) => (value.qualified_name, "resolved"),
                VerificationRequirement::Ambiguous(values) => (values.into_iter().map(|v| v.qualified_name).collect::<Vec<_>>().join("|"), "ambiguous"),
                VerificationRequirement::Unresolved => (String::new(), "unresolved"),
                VerificationRequirement::Unsupported => (String::new(), "unsupported"),
            };
            Row {
                requirement_qualified_name,
                verification_case_qualified_name: link.verification_case.qualified_name,
                relationship_semantic_id: link.semantic_id,
                provenance: match link.provenance { RelationshipProvenance::Authored => "authored", RelationshipProvenance::Implied => "implied" },
                requirement_status,
                outcome: String::new(),
                outcome_status: match link.outcome { VerificationOutcome::Unsupported => "unsupported" },
                publication_status: if link.recovered { "recovered" } else { "resolved" },
            }
        }).collect::<Vec<_>>();
        Ok(vec![Artifact { file_path: "requirement_verification.csv".into(), contents: encode_csv(&rows).into_bytes() }])
    }
}

struct Row {
    requirement_qualified_name: String,
    verification_case_qualified_name: String,
    relationship_semantic_id: String,
    provenance: &'static str,
    requirement_status: &'static str,
    outcome: String,
    outcome_status: &'static str,
    publication_status: &'static str,
}

fn encode_csv(rows: &[Row]) -> String {
    let mut output = String::from("requirement_qualified_name,verification_case_qualified_name,relationship_semantic_id,provenance,requirement_status,outcome,outcome_status,publication_status\n");
    for row in rows {
        for (index, value) in [row.requirement_qualified_name.as_str(), row.verification_case_qualified_name.as_str(), row.relationship_semantic_id.as_str(), row.provenance, row.requirement_status, row.outcome.as_str(), row.outcome_status, row.publication_status].into_iter().enumerate() {
            if index != 0 { output.push(','); }
            write_field(&mut output, value);
        }
        output.push('\n');
    }
    output
}

fn write_field(output: &mut String, field: &str) {
    if !field.bytes().any(|byte| matches!(byte, b',' | b'"' | b'\r' | b'\n')) { output.push_str(field); return; }
    output.push('"');
    for character in field.chars() { if character == '"' { output.push('"'); } output.push(character); }
    output.push('"');
}

export!(RequirementVerificationCsv);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn csv_quotes_and_has_one_final_lf() {
        let output = encode_csv(&[Row { requirement_qualified_name: "R::Safe,Stop".into(), verification_case_qualified_name: "V::\"Check\"".into(), relationship_semantic_id: "id\r\nnext".into(), provenance: "authored", requirement_status: "resolved", outcome: String::new(), outcome_status: "unsupported", publication_status: "resolved" }]);
        assert!(output.contains("\"R::Safe,Stop\",\"V::\"\"Check\"\"\",\"id\r\nnext\""));
        assert!(output.ends_with('\n')); assert!(!output.ends_with("\n\n"));
    }
}
