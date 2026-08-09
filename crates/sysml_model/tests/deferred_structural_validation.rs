//! Deferred structural-validation regressions from the compatibility corpus.
//!
//! These are deliberately ignored rather than silently discarded: the current
//! graph does not retain enough facts to validate them without guessing. Each
//! fixture is a concrete migration target for the relevant graph projection.

#[test]
#[ignore = "SKIP S42-COMPAT-STRUCT-001: connector/flow/allocation end declarations are not projected uniformly as positional ends"]
fn skip_binary_connector_end_cardinality_fixture() {
    let source = r#"
package P {
  connection def Incomplete {
    end feature source;
  }
}
"#;
    panic!("SKIP: retain fixture until positional connector ends are graph facts: {source}");
}

#[test]
#[ignore = "SKIP S42-COMPAT-STRUCT-002: FlowStatementDetail carries a payload name but not the resolved occurrence-type closure"]
fn skip_flow_payload_occurrence_type_fixture() {
    let source = r#"
package P {
  attribute def Scalar;
  part def Source;
  part def Target;
  flow transfer of Scalar from Source to Target;
}
"#;
    panic!("SKIP: retain fixture until flow payload typing is represented as a resolved graph fact: {source}");
}

#[test]
#[ignore = "SKIP S42-COMPAT-STRUCT-003: graph has ownership but no complete featuring-type closure for the KerML overlap constraint"]
fn skip_redefinition_featuring_type_overlap_fixture() {
    let source = r#"
package P {
  part def Vehicle { attribute mass : Real; }
  part def Car :> Vehicle { attribute mass :>> mass; }
}
"#;
    panic!("SKIP: retain fixture until featuring-type closure is graph-backed: {source}");
}
