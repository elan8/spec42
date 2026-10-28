//! Statements whose keywords, names and type references must keep distinct token types:
//! directed parameters, `perform`, state actions, assignments and `for` loops.

use sysml_tokens::{
    ast_semantic_ranges, semantic_tokens_full, TYPE_FUNCTION, TYPE_KEYWORD, TYPE_NAMES,
    TYPE_PROPERTY, TYPE_TYPE,
};

/// Token types for `content`, as `(line, word, type)` in document order.
fn tokens(content: &str) -> Vec<(u32, String, u32)> {
    let parsed = sysml_query::syntax::SyntaxService::new().parse_text(content);
    let ranges = ast_semantic_ranges(&parsed, content);
    let (encoded, _) = semantic_tokens_full(content, Some(&ranges));
    let lines: Vec<&str> = content.lines().collect();
    let (mut line, mut start) = (0u32, 0u32);
    encoded
        .data
        .chunks(5)
        .map(|token| {
            line += token[0];
            start = if token[0] == 0 {
                start + token[1]
            } else {
                token[1]
            };
            let word = lines[line as usize]
                .chars()
                .skip(start as usize)
                .take(token[2] as usize)
                .collect();
            (line, word, token[3])
        })
        .collect()
}

/// Assert the token types of the words on the first line containing `statement`.
fn assert_line(content: &str, statement: &str, expected: &[(&str, u32)]) {
    let line = content
        .lines()
        .position(|text| text.contains(statement))
        .unwrap_or_else(|| panic!("no line contains {statement:?}")) as u32;
    let actual = tokens(content);
    for (word, expected_type) in expected {
        let found = actual
            .iter()
            .find(|(l, w, _)| *l == line && w == word)
            .unwrap_or_else(|| panic!("{word:?} is not a token on the line {statement:?}"));
        assert_eq!(
            found.2, *expected_type,
            "{word:?} in {statement:?} is {}, expected {}",
            TYPE_NAMES[found.2 as usize], TYPE_NAMES[*expected_type as usize]
        );
    }
}

const ACTIONS: &str = r#"package P {
  action def Initialize;
  action def Start {
    in speed : Real;
    out status : Real;
    inout level : Real;
    perform action initialize : Initialize;
    action step : Initialize;
    assign level := speed;
    for i in (1, 2) { }
  }
  part def Engine {
    perform action warmUp : Initialize;
    perform warmUp;
  }
  action run : Initialize {
    in load : Real;
    in :>> speed;
    in limit : Real default speed as Real;
    perform action again : Initialize;
  }
  state def Machine {
    in command : Real;
    entry action init : Initialize;
  }
}"#;

#[test]
fn direction_keywords_stay_keywords_and_parameters_keep_name_and_type() {
    for (statement, direction, name) in [
        ("in speed", "in", "speed"),
        ("out status", "out", "status"),
        ("inout level", "inout", "level"),
        ("in load", "in", "load"),
        ("in command", "in", "command"),
    ] {
        assert_line(
            ACTIONS,
            statement,
            &[
                (direction, TYPE_KEYWORD),
                (name, TYPE_PROPERTY),
                ("Real", TYPE_TYPE),
            ],
        );
    }
}

#[test]
fn redefined_parameter_and_default_value_keep_their_roles() {
    // `in :>> speed;` names the parameter only through the redefinition.
    assert_line(
        ACTIONS,
        "in :>> speed",
        &[("in", TYPE_KEYWORD), ("speed", TYPE_PROPERTY)],
    );
    // The value expression is a feature reference, but the keywords inside it stay keywords.
    assert_line(
        ACTIONS,
        "in limit",
        &[
            ("limit", TYPE_PROPERTY),
            ("default", TYPE_KEYWORD),
            ("speed", TYPE_PROPERTY),
            ("as", TYPE_KEYWORD),
        ],
    );
}

#[test]
fn performed_action_has_a_usage_name_and_a_type_reference() {
    for (statement, name) in [
        ("perform action initialize", "initialize"),
        ("perform action warmUp", "warmUp"),
        ("perform action again", "again"),
    ] {
        assert_line(
            ACTIONS,
            statement,
            &[
                ("perform", TYPE_KEYWORD),
                ("action", TYPE_KEYWORD),
                (name, TYPE_PROPERTY),
                ("Initialize", TYPE_TYPE),
            ],
        );
    }
    // The same reference is coloured alike with and without `perform`.
    assert_line(
        ACTIONS,
        "action step",
        &[("step", TYPE_PROPERTY), ("Initialize", TYPE_TYPE)],
    );
}

#[test]
fn performed_reference_is_a_usage() {
    assert_line(
        ACTIONS,
        "perform warmUp;",
        &[("perform", TYPE_KEYWORD), ("warmUp", TYPE_PROPERTY)],
    );
}

#[test]
fn state_entry_action_has_a_usage_name_and_a_type_reference() {
    assert_line(
        ACTIONS,
        "entry action init",
        &[
            ("entry", TYPE_KEYWORD),
            ("init", TYPE_PROPERTY),
            ("Initialize", TYPE_TYPE),
        ],
    );
}

#[test]
fn assignment_and_for_loop_keywords_stay_keywords() {
    assert_line(
        ACTIONS,
        "assign level",
        &[
            ("assign", TYPE_KEYWORD),
            ("level", TYPE_PROPERTY),
            ("speed", TYPE_PROPERTY),
        ],
    );
    assert_line(
        ACTIONS,
        "for i in",
        &[
            ("for", TYPE_KEYWORD),
            ("i", TYPE_PROPERTY),
            ("in", TYPE_KEYWORD),
        ],
    );
}

#[test]
fn keywords_inside_member_ranges_stay_keywords() {
    // A role only replaces a keyword when its range is exactly that word, so `subject` and
    // `stakeholder` stay keywords while a keyword-spelled name is still an identifier.
    let content = r#"package P {
  part def Vehicle;
  part def Driver;
  requirement def Safe {
    subject vehicle : Vehicle;
    stakeholder driver : Driver;
  }
  enum def Tier {
    entry;
  }
}"#;
    assert_line(
        content,
        "subject vehicle",
        &[("subject", TYPE_KEYWORD), ("Vehicle", TYPE_TYPE)],
    );
    assert_line(
        content,
        "stakeholder driver",
        &[("stakeholder", TYPE_KEYWORD)],
    );
    assert_line(content, "entry;", &[("entry", TYPE_PROPERTY)]);
}

#[test]
fn named_assign_and_terminate_headers_keep_precise_roles_in_definitions_and_usages() {
    let source = r#"package Named {
  action def <work> Work;
  action def Definition {
    action <inc> increment : Work assign lhs := rhs;
    then action <stop> finish : Work terminate;
  }
  action usage {
    action <next> advance : Work assign lhs := rhs;
    then action <haltId> halt : Work terminate self;
  }
}"#;
    for (statement, short, name, keyword) in [
        ("action <inc>", "inc", "increment", "assign"),
        ("then action <stop>", "stop", "finish", "terminate"),
        ("action <next>", "next", "advance", "assign"),
        ("then action <haltId>", "haltId", "halt", "terminate"),
    ] {
        assert_line(
            source,
            statement,
            &[
                ("action", TYPE_KEYWORD),
                (short, TYPE_PROPERTY),
                (name, TYPE_PROPERTY),
                ("Work", TYPE_TYPE),
                (keyword, TYPE_KEYWORD),
            ],
        );
    }
    assert_line(source, "terminate self", &[("self", TYPE_PROPERTY)]);
    assert_line(
        source,
        "action def <work>",
        &[("work", TYPE_FUNCTION), ("Work", TYPE_FUNCTION)],
    );
}
