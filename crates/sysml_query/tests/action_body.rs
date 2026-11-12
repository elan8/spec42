//! An action publishes the steps it owns, including their settled expressions.

use std::sync::Arc;

use sysml_query::resolved_slice::{
    ExpressionNodeKind, ExpressionOutcome, PublishedActionBody, PublishedActionBranch,
    PublishedActionStep, PublishedExpression, PublishedModel, PublishedTrigger, QueryAnswer,
    RelationshipTarget, SymbolId,
};
use sysml_query::source::{SourceKind, SourceService};

const DOCUMENT: &str = "file:///workspace/firmware.sysml";

const SOURCE: &str = r#"
package Timer {
    item def IncrementPressed;
    enum def TimerMode { idle; }
    part def Firmware {
        attribute mode : TimerMode;
        attribute remaining;
        attribute maximumSetpoint;
        attribute sounderDriven;
        action showIdle {
            assign mode := TimerMode::idle;
            then assign sounderDriven := false;
        }
        action countDown {
            while remaining > 1 {
                accept after 1;
                then assign remaining := remaining - 1;
            }
            then accept after 1;
            then assign remaining := 0;
        }
        action ping {
            send new IncrementPressed() to Firmware;
        }
        action step {
            if remaining > maximumSetpoint {
                assign remaining := maximumSetpoint;
            } else {
                assign remaining := remaining + 60;
            }
        }
    }
}
"#;

fn model() -> Arc<PublishedModel> {
    let source = SourceService::new();
    let document = source
        .admit(DOCUMENT, SOURCE, SourceKind::Workspace)
        .unwrap();
    sysml_query::Services::new()
        .publication
        .publish(&[document], [])
        .unwrap()
}

fn symbol_named(model: &PublishedModel, qualified: &str) -> SymbolId {
    match model.inspection().document_symbols(DOCUMENT).answer {
        QueryAnswer::Resolved(entries) => {
            entries
                .iter()
                .find(|entry| model.qualified_name(entry.identity) == Some(qualified))
                .unwrap_or_else(|| panic!("no declaration named {qualified}"))
                .identity
        }
        other => panic!("expected document symbols, got {other:?}"),
    }
}

fn body(model: &PublishedModel, qualified: &str) -> PublishedActionBody {
    let symbol = symbol_named(model, qualified);
    match model.structure().action_body(symbol).answer {
        QueryAnswer::Resolved(Some(body)) => body,
        other => panic!("expected an action body for {qualified}, got {other:?}"),
    }
}

fn target_name(model: &PublishedModel, target: &RelationshipTarget) -> String {
    match target {
        RelationshipTarget::Resolved(identity) => model
            .symbol_name(*identity)
            .unwrap_or("<anonymous>")
            .to_owned(),
        other => panic!("expected a resolved target, got {other:?}"),
    }
}

fn expression_text(expression: &PublishedExpression) -> String {
    assert_eq!(expression.outcome, ExpressionOutcome::Resolved);
    let Some(root) = expression.root else {
        panic!("resolved expression has no root");
    };
    render(expression, root)
}

fn render(expression: &PublishedExpression, index: u32) -> String {
    match &expression.nodes[index as usize].kind {
        ExpressionNodeKind::FeatureReference { authored, .. } => authored.to_string(),
        ExpressionNodeKind::Literal(value) => format!("{value:?}"),
        ExpressionNodeKind::Operator { operator, operands } => {
            let rendered = operands
                .iter()
                .map(|operand| render(expression, *operand))
                .collect::<Vec<_>>()
                .join(" ");
            format!("{} {rendered}", operator.as_str())
        }
        ExpressionNodeKind::Invocation { callee, arguments } => {
            let mut parts = vec![render(expression, *callee)];
            parts.extend(
                arguments
                    .iter()
                    .map(|argument| render(expression, *argument)),
            );
            parts.join(" ")
        }
        ExpressionNodeKind::Sequence { elements } => elements
            .iter()
            .map(|element| render(expression, *element))
            .collect::<Vec<_>>()
            .join(", "),
        other => format!("{other:?}"),
    }
}

#[test]
fn countdown_actions_publish_their_steps() {
    let model = model();

    let show_idle = body(&model, "Timer::Firmware::showIdle");
    assert_eq!(show_idle.steps.len(), 2);
    let PublishedActionStep::Assign {
        identity: first,
        target,
        value,
        then_source,
    } = &show_idle.steps[0]
    else {
        panic!("expected an assignment, got {:?}", show_idle.steps[0]);
    };
    assert_eq!(target_name(&model, target), "mode");
    assert!(then_source.is_none());
    assert!(
        expression_text(value).contains("idle"),
        "mode assignment was {}",
        expression_text(value)
    );
    let PublishedActionStep::Assign {
        target,
        value,
        then_source,
        ..
    } = &show_idle.steps[1]
    else {
        panic!("expected an assignment, got {:?}", show_idle.steps[1]);
    };
    assert_eq!(target_name(&model, target), "sounderDriven");
    assert_eq!(then_source.as_ref(), Some(first));
    assert!(
        expression_text(value).contains("false") || expression_text(value).contains("False"),
        "sounder assignment was {}",
        expression_text(value)
    );

    let count_down = body(&model, "Timer::Firmware::countDown");
    assert_eq!(count_down.steps.len(), 3);
    let PublishedActionStep::While {
        condition,
        body: loop_steps,
        ..
    } = &count_down.steps[0]
    else {
        panic!("expected a while, got {:?}", count_down.steps[0]);
    };
    let condition_text = expression_text(condition);
    assert!(
        condition_text.contains("remaining"),
        "while condition was {condition_text}"
    );
    assert_eq!(loop_steps.len(), 2);
    let PublishedActionStep::Accept { trigger, .. } = &loop_steps[0] else {
        panic!("expected an accept, got {:?}", loop_steps[0]);
    };
    match trigger {
        PublishedTrigger::Accept { label, .. } => assert_eq!(&**label, "after 1"),
        other => panic!("expected an after trigger, got {other:?}"),
    }
    let PublishedActionStep::Assign { target, value, .. } = &loop_steps[1] else {
        panic!("expected an assignment, got {:?}", loop_steps[1]);
    };
    assert_eq!(target_name(&model, target), "remaining");
    let value_text = expression_text(value);
    assert!(
        value_text.contains("remaining"),
        "loop assignment was {value_text}"
    );

    let PublishedActionStep::Accept { trigger, .. } = &count_down.steps[1] else {
        panic!("expected an accept, got {:?}", count_down.steps[1]);
    };
    match trigger {
        PublishedTrigger::Accept { label, .. } => assert_eq!(&**label, "after 1"),
        other => panic!("expected an after trigger, got {other:?}"),
    }
    let PublishedActionStep::Assign { target, .. } = &count_down.steps[2] else {
        panic!("expected an assignment, got {:?}", count_down.steps[2]);
    };
    assert_eq!(target_name(&model, target), "remaining");

    let step = body(&model, "Timer::Firmware::step");
    let PublishedActionStep::If {
        condition,
        then_branch,
        else_branch,
        ..
    } = &step.steps[0]
    else {
        panic!("expected an if, got {:?}", step.steps[0]);
    };
    let condition_text = expression_text(condition);
    assert!(
        condition_text.contains("remaining") && condition_text.contains("maximumSetpoint"),
        "if condition was {condition_text}"
    );
    let PublishedActionBranch::Steps(then_steps) = then_branch else {
        panic!("then branch was unmarked");
    };
    let PublishedActionStep::Assign { target, .. } = &then_steps[0] else {
        panic!("expected an assignment, got {:?}", then_steps[0]);
    };
    assert_eq!(target_name(&model, target), "remaining");
    let Some(PublishedActionBranch::Steps(else_steps)) = else_branch else {
        panic!("else branch was missing or unmarked");
    };
    let PublishedActionStep::Assign { value, .. } = &else_steps[0] else {
        panic!("expected an assignment, got {:?}", else_steps[0]);
    };
    assert!(
        expression_text(value).contains("remaining"),
        "else assignment was {}",
        expression_text(value)
    );
}

#[test]
fn publishes_a_send_and_its_receiver() {
    let model = model();
    let body = body(&model, "Timer::Firmware::ping");
    let PublishedActionStep::Send {
        payload, receiver, ..
    } = &body.steps[0]
    else {
        panic!("expected a send, got {:?}", body.steps[0]);
    };
    assert!(
        expression_text(payload).contains("IncrementPressed"),
        "payload was {}",
        expression_text(payload)
    );
    let Some(RelationshipTarget::Resolved(target)) = receiver else {
        panic!("receiver did not resolve: {receiver:?}");
    };
    assert_eq!(model.symbol_name(*target), Some("Firmware"));
}

#[test]
fn a_part_definition_is_not_an_action_body() {
    let model = model();
    let firmware = symbol_named(&model, "Timer::Firmware");
    match model.structure().action_body(firmware).answer {
        QueryAnswer::Resolved(None) => {}
        other => panic!("expected no action body, got {other:?}"),
    }
}
