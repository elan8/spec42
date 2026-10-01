//! SysML 8.3.17.6-13 ControlNode succession validations, settled at the publication barrier.
//!
//! Every rule reads two owned facts and nothing else: the canonical incidence of Successions on
//! ControlNodes ([`crate::resolve::results::ControlNodeSuccession`], derived once from the
//! settled succession end references) and each succession's authored end multiplicities
//! (`DeclarationFacts::succession_end_multiplicities`).
//!
//! # What is not answered
//!
//! The OCL `multiplicityHasBounds` reads a connector end's effective `multiplicity`, which for an
//! end with none authored is inherited through the end's implied specializations. That inherited
//! end multiplicity is not a fact this publication derives, so an end without an authored
//! multiplicity -- and an authored bound that is an expression rather than a literal -- leaves the
//! multiplicity rules unanswered rather than answered from a guessed default.
//!
//! Only Successions the publication models count toward the incoming/outgoing totals. A bare
//! `then <target>;` continuation names an implicit predecessor that is not yet a published fact
//! (`DeclarationKind::ThenContinuation`), so it contributes no succession here.

use crate::lower::facts::MultiplicityRecord;
use crate::model::resolver::SemanticModel;
use crate::model::DeclarationId;
use crate::model::DeclarationKind;
use crate::model::DocumentIdx;
use crate::resolve::results::ControlNodeSuccession;
use crate::resolve::results::ControlNodeSuccessionEnd;
use crate::resolve::results::ResolutionError;
use crate::Diagnostic;
use crate::DiagnosticCode;
use crate::DiagnosticSeverity;

/// The note pointing a control-node succession diagnostic at the node it attaches to.
const RELATED_CONTROL_NODE: &str = "The control node this succession attaches to.";

/// Which authored end multiplicity of a succession a rule constrains.
#[derive(Debug, Clone, Copy)]
enum SuccessionEnd {
    Source,
    Target,
}

impl<D> SemanticModel<D> {
    /// Appends every ControlNode succession diagnostic whose succession is authored in `document`.
    pub(crate) fn collect_control_node_succession_rules(
        &self,
        document: DocumentIdx,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        let incidences = &self.resolution.control_node_successions;
        // Incidences are ordered by `(node, end, succession)`, so the position of an incidence
        // within its node's same-end run is its canonical declaration-order rank.
        let mut rank = 0usize;
        for (index, incidence) in incidences.iter().enumerate() {
            rank = match index.checked_sub(1).map(|previous| &incidences[previous]) {
                Some(previous)
                    if previous.node == incidence.node && previous.end == incidence.end =>
                {
                    rank + 1
                }
                _ => 0,
            };
            let succession = self
                .storage
                .declaration(incidence.succession)
                .ok_or(ResolutionError::InvalidStorage)?;
            if succession.document != document {
                continue;
            }
            let node_kind = self
                .kind_of(incidence.node)
                .ok_or(ResolutionError::InvalidStorage)?;
            for (code, end, bounds) in multiplicity_rules(node_kind, incidence.end) {
                if self.end_multiplicity_violates(incidence.succession, end, bounds) {
                    diagnostics.push(self.control_node_diagnostic(incidence, code)?);
                }
            }
            if rank > 0 {
                if let Some(code) = single_succession_rule(node_kind, incidence.end) {
                    diagnostics.push(self.control_node_diagnostic(incidence, code)?);
                }
            }
        }
        Ok(())
    }

    /// Whether `succession`'s authored `end` multiplicity is literal and differs from `bounds`.
    fn end_multiplicity_violates(
        &self,
        succession: DeclarationId,
        end: SuccessionEnd,
        bounds: (i64, Option<i64>),
    ) -> bool {
        let Some(ends) = self
            .storage
            .declaration_facts(succession)
            .and_then(|facts| facts.succession_end_multiplicities.as_deref())
        else {
            return false;
        };
        let multiplicity: Option<&MultiplicityRecord> = match end {
            SuccessionEnd::Source => ends.source.as_ref(),
            SuccessionEnd::Target => ends.target.as_ref(),
        };
        multiplicity
            .and_then(Self::literal_bounds)
            .is_some_and(|authored| authored != bounds)
    }

    fn control_node_diagnostic(
        &self,
        incidence: &ControlNodeSuccession,
        code: DiagnosticCode,
    ) -> Result<Diagnostic, ResolutionError> {
        let mut diagnostic =
            self.declaration_diagnostic(incidence.succession, code, DiagnosticSeverity::Warning)?;
        diagnostic.related =
            Box::new([self.related_declaration(incidence.node, RELATED_CONTROL_NODE)?]);
        Ok(diagnostic)
    }
}

/// The end-multiplicity rules that apply to a succession attached at `end` of a `node_kind`
/// ControlNode, each with the end it constrains and the exact bounds it requires.
fn multiplicity_rules(
    node_kind: DeclarationKind,
    end: ControlNodeSuccessionEnd,
) -> Vec<(DiagnosticCode, SuccessionEnd, (i64, Option<i64>))> {
    let mut rules = Vec::new();
    match end {
        ControlNodeSuccessionEnd::Incoming => {
            // 8.3.17.6 validateControlNodeIncomingSuccessions: target multiplicity 1..1.
            rules.push((
                DiagnosticCode::ControlNodeIncomingMultiplicity,
                SuccessionEnd::Target,
                (1, Some(1)),
            ));
            // 8.3.17.13 validateMergeNodeIncomingSuccessions: source multiplicity 0..1.
            if node_kind == DeclarationKind::Merge {
                rules.push((
                    DiagnosticCode::MergeNodeIncomingMultiplicity,
                    SuccessionEnd::Source,
                    (0, Some(1)),
                ));
            }
        }
        ControlNodeSuccessionEnd::Outgoing => {
            // 8.3.17.6 validateControlNodeOutgoingSuccessions: source multiplicity 1..1.
            rules.push((
                DiagnosticCode::ControlNodeOutgoingMultiplicity,
                SuccessionEnd::Source,
                (1, Some(1)),
            ));
            // 8.3.17.7 validateDecisionNodeOutgoingSuccessions: target multiplicity 0..1.
            if node_kind == DeclarationKind::Decide {
                rules.push((
                    DiagnosticCode::DecisionNodeOutgoingMultiplicity,
                    SuccessionEnd::Target,
                    (0, Some(1)),
                ));
            }
        }
    }
    rules
}

/// The at-most-one-succession rule for `end` of a `node_kind` ControlNode, if any. Reported at
/// every succession after the first in declaration order.
fn single_succession_rule(
    node_kind: DeclarationKind,
    end: ControlNodeSuccessionEnd,
) -> Option<DiagnosticCode> {
    match (node_kind, end) {
        // 8.3.17.7 validateDecisionNodeIncomingSuccessions.
        (DeclarationKind::Decide, ControlNodeSuccessionEnd::Incoming) => {
            Some(DiagnosticCode::DecisionNodeMultipleIncoming)
        }
        // 8.3.17.8 validateForkNodeIncomingSuccessions.
        (DeclarationKind::Fork, ControlNodeSuccessionEnd::Incoming) => {
            Some(DiagnosticCode::ForkNodeMultipleIncoming)
        }
        // 8.3.17.11 validateJoinNodeOutgoingSuccessions.
        (DeclarationKind::Join, ControlNodeSuccessionEnd::Outgoing) => {
            Some(DiagnosticCode::JoinNodeMultipleOutgoing)
        }
        // 8.3.17.13 validateMergeNodeOutgoingSuccessions.
        (DeclarationKind::Merge, ControlNodeSuccessionEnd::Outgoing) => {
            Some(DiagnosticCode::MergeNodeMultipleOutgoing)
        }
        _ => None,
    }
}
