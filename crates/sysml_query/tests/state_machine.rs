//! A state usage publishes the states and transitions it owns, without a view.

use std::sync::Arc;

use sysml_query::resolved_slice::{
    PublishedEffect, PublishedGuard, PublishedStateMachine, PublishedTrigger, QueryAnswer,
    RelationshipTarget,
};
use sysml_query::resolved_slice::{PublishedModel, SymbolId};
use sysml_query::source::{SourceKind, SourceService};

const DOCUMENT: &str = "file:///workspace/timer.sysml";

const SOURCE: &str = r#"
package Timer {
    item def IncrementPressed;
    item def StartStopPressed;
    part def TimerFirmware {
        attribute remaining;
        attribute minimumSetpoint;
        state countdown {
            entry;
            then idle;
            state idle {
                entry action showIdle;
            }
            state running {
                do action countDown;
            }
            state paused;
            state alarming {
                entry action soundAlarm;
            }
            transition increment first idle
                accept IncrementPressed
                do action step
                then idle;
            transition startCountdown first idle
                accept StartStopPressed
                if remaining >= minimumSetpoint
                then running;
            transition complete first running
                accept when remaining == 0
                then alarming;
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
            if let Some(entry) = entries
                .iter()
                .find(|entry| model.qualified_name(entry.identity) == Some(qualified))
            {
                return entry.identity;
            }
            let names = entries
                .iter()
                .filter_map(|entry| model.qualified_name(entry.identity))
                .collect::<Vec<_>>();
            panic!("no declaration named {qualified}; publication has {names:?}");
        }
        other => panic!("expected document symbols, got {other:?}"),
    }
}

fn machine(model: &PublishedModel, qualified: &str) -> PublishedStateMachine {
    let symbol = symbol_named(model, qualified);
    match model.structure().state_machine(symbol).answer {
        QueryAnswer::Resolved(Some(machine)) => machine,
        other => panic!("expected a state machine for {qualified}, got {other:?}"),
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

#[test]
fn countdown_publishes_states_and_transitions() {
    let model = model();
    let countdown = machine(&model, "Timer::TimerFirmware::countdown");
    assert_eq!(countdown.name.as_deref(), Some("countdown"));
    let initial = countdown.initial.as_ref().expect("initial succession");
    assert_eq!(target_name(&model, initial), "idle");

    let state_names = countdown
        .states
        .iter()
        .map(|state| state.name.as_deref().unwrap_or("<anonymous>"))
        .collect::<Vec<_>>();
    assert_eq!(state_names, ["idle", "running", "paused", "alarming"]);

    let idle = &countdown.states[0];
    let show_idle = idle.entry.expect("idle entry");
    assert_eq!(model.symbol_name(show_idle), Some("showIdle"));
    assert!(idle.do_action.is_none());
    assert!(idle.exit.is_none());

    let running = &countdown.states[1];
    let count_down = running.do_action.expect("running do");
    assert_eq!(model.symbol_name(count_down), Some("countDown"));
    assert!(running.entry.is_none());

    let alarming = &countdown.states[3];
    let sound_alarm = alarming.entry.expect("alarming entry");
    assert_eq!(model.symbol_name(sound_alarm), Some("soundAlarm"));

    let names = countdown
        .transitions
        .iter()
        .map(|transition| transition.name.as_deref().unwrap_or("<anonymous>"))
        .collect::<Vec<_>>();
    assert_eq!(names, ["increment", "startCountdown", "complete"]);

    let increment = &countdown.transitions[0];
    assert_eq!(target_name(&model, &increment.source), "idle");
    assert_eq!(target_name(&model, &increment.target), "idle");
    match &increment.trigger {
        PublishedTrigger::Accept { label, .. } => assert_eq!(&**label, "IncrementPressed"),
        other => panic!("expected an accept trigger, got {other:?}"),
    }
    assert!(increment.guard.is_none());
    match &increment.effect {
        PublishedEffect::Resolved(effect) => assert_eq!(model.symbol_name(*effect), Some("step")),
        other => panic!("expected a resolved effect, got {other:?}"),
    }

    let start = &countdown.transitions[1];
    assert_eq!(target_name(&model, &start.source), "idle");
    assert_eq!(target_name(&model, &start.target), "running");
    match &start.trigger {
        PublishedTrigger::Accept { label, .. } => assert_eq!(&**label, "StartStopPressed"),
        other => panic!("expected an accept trigger, got {other:?}"),
    }
    match &start.guard {
        Some(PublishedGuard::Condition(text)) => {
            assert_eq!(&**text, "remaining >= minimumSetpoint")
        }
        other => panic!("expected a condition guard, got {other:?}"),
    }
    assert_eq!(start.effect, PublishedEffect::Absent);

    let complete = &countdown.transitions[2];
    assert_eq!(target_name(&model, &complete.source), "running");
    assert_eq!(target_name(&model, &complete.target), "alarming");
    match &complete.trigger {
        PublishedTrigger::Accept { label, .. } => assert_eq!(&**label, "when remaining == 0"),
        other => panic!("expected a when trigger, got {other:?}"),
    }
    assert!(complete.guard.is_none());
    assert_eq!(complete.effect, PublishedEffect::Absent);
}

#[test]
fn a_part_definition_is_not_a_state_machine() {
    let model = model();
    let firmware = symbol_named(&model, "Timer::TimerFirmware");
    match model.structure().state_machine(firmware).answer {
        QueryAnswer::Resolved(None) => {}
        other => panic!("expected no state machine, got {other:?}"),
    }
}
