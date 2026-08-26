//! The conformance families a validating host reports, decided from settled facts at the barrier.
//!
//! These are the rules the legacy graph engine owned: namespace identity, connection, behavior,
//! requirement/case, view and inherited-value conformance, plus the two authoring hints a host
//! surfaces. Every one of them reads what an earlier phase settled -- a
//! declaration's kind, an authored reference's outcome, the derived redefinitions, the effective
//! type closure, the evaluated value -- and asks a structural question about it.
//!
//! # What changed relative to the legacy checks
//!
//! The legacy versions worked over a mutable graph that could not state most of these facts, so
//! they recovered them from text: an endpoint's kind by re-resolving a dotted expression per
//! diagnostic, an inherited member by walking types at report time, a payload's kind by parsing the
//! authored type reference, an enum-valued attribute by testing whether the value text started with
//! a quote, a non-standard view by testing whether the authored type name ended in `View`. Each of
//! those agrees with the model often enough to look right and is wrong exactly where a model is
//! unusual.
//!
//! Here, each rule names the settled fact it reads. Where the fact is genuinely absent, the rule
//! declines to answer and says which fact is missing, rather than guessing from a name.
//!
//! # Ordering
//!
//! None of these rules sorts. [`super::ResolvedSemanticModel::derive_diagnostics`] canonicalizes
//! each document's diagnostics by range and code once every producer has contributed, so the order
//! a rule happens to visit storage in is never observable.

use crate::lower::facts::AnnotationForm;
use crate::lower::facts::AuthoredReference;
use crate::lower::facts::MultiplicityBound;
use crate::model::render as writer;
use crate::model::resolver::SemanticModel;
use crate::model::span::document_range;
use crate::model::AuthoredReferenceId;
use crate::model::DeclarationId;
use crate::model::DeclarationKind;
use crate::model::DocumentIdx;
use crate::model::ReferenceKind;
use crate::resolve::results::ResolutionError;
use crate::resolve::results::ResolutionStatus;
use crate::type_query::Conformance;
use crate::type_query::SpecializationScope;
use crate::Diagnostic;
use crate::DiagnosticCode;
use crate::DiagnosticLocation;
use crate::DiagnosticOrigin;
use crate::DiagnosticSeverity;
use crate::TextRange;

use std::collections::{BTreeMap, BTreeSet};

use crate::check::conformance::{Family, Role};
use crate::evaluation::{EvaluatedScalar, EvaluationState};

/// The note attached to the earlier member a duplicate name collides with.
pub(crate) const RELATED_FIRST_DECLARATION: &str = "First declared here.";
/// The note attached to the element a relationship names.
pub(crate) const RELATED_TARGET: &str = "Target resolved here.";
/// The note attached to the member a feature implicitly redefines.
pub(crate) const RELATED_INHERITED: &str = "Inherited member this feature overrides.";
/// The note attached to the other end of a connection-like relationship.
pub(crate) const RELATED_OTHER_END: &str = "Other end declared here.";

/// The SysML v2 standard view definitions (§9.2.20 Table 34).
///
/// A normative list of *names the specification owns*, compared against the name of the definition
/// a view usage's typing settled to -- not against the text the author wrote. A workspace's own
/// `view def` is never in this list and is never reported: the rule is about reaching for a library
/// view definition the specification does not define.
pub(crate) const STANDARD_VIEW_DEFINITIONS: &[&str] = &[
    "ActionFlowView",
    "BrowserView",
    "GeneralView",
    "GeometryView",
    "GridView",
    "InterconnectionView",
    "SequenceView",
    "StateTransitionView",
];

/// Whether a declaration kind is a package-like namespace.
///
/// The narrow sense: what a qualified name addresses as a scope of its own. Used by the
/// name-collision rule, where a package's definitions share one identity domain.
pub(crate) fn is_namespace_kind(kind: DeclarationKind) -> bool {
    matches!(
        kind,
        DeclarationKind::Namespace | DeclarationKind::Package | DeclarationKind::LibraryPackage
    )
}

/// Whether a declaration kind states a state, either half of the family.
///
/// The two authored halves only. A final pseudo-state is a state a transition may name but not a
/// context that owns one, which is why the endpoint rules use [`is_state_endpoint_kind`] instead.
pub(crate) fn is_state_kind(kind: DeclarationKind) -> bool {
    matches!(
        kind,
        DeclarationKind::StateDefinition | DeclarationKind::StateUsage
    )
}

/// Whether a declaration kind is something a transition or initial marker may name.
///
/// A `final <name>;` declares a final pseudo-state, so a transition into it is the authored way to
/// terminate a machine.
pub(crate) fn is_state_endpoint_kind(kind: DeclarationKind) -> bool {
    is_state_kind(kind) || kind == DeclarationKind::FinalState
}

/// Whether a declaration kind states behavior a `perform` or a succession can name.
///
/// The control nodes are included because a succession legitimately sequences them: `first decide
/// d then merge m;` relates two nodes that are steps, not action usages.
pub(crate) fn is_action_kind(kind: DeclarationKind) -> bool {
    matches!(
        kind,
        DeclarationKind::ActionDefinition
            | DeclarationKind::ActionUsage
            | DeclarationKind::AcceptActionUsage
            | DeclarationKind::SendActionUsage
            | DeclarationKind::TerminateActionUsage
            | DeclarationKind::PerformActionUsage
            | DeclarationKind::Decide
            | DeclarationKind::Merge
            | DeclarationKind::Fork
            | DeclarationKind::Join
            | DeclarationKind::If
            | DeclarationKind::While
            | DeclarationKind::Loop
            | DeclarationKind::ForLoop
            | DeclarationKind::KermlBehavior
            | DeclarationKind::KermlStep
    )
}

/// Whether a declaration kind states a requirement a `satisfy` or a `verify` can name.
pub(crate) fn is_requirement_kind(kind: DeclarationKind) -> bool {
    matches!(
        kind,
        DeclarationKind::RequirementDefinition
            | DeclarationKind::RequirementUsage
            | DeclarationKind::VerifyRequirement
    )
}

pub(crate) fn is_viewpoint_kind(kind: DeclarationKind) -> bool {
    matches!(
        kind,
        DeclarationKind::ViewpointDefinition | DeclarationKind::ViewpointUsage
    )
}

pub(crate) fn is_view_kind(kind: DeclarationKind) -> bool {
    matches!(
        kind,
        DeclarationKind::ViewDefinition | DeclarationKind::ViewUsage
    )
}

pub(crate) fn is_use_case_kind(kind: DeclarationKind) -> bool {
    matches!(
        kind,
        DeclarationKind::UseCaseDefinition | DeclarationKind::UseCaseUsage
    )
}

/// Whether a declaration kind carries a subject and other input role members.
pub(crate) fn supports_subject_role(kind: DeclarationKind) -> bool {
    matches!(
        kind,
        DeclarationKind::RequirementDefinition
            | DeclarationKind::RequirementUsage
            | DeclarationKind::ConcernDefinition
            | DeclarationKind::ConcernUsage
            | DeclarationKind::CaseDefinition
            | DeclarationKind::CaseUsage
            | DeclarationKind::AnalysisCaseDefinition
            | DeclarationKind::AnalysisCaseUsage
            | DeclarationKind::VerificationCaseDefinition
            | DeclarationKind::VerificationCaseUsage
            | DeclarationKind::UseCaseDefinition
            | DeclarationKind::UseCaseUsage
            | DeclarationKind::ViewpointDefinition
            | DeclarationKind::ViewpointUsage
    )
}

/// Whether a declaration kind is an input role member of the declaration that owns it.
pub(crate) fn is_input_role_member(kind: DeclarationKind) -> bool {
    matches!(
        kind,
        DeclarationKind::SubjectUsage
            | DeclarationKind::RequirementActor
            | DeclarationKind::CaseActor
            | DeclarationKind::StakeholderUsage
    )
}

/// The reference families a SysML `connect` states its ends with.
///
/// `connect a to b;` names each end directly; `connect a.fill to b.fill;` names it through a
/// member access, which the resolver settles to the same declaration. Reading only the first would
/// leave every dotted connection unchecked, which is the form real models are written in.
pub(crate) const CONNECTOR_END_KINDS: &[ReferenceKind] = &[
    ReferenceKind::ConnectorEnd,
    ReferenceKind::MemberAccessOperand,
];

/// Whether a declaration is a SysML `connect`, whose ends must be connectable structure.
///
/// `KermlConnector` is deliberately absent. KerML relates any two features -- the Kernel library's
/// own `connector c1 from a to b;` relates two plain features -- so the SysML rule that a
/// connection joins ports or structural parts states nothing about it.
pub(crate) fn is_connector_kind(kind: DeclarationKind) -> bool {
    matches!(
        kind,
        DeclarationKind::ConnectionUsage
            | DeclarationKind::InterfaceUsage
            | DeclarationKind::BareConnect
    )
}

/// Whether a declaration is a SysML behavior body, where a succession sequences actions.
///
/// A succession written anywhere else -- a KerML `behavior`, whose `succession [1] ifTest then
/// [0..1] thenClause;` sequences occurrences -- states a different relationship, and the SysML
/// action rule says nothing about it.
pub(crate) fn is_action_body_kind(kind: DeclarationKind) -> bool {
    matches!(
        kind,
        DeclarationKind::ActionDefinition
            | DeclarationKind::ActionUsage
            | DeclarationKind::AcceptActionUsage
            | DeclarationKind::SendActionUsage
            | DeclarationKind::TerminateActionUsage
            | DeclarationKind::PerformActionUsage
            | DeclarationKind::VerificationCaseDefinition
            | DeclarationKind::VerificationCaseUsage
    )
}

/// Whether two declarations owned by one namespace must be distinguishable by name.
///
/// KerML requires member names to be unique within a namespace, but the publication models kinds
/// the parser keeps apart -- a `doc`, an anonymous transition, a synthesized binding -- that never
/// competed for a name. Two members of the same kind always collide; two definitions collide inside
/// a package, where a definition is the thing a qualified name addresses.
pub(crate) fn names_must_be_distinguishable(
    owner: DeclarationKind,
    left: (Family, Role),
    right: (Family, Role),
) -> bool {
    left == right
        || (is_namespace_kind(owner) && left.1 == Role::Definition && right.1 == Role::Definition)
}

impl<D> SemanticModel<D> {
    /// Appends every host-reported conformance diagnostic authored in `document`.
    pub(crate) fn collect_host_conformance(
        &self,
        document: DocumentIdx,
        declared: &[DeclarationId],
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        self.collect_namespace_identity(declared, diagnostics)?;
        self.collect_connection_structure(declared, diagnostics)?;
        self.collect_behavior_structure(document, declared, diagnostics)?;
        self.collect_requirement_case_structure(declared, diagnostics)?;
        self.collect_view_structure(document, declared, diagnostics)?;
        self.collect_declaration_rules(document, declared, diagnostics)?;
        self.collect_inherited_value_rules(document, diagnostics)?;
        self.collect_analysis_status(declared, diagnostics)?;
        Ok(())
    }

    /// The declaration kind of one declaration, or `None` when storage does not hold it.
    pub(crate) fn kind_of(&self, id: DeclarationId) -> Option<DeclarationKind> {
        self.storage.declaration(id).map(|value| value.kind)
    }

    /// The authored name of one declaration, or `<anonymous>` when it has none.
    ///
    /// Only for message text. No rule branches on it.
    pub(crate) fn display_name(&self, id: DeclarationId) -> &str {
        self.storage
            .declaration(id)
            .and_then(|declaration| declaration.name)
            .and_then(|name| self.storage.symbol(name))
            .unwrap_or("<anonymous>")
    }

    /// The source range of one declaration, for ordering members by where they were written.
    ///
    /// `None` sorts first and means the range could not be mapped, which is a storage fault the
    /// caller's own range mapping reports; it is never a claim about authored order.
    pub(crate) fn declaration_range(&self, id: DeclarationId) -> Option<TextRange> {
        let declaration = self.storage.declaration(id)?;
        document_range(&self.storage, declaration.document, &declaration.span).ok()
    }

    /// The single settled target of one authored reference, if it settled to one.
    pub(crate) fn settled_target(&self, reference: AuthoredReferenceId) -> Option<DeclarationId> {
        match self.resolution.outcome(reference) {
            Some(ResolutionStatus::Resolved(target)) => Some(target),
            _ => None,
        }
    }

    /// The authored references of one declaration in a given family, with their identities.
    ///
    /// Implied references are excluded: every rule here judges what the author wrote.
    pub(crate) fn authored_references(
        &self,
        id: DeclarationId,
        kinds: &[ReferenceKind],
    ) -> Vec<(AuthoredReferenceId, &AuthoredReference)> {
        self.outgoing_reference_ids(id)
            .iter()
            .map(|reference_id| {
                (
                    *reference_id,
                    &self.storage.references[reference_id.index()],
                )
            })
            .filter(|(_, reference)| !reference.flags.implied && kinds.contains(&reference.kind))
            .collect()
    }

    /// The settled targets of one declaration's authored references in a given family.
    pub(crate) fn settled_targets(
        &self,
        id: DeclarationId,
        kinds: &[ReferenceKind],
    ) -> Vec<DeclarationId> {
        self.authored_references(id, kinds)
            .into_iter()
            .filter_map(|(reference_id, _)| self.settled_target(reference_id))
            .collect()
    }

    /// The nearest enclosing declaration of a given kind, following ownership.
    pub(crate) fn enclosing(
        &self,
        id: DeclarationId,
        accepts: impl Fn(DeclarationKind) -> bool,
    ) -> Option<DeclarationId> {
        let mut current = self.storage.declaration(id)?.owner;
        while let Some(owner) = current {
            let declaration = self.storage.declaration(owner)?;
            if accepts(declaration.kind) {
                return Some(owner);
            }
            current = declaration.owner;
        }
        None
    }

    /// One diagnostic reported at an authored reference, with the owner's own sentence.
    pub(crate) fn reference_message_diagnostic(
        &self,
        reference: &AuthoredReference,
        code: DiagnosticCode,
        severity: DiagnosticSeverity,
        message: String,
        related: Option<(DeclarationId, &str)>,
    ) -> Result<Diagnostic, ResolutionError> {
        let source = self
            .storage
            .declaration(reference.source)
            .ok_or(ResolutionError::InvalidStorage)?;
        Ok(Diagnostic {
            message: message.into_boxed_str(),
            payload: matches!(
                code,
                DiagnosticCode::UnresolvedTypeReference
                    | DiagnosticCode::UnresolvedSpecializesReference
                    | DiagnosticCode::UnresolvedImportTarget
                    | DiagnosticCode::UnresolvedReference
            )
            .then(
                || crate::diagnostics::DiagnosticPayload::UnresolvedReference {
                    authored_target: self.authored_path(reference.path).into_boxed_str(),
                },
            ),
            code,
            severity,
            origin: DiagnosticOrigin::Semantic,
            subject: self.symbol_id(reference.source),
            location: DiagnosticLocation {
                document: writer::document_identity(self, source.document).into(),
                range: document_range(&self.storage, source.document, &reference.span)?,
            },
            related: match related {
                Some((target, note)) => Box::from([self.related_declaration(target, note)?]),
                None => Box::default(),
            },
        })
    }

    // ------------------------------------------------------------------------------------------
    // Namespace identity
    // ------------------------------------------------------------------------------------------

    /// Reports a namespace that declares two members resolution cannot tell apart.
    ///
    /// Reported at the later member, with the first as related information, so the diagnostic
    /// points at the declaration that introduced the collision. Both the authored name and the
    /// authored short name are identities resolution addresses a member by, so both collide.
    pub(crate) fn collect_namespace_identity(
        &self,
        declared: &[DeclarationId],
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        for owner in declared.iter().copied() {
            let owner_kind = self.kind_of(owner).ok_or(ResolutionError::InvalidStorage)?;
            if !matches!(
                owner_kind,
                DeclarationKind::Namespace
                    | DeclarationKind::Package
                    | DeclarationKind::LibraryPackage
                    | DeclarationKind::PartDefinition
                    | DeclarationKind::RequirementDefinition
                    | DeclarationKind::UseCaseDefinition
            ) {
                continue;
            }
            let mut by_identifier: BTreeMap<&str, Vec<DeclarationId>> = BTreeMap::new();
            for child in self.child_declarations(owner) {
                let declaration = self
                    .storage
                    .declaration(*child)
                    .ok_or(ResolutionError::InvalidStorage)?;
                if matches!(
                    declaration.kind,
                    DeclarationKind::Import | DeclarationKind::Alias
                ) {
                    continue;
                }
                for name in [
                    declaration.name,
                    self.storage
                        .declaration_facts(*child)
                        .and_then(|facts| facts.short_name),
                ]
                .into_iter()
                .flatten()
                .filter_map(|name| self.storage.symbol(name))
                .filter(|name| !name.trim().is_empty())
                {
                    let entry = by_identifier.entry(name).or_default();
                    if !entry.contains(child) {
                        entry.push(*child);
                    }
                }
            }
            for (name, mut members) in by_identifier {
                if members.len() < 2 {
                    continue;
                }
                members.sort_by_key(|member| self.declaration_range(*member));
                let Some((first, duplicate)) = self.first_collision(owner_kind, &members) else {
                    continue;
                };
                let mut diagnostic = self.declaration_message_diagnostic(
                    duplicate,
                    DiagnosticCode::DuplicateNamespaceMember,
                    DiagnosticSeverity::Warning,
                    Some(format!(
                        "'{}' declares '{name}' more than once; member names must be unique \
                         within a namespace.",
                        self.display_name(owner)
                    )),
                )?;
                diagnostic.related =
                    Box::from([self.related_declaration(first, RELATED_FIRST_DECLARATION)?]);
                diagnostics.push(diagnostic);
            }
        }
        Ok(())
    }

    /// The first pair of same-named members whose kinds must be distinguishable.
    pub(crate) fn first_collision(
        &self,
        owner: DeclarationKind,
        members: &[DeclarationId],
    ) -> Option<(DeclarationId, DeclarationId)> {
        for (index, member) in members.iter().enumerate().skip(1) {
            let later = self.declaration_family(*member)?;
            for earlier_id in &members[..index] {
                let Some(earlier) = self.declaration_family(*earlier_id) else {
                    continue;
                };
                if names_must_be_distinguishable(owner, earlier, later) {
                    return Some((*earlier_id, *member));
                }
            }
        }
        None
    }

    // ------------------------------------------------------------------------------------------
    // Connection conformance
    // ------------------------------------------------------------------------------------------

    /// Reports connectors whose ends are not connectable, and ports that connect to nothing.
    pub(crate) fn collect_connection_structure(
        &self,
        declared: &[DeclarationId],
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        let mut connected_ends: BTreeSet<DeclarationId> = BTreeSet::new();
        for id in declared.iter().copied() {
            let declaration = self
                .storage
                .declaration(id)
                .ok_or(ResolutionError::InvalidStorage)?;
            if !is_connector_kind(declaration.kind) {
                continue;
            }
            let ends = self.authored_references(id, CONNECTOR_END_KINDS);
            let mut settled = Vec::with_capacity(ends.len());
            for (reference_id, reference) in &ends {
                let Some(target) = self.settled_target(*reference_id) else {
                    continue;
                };
                settled.push((target, *reference));
                connected_ends.insert(target);
            }
            if settled.len() < 2 {
                // One settled end states no relationship to judge; the other end is already its
                // own unresolved-reference diagnostic.
                continue;
            }
            let families = settled
                .iter()
                .map(|(target, _)| self.declaration_family(*target))
                .collect::<Vec<_>>();
            // A metaclass family this publication does not classify -- a `#keyword`-prefixed
            // usage, a KerML feature -- states nothing about connectability, so the kind rules
            // decline to answer rather than reporting the missing classification as a fault. The
            // duplicate-pair rule below is unaffected: it compares identities, not kinds.
            let classified = families.iter().all(Option::is_some);
            let any_port = families
                .iter()
                .any(|family| matches!(family, Some((Family::Port, _))));
            let all_structural = families.iter().all(|family| {
                matches!(
                    family,
                    Some((Family::Part, _))
                        | Some((Family::Item, _))
                        | Some((Family::Occurrence, _))
                )
            });
            if classified && !any_port && !all_structural {
                diagnostics.push(self.declaration_message_diagnostic(
                    id,
                    DiagnosticCode::ConnectionContextInvalid,
                    DiagnosticSeverity::Warning,
                    None,
                )?);
            } else if classified && any_port {
                // A mixed connector states a port on one end and something else on the other; the
                // non-port end is the one that cannot carry the connection.
                for ((target, reference), family) in settled.iter().zip(families.iter()) {
                    if matches!(family, Some((Family::Port, _))) {
                        continue;
                    }
                    diagnostics.push(self.reference_message_diagnostic(
                        reference,
                        DiagnosticCode::ConnectionEndpointNotPort,
                        DiagnosticSeverity::Warning,
                        format!(
                            "Connector end '{}' does not resolve to a port.",
                            self.display_name(*target)
                        ),
                        Some((*target, RELATED_TARGET)),
                    )?);
                }
            }

            // Two connected ports must be typed by related definitions. Only settled effective
            // types are compared: a port with no effective type has nothing to conform to.
            //
            // SysML 8.4.10.2: the ends of a typed interface usage redefine the corresponding ends
            // of its interface definition, so end conformance is stated there and the pairwise
            // question does not arise here.
            let redefines_declared_ends = declaration.kind == DeclarationKind::InterfaceUsage
                && !self
                    .settled_targets(id, &[ReferenceKind::FeatureTyping])
                    .is_empty();
            if !redefines_declared_ends
                && settled.len() == 2
                && families
                    .iter()
                    .all(|family| matches!(family, Some((Family::Port, _))))
            {
                let left = settled[0].0;
                let right = settled[1].0;
                let code = if self.types_are_unrelated(left, right)
                    && !self.ports_are_feature_compatible(left, right)
                {
                    Some(DiagnosticCode::PortTypeMismatch)
                } else if self.ports_mirror_direction(left, right) {
                    Some(DiagnosticCode::FlowDirectionIncompatible)
                } else {
                    None
                };
                if let Some(code) = code {
                    let mut diagnostic = self.declaration_message_diagnostic(
                        id,
                        code.clone(),
                        DiagnosticSeverity::Warning,
                        Some(format!(
                            "{} Connecting '{}' to '{}'.",
                            code.describe(),
                            self.display_name(left),
                            self.display_name(right)
                        )),
                    )?;
                    diagnostic.related = Box::from([
                        self.related_declaration(left, RELATED_OTHER_END)?,
                        self.related_declaration(right, RELATED_OTHER_END)?,
                    ]);
                    diagnostics.push(diagnostic);
                }
            }
        }

        self.collect_unconnected_ports(declared, &connected_ends, diagnostics)?;
        self.collect_interface_ends(declared, diagnostics)?;
        self.collect_binding_connectors(declared, diagnostics)?;
        Ok(())
    }

    /// Reports ports that no connector, flow or binding names.
    ///
    /// A port that redefines or subsets another states a refinement of a connected feature rather
    /// than a new endpoint, so it is not reported: the feature it specializes carries the
    /// connection.
    pub(crate) fn collect_unconnected_ports(
        &self,
        declared: &[DeclarationId],
        connected: &BTreeSet<DeclarationId>,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        for id in declared.iter().copied() {
            if self.kind_of(id) != Some(DeclarationKind::PortUsage) {
                continue;
            }
            if connected.contains(&id) {
                continue;
            }
            if !self
                .authored_references(
                    id,
                    &[
                        ReferenceKind::Redefinition,
                        ReferenceKind::Subsetting,
                        ReferenceKind::References,
                    ],
                )
                .is_empty()
            {
                continue;
            }
            // A port named by any endpoint-bearing reference is connected, whatever the spelling.
            if self.reverse_references.references(id).iter().any(|id| {
                matches!(
                    self.storage.references[id.index()].kind,
                    ReferenceKind::ConnectorEnd
                        | ReferenceKind::MemberAccessOperand
                        | ReferenceKind::FlowSource
                        | ReferenceKind::FlowTarget
                        | ReferenceKind::BindSource
                        | ReferenceKind::BindTarget
                )
            }) {
                continue;
            }
            diagnostics.push(self.declaration_message_diagnostic(
                id,
                DiagnosticCode::UnconnectedPort,
                DiagnosticSeverity::Information,
                Some(format!(
                    "Port '{}' takes part in no connection.",
                    self.display_name(id)
                )),
            )?);
        }
        Ok(())
    }

    /// Reports interface ends that declare no port type.
    ///
    /// An end that references another feature (`end e ::> p;`) inherits its type from what it
    /// references, so it is not required to declare one.
    pub(crate) fn collect_interface_ends(
        &self,
        declared: &[DeclarationId],
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        for id in declared.iter().copied() {
            let Some(owner) = self.storage.declaration(id).and_then(|value| value.owner) else {
                continue;
            };
            if self.kind_of(owner) != Some(DeclarationKind::InterfaceDefinition) {
                continue;
            }
            if self
                .storage
                .declaration_facts(id)
                .is_none_or(|facts| facts.positional_end.is_none())
            {
                continue;
            }
            if !self
                .authored_references(
                    id,
                    &[ReferenceKind::FeatureTyping, ReferenceKind::References],
                )
                .is_empty()
            {
                continue;
            }
            diagnostics.push(self.declaration_message_diagnostic(
                id,
                DiagnosticCode::InterfaceEndInvalid,
                DiagnosticSeverity::Warning,
                Some(format!(
                    "Interface end '{}' declares no port type.",
                    self.display_name(id)
                )),
            )?);
        }
        Ok(())
    }

    /// Reports a binding connector whose two ends have unrelated effective types.
    pub(crate) fn collect_binding_connectors(
        &self,
        declared: &[DeclarationId],
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        for id in declared.iter().copied() {
            if !matches!(
                self.kind_of(id),
                Some(DeclarationKind::Bind) | Some(DeclarationKind::KermlBinding)
            ) {
                continue;
            }
            let source = self.settled_targets(id, &[ReferenceKind::BindSource]);
            let target = self.settled_targets(id, &[ReferenceKind::BindTarget]);
            let (Some(source), Some(target)) = (source.first(), target.first()) else {
                continue;
            };
            if !self.types_are_unrelated(*source, *target) {
                continue;
            }
            let mut diagnostic = self.declaration_message_diagnostic(
                id,
                DiagnosticCode::BindingConnectorIncompatible,
                DiagnosticSeverity::Warning,
                Some(format!(
                    "Binding connector binds '{}' to '{}', whose types are unrelated.",
                    self.display_name(*source),
                    self.display_name(*target)
                )),
            )?;
            diagnostic.related = Box::from([
                self.related_declaration(*source, RELATED_OTHER_END)?,
                self.related_declaration(*target, RELATED_OTHER_END)?,
            ]);
            diagnostics.push(diagnostic);
        }
        Ok(())
    }

    /// Whether two port types offer each other the same features.
    ///
    /// SysML connects ports whose *definitions* are unrelated all the time -- a spigot's `out item
    /// water` meets an inlet's `in item water`, and neither definition specializes the other. What
    /// makes the connection valid is that each end offers the feature the other expects, so this
    /// compares the feature sets rather than the definitions.
    ///
    /// A port with no features has nothing to match, so the question falls back to the definitions.
    pub(crate) fn ports_are_feature_compatible(
        &self,
        left: DeclarationId,
        right: DeclarationId,
    ) -> bool {
        let left_features = self.port_features(left);
        let right_features = self.port_features(right);
        if left_features.is_empty() || right_features.is_empty() {
            return false;
        }
        if left_features.keys().ne(right_features.keys()) {
            return false;
        }
        left_features.iter().all(|(name, left_types)| {
            right_features.get(name).is_some_and(|right_types| {
                left_types.is_empty()
                    || right_types.is_empty()
                    || left_types.iter().any(|left| {
                        right_types.iter().any(|right| {
                            self.conformance(*left, *right, SpecializationScope::AnySpecialization)
                                == Conformance::Conforms
                                || self.conformance(
                                    *right,
                                    *left,
                                    SpecializationScope::AnySpecialization,
                                ) == Conformance::Conforms
                        })
                    })
            })
        })
    }

    /// The named features a port offers, with the effective types of each.
    ///
    /// Read through the port's effective types and their specializations, so a port typed by a
    /// definition that inherits its features offers them too.
    pub(crate) fn port_features(
        &self,
        port: DeclarationId,
    ) -> BTreeMap<Box<str>, Vec<DeclarationId>> {
        let mut features: BTreeMap<Box<str>, Vec<DeclarationId>> = BTreeMap::new();
        let mut owners = self
            .types
            .effective_types(port)
            .iter()
            .map(|(target, _)| *target)
            .collect::<Vec<_>>();
        let direct = owners.clone();
        for owner in direct {
            owners.extend(
                self.types
                    .supertypes(owner)
                    .iter()
                    .map(|(target, _)| *target),
            );
        }
        for owner in owners {
            for child in self.child_declarations(owner) {
                let Some(name) = self
                    .storage
                    .declaration(*child)
                    .and_then(|declaration| declaration.name)
                    .and_then(|name| self.storage.symbol(name))
                else {
                    continue;
                };
                let types = self
                    .types
                    .effective_types(*child)
                    .iter()
                    .map(|(target, _)| *target)
                    .collect::<Vec<_>>();
                features.entry(name.into()).or_insert(types);
            }
        }
        features
    }

    /// Whether two connected ports present the same direction to each other.
    ///
    /// SysML conjugation is what makes a connection carry anything: one end offers `in x` and the
    /// other, conjugated, offers it as `out x`. Two ends of comparable types with the *same*
    /// conjugation therefore mirror each other, and nothing flows.
    ///
    /// Decided from the conjugation flag on each end's authored typing and the directions its type
    /// declares, both settled facts. A type that declares no directed feature carries no direction
    /// to mirror, so the question does not arise; the legacy check answered it from the spelling of
    /// the authored type reference instead.
    pub(crate) fn ports_mirror_direction(&self, left: DeclarationId, right: DeclarationId) -> bool {
        if self.types_are_unrelated(left, right) {
            return false;
        }
        if self.port_is_conjugated(left) != self.port_is_conjugated(right) {
            return false;
        }
        self.types
            .effective_types(left)
            .iter()
            .any(|(target, _)| self.declares_a_directed_feature(*target))
    }

    /// Whether a port's authored typing conjugates the definition it names (`port p : ~PD;`).
    pub(crate) fn port_is_conjugated(&self, port: DeclarationId) -> bool {
        self.authored_references(port, &[ReferenceKind::FeatureTyping])
            .iter()
            .any(|(_, reference)| reference.flags.conjugated)
    }

    /// Whether a type declares at least one member with an authored direction.
    pub(crate) fn declares_a_directed_feature(&self, type_id: DeclarationId) -> bool {
        self.child_declarations(type_id).iter().any(|child| {
            self.storage
                .declaration_facts(*child)
                .is_some_and(|facts| facts.direction.is_some())
        })
    }

    /// Whether two features both have effective types and no pair of them is comparable.
    ///
    /// Answers `false` when either side has no effective type: that is a question this publication
    /// cannot answer, and answering it as "unrelated" would report the absence of a type as a
    /// mismatch. Comparability rather than conformance in one direction, for the same reason the
    /// value rules use it: either side may legitimately be the narrower one.
    pub(crate) fn types_are_unrelated(&self, left: DeclarationId, right: DeclarationId) -> bool {
        let left_types = self.types.effective_types(left);
        let right_types = self.types.effective_types(right);
        if left_types.is_empty() || right_types.is_empty() {
            return false;
        }
        !left_types.iter().any(|(left, _)| {
            right_types.iter().any(|(right, _)| {
                self.conformance(*left, *right, SpecializationScope::AnySpecialization)
                    == Conformance::Conforms
                    || self.conformance(*right, *left, SpecializationScope::AnySpecialization)
                        == Conformance::Conforms
            })
        })
    }

    // ------------------------------------------------------------------------------------------
    // Behavior conformance
    // ------------------------------------------------------------------------------------------

    pub(crate) fn collect_behavior_structure(
        &self,
        document: DocumentIdx,
        declared: &[DeclarationId],
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        let mut initial_by_context: BTreeMap<DeclarationId, Vec<DeclarationId>> = BTreeMap::new();
        let mut final_by_context: BTreeMap<DeclarationId, Vec<DeclarationId>> = BTreeMap::new();

        for id in declared.iter().copied() {
            let kind = self.kind_of(id).ok_or(ResolutionError::InvalidStorage)?;
            match kind {
                DeclarationKind::PerformActionUsage => {
                    for (reference_id, reference) in self.authored_references(
                        id,
                        &[ReferenceKind::FeatureTyping, ReferenceKind::References],
                    ) {
                        let Some(target) = self.settled_target(reference_id) else {
                            continue;
                        };
                        let Some(target_kind) = self.kind_of(target) else {
                            continue;
                        };
                        // A directed parameter is undecidable here: the pinned parser consumes
                        // the kind keyword of `in action body { ... }` (`Actions.sysml`) without
                        // recording it, so an action parameter and a value parameter reach
                        // semantics as the same `ParameterUsage` (planning/UPSTREAM_PARSER_GAPS.md,
                        // gap 82). Reporting it would name a defect the author did not write.
                        if is_action_kind(target_kind)
                            || target_kind == DeclarationKind::ParameterUsage
                        {
                            continue;
                        }
                        diagnostics.push(self.reference_message_diagnostic(
                            reference,
                            DiagnosticCode::PerformTargetInvalidKind,
                            DiagnosticSeverity::Warning,
                            format!(
                                "Performed behavior '{}' does not resolve to an action.",
                                self.display_name(target)
                            ),
                            Some((target, RELATED_TARGET)),
                        )?);
                    }
                }
                DeclarationKind::Transition => {
                    self.collect_transition(id, diagnostics)?;
                }
                DeclarationKind::InitialState => {
                    for (reference_id, reference) in
                        self.authored_references(id, &[ReferenceKind::InitialState])
                    {
                        let Some(target) = self.settled_target(reference_id) else {
                            continue;
                        };
                        let Some(target_kind) = self.kind_of(target) else {
                            continue;
                        };
                        if is_state_endpoint_kind(target_kind) {
                            continue;
                        }
                        diagnostics.push(self.reference_message_diagnostic(
                            reference,
                            DiagnosticCode::InitialStateInvalidTarget,
                            DiagnosticSeverity::Warning,
                            format!(
                                "Initial transition target '{}' does not resolve to a state.",
                                self.display_name(target)
                            ),
                            Some((target, RELATED_TARGET)),
                        )?);
                    }
                    if let Some(context) = self.enclosing(id, is_state_kind) {
                        initial_by_context.entry(context).or_default().push(id);
                    }
                }
                DeclarationKind::FinalState => {
                    if let Some(context) = self.enclosing(id, is_state_kind) {
                        final_by_context.entry(context).or_default().push(id);
                    }
                }
                DeclarationKind::Succession => {
                    if !self
                        .storage
                        .declaration(id)
                        .and_then(|declaration| declaration.owner)
                        .and_then(|owner| self.kind_of(owner))
                        .is_some_and(is_action_body_kind)
                    {
                        continue;
                    }
                    for (reference_id, reference) in
                        self.authored_references(id, &[ReferenceKind::Succession])
                    {
                        let Some(target) = self.settled_target(reference_id) else {
                            continue;
                        };
                        let Some(target_kind) = self.kind_of(target) else {
                            continue;
                        };
                        // A parameter carries an item between actions rather than being one, so a
                        // succession naming one sequences nothing this rule can judge.
                        if is_action_kind(target_kind)
                            || target_kind == DeclarationKind::ParameterUsage
                        {
                            continue;
                        }
                        diagnostics.push(self.reference_message_diagnostic(
                            reference,
                            DiagnosticCode::SuccessionEndpointInvalid,
                            DiagnosticSeverity::Warning,
                            format!(
                                "Succession endpoint '{}' does not resolve to an action.",
                                self.display_name(target)
                            ),
                            Some((target, RELATED_TARGET)),
                        )?);
                    }
                }
                _ => {}
            }

            for (reference_id, reference) in
                self.authored_references(id, &[ReferenceKind::AcceptPayloadType])
            {
                let Some(target) = self.settled_target(reference_id) else {
                    continue;
                };
                let Some((family, _)) = self.declaration_family(target) else {
                    continue;
                };
                if matches!(
                    family,
                    Family::Item | Family::Part | Family::Attribute | Family::Occurrence
                ) {
                    continue;
                }
                diagnostics.push(self.reference_message_diagnostic(
                    reference,
                    DiagnosticCode::AcceptPayloadIncompatible,
                    DiagnosticSeverity::Warning,
                    format!(
                        "Accept payload type '{}' cannot carry an item.",
                        self.display_name(target)
                    ),
                    Some((target, RELATED_TARGET)),
                )?);
            }
        }

        self.collect_state_machine_shape(
            document,
            declared,
            &initial_by_context,
            &final_by_context,
            diagnostics,
        )?;
        Ok(())
    }

    /// The two endpoint rules and the guard rule of one transition.
    pub(crate) fn collect_transition(
        &self,
        id: DeclarationId,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        let source = self.settled_targets(id, &[ReferenceKind::TransitionSource]);
        let target = self.settled_targets(id, &[ReferenceKind::TransitionTarget]);
        // A guard settles through the same expression pipeline every other condition does, so a
        // non-Boolean constant is the same fact a non-Boolean constraint is.
        if let Some(value) = self.evaluation_for(id).value() {
            if !matches!(value, EvaluatedScalar::Boolean(_)) {
                diagnostics.push(self.declaration_diagnostic(
                    id,
                    DiagnosticCode::TransitionGuardNonBoolean,
                    DiagnosticSeverity::Warning,
                )?);
            }
        }
        let (Some(source), Some(target)) = (source.first(), target.first()) else {
            return Ok(());
        };
        let (Some(source_kind), Some(target_kind)) = (self.kind_of(*source), self.kind_of(*target))
        else {
            return Ok(());
        };
        if !is_state_endpoint_kind(source_kind) || !is_state_endpoint_kind(target_kind) {
            let mut diagnostic = self.declaration_message_diagnostic(
                id,
                DiagnosticCode::TransitionEndpointInvalidState,
                DiagnosticSeverity::Warning,
                Some(format!(
                    "Transition endpoints '{}' and '{}' do not both resolve to states.",
                    self.display_name(*source),
                    self.display_name(*target)
                )),
            )?;
            diagnostic.related = Box::from([
                self.related_declaration(*source, RELATED_OTHER_END)?,
                self.related_declaration(*target, RELATED_OTHER_END)?,
            ]);
            diagnostics.push(diagnostic);
            return Ok(());
        }
        let source_context = self.enclosing(*source, |kind| {
            kind == DeclarationKind::StateDefinition || kind == DeclarationKind::StateUsage
        });
        let target_context = self.enclosing(*target, |kind| {
            kind == DeclarationKind::StateDefinition || kind == DeclarationKind::StateUsage
        });
        if let (Some(source_context), Some(target_context)) = (source_context, target_context) {
            if source_context != target_context {
                let mut diagnostic = self.declaration_diagnostic(
                    id,
                    DiagnosticCode::TransitionEndpointInvalidContext,
                    DiagnosticSeverity::Warning,
                )?;
                diagnostic.related = Box::from([
                    self.related_declaration(source_context, RELATED_OTHER_END)?,
                    self.related_declaration(target_context, RELATED_OTHER_END)?,
                ]);
                diagnostics.push(diagnostic);
            }
        }
        Ok(())
    }

    /// The cardinality and completeness rules of each state definition in `document`.
    pub(crate) fn collect_state_machine_shape(
        &self,
        document: DocumentIdx,
        declared: &[DeclarationId],
        initial_by_context: &BTreeMap<DeclarationId, Vec<DeclarationId>>,
        final_by_context: &BTreeMap<DeclarationId, Vec<DeclarationId>>,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        // The cardinality of initial transitions is deliberately not reported. The pinned parser
        // gives a bare `then <state>;` initial marker and a `then <state>;` continuation after a
        // nested state the same shape, so both lower as an initial-state declaration, and counting
        // them would report an ordinary continuation as a second initial transition. Distinguishing
        // them needs a parser fact this publication does not have.
        for (context, markers) in final_by_context {
            if markers.len() < 2
                || self
                    .storage
                    .declaration(*context)
                    .ok_or(ResolutionError::InvalidStorage)?
                    .document
                    != document
            {
                continue;
            }
            diagnostics.push(self.declaration_message_diagnostic(
                *context,
                DiagnosticCode::MultipleFinalStates,
                DiagnosticSeverity::Warning,
                Some(format!(
                    "State definition '{}' declares {} final states; one is expected.",
                    self.display_name(*context),
                    markers.len()
                )),
            )?);
        }

        for id in declared.iter().copied() {
            if self.kind_of(id) != Some(DeclarationKind::StateDefinition) {
                continue;
            }
            let states = self
                .child_declarations(id)
                .iter()
                .filter(|child| self.kind_of(**child) == Some(DeclarationKind::StateUsage))
                .count();
            if states == 0 {
                continue;
            }
            if !initial_by_context.contains_key(&id) {
                diagnostics.push(self.declaration_message_diagnostic(
                    id,
                    DiagnosticCode::MissingInitialState,
                    DiagnosticSeverity::Information,
                    Some(format!(
                        "State definition '{}' owns states but declares no initial transition.",
                        self.display_name(id)
                    )),
                )?);
            }
            // A machine whose transitions form a cycle has no terminating path by construction,
            // so the absence of a final state states its shape rather than an omission.
            if !final_by_context.contains_key(&id) && !self.state_transitions_are_cyclic(id) {
                diagnostics.push(self.declaration_message_diagnostic(
                    id,
                    DiagnosticCode::MissingFinalState,
                    DiagnosticSeverity::Information,
                    Some(format!(
                        "State definition '{}' owns states but declares no finality indicator.",
                        self.display_name(id)
                    )),
                )?);
            }
        }
        Ok(())
    }

    /// Whether the settled transitions owned by one state definition form a cycle.
    ///
    /// Over settled endpoints, not authored expressions: a transition whose ends did not resolve
    /// contributes no edge, which is the same answer as having no transition at all.
    pub(crate) fn state_transitions_are_cyclic(&self, state_definition: DeclarationId) -> bool {
        let mut edges: BTreeMap<DeclarationId, Vec<DeclarationId>> = BTreeMap::new();
        for child in self.child_declarations(state_definition) {
            if self.kind_of(*child) != Some(DeclarationKind::Transition) {
                continue;
            }
            let source = self.settled_targets(*child, &[ReferenceKind::TransitionSource]);
            let target = self.settled_targets(*child, &[ReferenceKind::TransitionTarget]);
            if let (Some(source), Some(target)) = (source.first(), target.first()) {
                edges.entry(*source).or_default().push(*target);
            }
        }
        let mut visited = BTreeSet::new();
        let mut stack = BTreeSet::new();
        edges
            .keys()
            .any(|start| reaches_cycle(*start, &edges, &mut visited, &mut stack))
    }

    // ------------------------------------------------------------------------------------------
    // Requirement and case conformance
    // ------------------------------------------------------------------------------------------

    pub(crate) fn collect_requirement_case_structure(
        &self,
        declared: &[DeclarationId],
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        for id in declared.iter().copied() {
            let kind = self.kind_of(id).ok_or(ResolutionError::InvalidStorage)?;
            if supports_subject_role(kind) {
                self.collect_subject_roles(id, diagnostics)?;
            }
            match kind {
                DeclarationKind::Satisfy => self.collect_satisfy(id, diagnostics)?,
                // A `verify` membership owned by anything but a verification case is deliberately
                // not reported here: the parser accepts an `objective { ... }` body only inside a
                // verification case, so the membership cannot be authored anywhere else and the
                // rule would be a code that can never fire.
                DeclarationKind::VerifyRequirement => {
                    self.collect_target_kind(
                        id,
                        &[ReferenceKind::VerifyRequirementTarget],
                        is_requirement_kind,
                        DiagnosticCode::VerifiedRequirementInvalidTarget,
                        diagnostics,
                    )?;
                }
                _ => {}
            }
            self.collect_target_kind(
                id,
                &[ReferenceKind::IncludeUseCase],
                is_use_case_kind,
                DiagnosticCode::UseCaseIncludeInvalidTarget,
                diagnostics,
            )?;
        }
        Ok(())
    }

    /// Reports a reference family whose settled target is of the wrong kind.
    pub(crate) fn collect_target_kind(
        &self,
        id: DeclarationId,
        kinds: &[ReferenceKind],
        accepts: impl Fn(DeclarationKind) -> bool,
        code: DiagnosticCode,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        for (reference_id, reference) in self.authored_references(id, kinds) {
            let Some(target) = self.settled_target(reference_id) else {
                continue;
            };
            let Some(target_kind) = self.kind_of(target) else {
                continue;
            };
            if accepts(target_kind) {
                continue;
            }
            diagnostics.push(self.reference_message_diagnostic(
                reference,
                code.clone(),
                DiagnosticSeverity::Warning,
                format!(
                    "{} It resolved to '{}'.",
                    code.describe(),
                    self.display_name(target)
                ),
                Some((target, RELATED_TARGET)),
            )?);
        }
        Ok(())
    }

    /// The subject-cardinality and subject-order rules of one declaration.
    pub(crate) fn collect_subject_roles(
        &self,
        id: DeclarationId,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        let mut roles = self
            .child_declarations(id)
            .iter()
            .filter_map(|child| {
                let declaration = self.storage.declaration(*child)?;
                is_input_role_member(declaration.kind).then_some((*child, declaration.kind))
            })
            .collect::<Vec<_>>();
        roles.sort_by_key(|(child, _)| self.declaration_range(*child));
        let subjects = roles
            .iter()
            .filter(|(_, kind)| *kind == DeclarationKind::SubjectUsage)
            .map(|(child, _)| *child)
            .collect::<Vec<_>>();
        if subjects.len() > 1 {
            let mut diagnostic = self.declaration_message_diagnostic(
                subjects[1],
                DiagnosticCode::DuplicateRoleMember,
                DiagnosticSeverity::Warning,
                Some(format!(
                    "'{}' declares more than one subject member.",
                    self.display_name(id)
                )),
            )?;
            diagnostic.related =
                Box::from([self.related_declaration(subjects[0], RELATED_FIRST_DECLARATION)?]);
            diagnostics.push(diagnostic);
        }
        if let (Some(first), Some(subject)) = (roles.first(), subjects.first()) {
            if first.1 != DeclarationKind::SubjectUsage {
                let mut diagnostic = self.declaration_message_diagnostic(
                    *subject,
                    DiagnosticCode::SubjectMemberNotFirst,
                    DiagnosticSeverity::Warning,
                    Some(format!(
                        "Subject member of '{}' must precede its other input role members.",
                        self.display_name(id)
                    )),
                )?;
                diagnostic.related =
                    Box::from([self.related_declaration(first.0, RELATED_FIRST_DECLARATION)?]);
                diagnostics.push(diagnostic);
            }
        }
        Ok(())
    }

    /// The endpoint-kind rule of one satisfy relationship.
    ///
    /// A view satisfies a viewpoint and everything else satisfies a requirement; those are two
    /// different rules with two different codes, decided by what owns the relationship rather than
    /// by what it named.
    pub(crate) fn collect_satisfy(
        &self,
        id: DeclarationId,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        let owner_is_view = self
            .storage
            .declaration(id)
            .and_then(|declaration| declaration.owner)
            .and_then(|owner| self.kind_of(owner))
            .is_some_and(is_view_kind);
        let satisfied_is_view = self
            .settled_targets(id, &[ReferenceKind::SatisfySource])
            .first()
            .and_then(|target| self.kind_of(*target))
            .is_some_and(is_view_kind);
        // Two authored forms of viewpoint conformance. `satisfy <viewpoint>;` inside a view body
        // names the viewpoint as the satisfied thing; `satisfy <view> by <viewpoint>;` names the
        // view as the satisfied thing and the viewpoint in the `by` clause. The conforming target
        // is whichever operand is not the view.
        match (owner_is_view, satisfied_is_view) {
            (_, true) => self.collect_target_kind(
                id,
                &[ReferenceKind::SatisfyTarget],
                is_viewpoint_kind,
                DiagnosticCode::ViewpointConformanceInvalidTargetKind,
                diagnostics,
            ),
            (true, false) => self.collect_target_kind(
                id,
                &[ReferenceKind::SatisfySource],
                is_viewpoint_kind,
                DiagnosticCode::ViewpointConformanceInvalidTargetKind,
                diagnostics,
            ),
            (false, false) => self.collect_target_kind(
                id,
                &[ReferenceKind::SatisfySource],
                |kind| is_requirement_kind(kind) || is_viewpoint_kind(kind),
                DiagnosticCode::SatisfyInvalidEndpointKind,
                diagnostics,
            ),
        }
    }

    // ------------------------------------------------------------------------------------------
    // View conformance
    // ------------------------------------------------------------------------------------------

    pub(crate) fn collect_view_structure(
        &self,
        document: DocumentIdx,
        declared: &[DeclarationId],
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        for id in declared.iter().copied() {
            match self.kind_of(id) {
                Some(DeclarationKind::ViewUsage) => {
                    // A view with members that exposes nothing renders nothing. Reported as
                    // information: it is legal, and a view under construction passes through it.
                    let members = self.child_declarations(id);
                    if !members.is_empty()
                        && !members
                            .iter()
                            .any(|child| self.kind_of(*child) == Some(DeclarationKind::Expose))
                    {
                        diagnostics.push(self.declaration_message_diagnostic(
                            id,
                            DiagnosticCode::ViewExposeEmpty,
                            DiagnosticSeverity::Information,
                            Some(format!(
                                "View '{}' declares a body but exposes no members.",
                                self.display_name(id)
                            )),
                        )?);
                    }
                    for (reference_id, reference) in
                        self.authored_references(id, &[ReferenceKind::FeatureTyping])
                    {
                        let Some(target) = self.settled_target(reference_id) else {
                            continue;
                        };
                        if !self.is_non_standard_library_view(target)? {
                            continue;
                        }
                        diagnostics.push(self.reference_message_diagnostic(
                            reference,
                            DiagnosticCode::ViewTypeNonStandard,
                            DiagnosticSeverity::Warning,
                            format!(
                                "View type '{}' is not one of the SysML v2 standard view \
                                 definitions (§9.2.20 Table 34).",
                                self.display_name(target)
                            ),
                            Some((target, RELATED_TARGET)),
                        )?);
                    }
                }
                Some(DeclarationKind::RenderingUsage) => self.collect_target_kind(
                    id,
                    &[ReferenceKind::FeatureTyping],
                    |kind| {
                        matches!(
                            kind,
                            DeclarationKind::RenderingDefinition | DeclarationKind::RenderingUsage
                        )
                    },
                    DiagnosticCode::ViewRenderingInvalidTarget,
                    diagnostics,
                )?,
                _ => {}
            }
        }

        for record in self.storage.documentation.iter() {
            if record.form != AnnotationForm::TextualRepresentation || record.language.is_some() {
                continue;
            }
            let declaration = self
                .storage
                .declaration(record.declaration)
                .ok_or(ResolutionError::InvalidStorage)?;
            if declaration.document != document {
                continue;
            }
            diagnostics.push(Diagnostic {
                payload: None,
                message: DiagnosticCode::ViewpointRepLanguageUnresolved
                    .describe()
                    .into(),
                code: DiagnosticCode::ViewpointRepLanguageUnresolved,
                severity: DiagnosticSeverity::Warning,
                origin: DiagnosticOrigin::Semantic,
                subject: self.symbol_id(record.declaration),
                location: DiagnosticLocation {
                    document: writer::document_identity(self, document).into(),
                    range: document_range(&self.storage, document, &record.span)?,
                },
                related: Box::default(),
            });
        }
        Ok(())
    }

    /// Whether a view usage's settled type is a library view definition outside the standard set.
    ///
    /// A workspace's own `view def` is its author's to define, so only a definition admitted from a
    /// library is judged against the specification's table.
    pub(crate) fn is_non_standard_library_view(
        &self,
        target: DeclarationId,
    ) -> Result<bool, ResolutionError> {
        let declaration = self
            .storage
            .declaration(target)
            .ok_or(ResolutionError::InvalidStorage)?;
        if declaration.kind != DeclarationKind::ViewDefinition {
            return Ok(false);
        }
        let role = self
            .storage
            .document(declaration.document)
            .ok_or(ResolutionError::InvalidStorage)?
            .role;
        if role == source_identity::SourceRole::Workspace {
            return Ok(false);
        }
        let name = self.display_name(target);
        Ok(!STANDARD_VIEW_DEFINITIONS.contains(&name))
    }

    // ------------------------------------------------------------------------------------------
    // Declaration-local rules
    // ------------------------------------------------------------------------------------------

    pub(crate) fn collect_declaration_rules(
        &self,
        document: DocumentIdx,
        declared: &[DeclarationId],
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        for id in declared.iter().copied() {
            let kind = self.kind_of(id).ok_or(ResolutionError::InvalidStorage)?;

            // A multiplicity whose literal bounds cross admits nothing at all.
            if let Some(multiplicity) = self
                .storage
                .declaration_facts(id)
                .and_then(|facts| facts.multiplicity.as_ref())
            {
                if let (MultiplicityBound::Literal(lower), MultiplicityBound::Literal(upper)) =
                    (multiplicity.lower, multiplicity.upper)
                {
                    if lower < 0 || upper < lower {
                        diagnostics.push(Diagnostic {
                            payload: None,
                            message: DiagnosticCode::InvalidMultiplicity.describe().into(),
                            code: DiagnosticCode::InvalidMultiplicity,
                            severity: DiagnosticSeverity::Warning,
                            origin: DiagnosticOrigin::Semantic,
                            subject: self.symbol_id(id),
                            location: DiagnosticLocation {
                                document: writer::document_identity(self, document).into(),
                                range: document_range(&self.storage, document, &multiplicity.span)?,
                            },
                            related: Box::default(),
                        });
                    }
                }
            }

            // A `redefines` clause naming its own feature is deliberately not its own code: the
            // lexical lookup excludes the redefining feature, so `attribute value :>> value;`
            // settles as an unresolved reference, which is already published and points at the
            // same range.

            // A part usage with no typing, and nothing to inherit one from, has no definition
            // behind it. Reported as information: it is legal SysML.
            if kind == DeclarationKind::PartUsage
                && self
                    .authored_references(
                        id,
                        &[
                            ReferenceKind::FeatureTyping,
                            ReferenceKind::Subsetting,
                            ReferenceKind::Redefinition,
                            ReferenceKind::References,
                        ],
                    )
                    .is_empty()
                && self.types.effective_types(id).is_empty()
            {
                diagnostics.push(self.declaration_message_diagnostic(
                    id,
                    DiagnosticCode::UntypedPartUsage,
                    DiagnosticSeverity::Information,
                    Some(format!(
                        "Part '{}' declares no type.",
                        self.display_name(id)
                    )),
                )?);
            }

            if kind == DeclarationKind::Allocate {
                let source = self.authored_references(id, &[ReferenceKind::AllocateSource]);
                let target = self.authored_references(id, &[ReferenceKind::AllocateTarget]);
                if source.is_empty() != target.is_empty() {
                    diagnostics.push(self.declaration_diagnostic(
                        id,
                        DiagnosticCode::InvalidAllocationEndpoints,
                        DiagnosticSeverity::Warning,
                    )?);
                }
            }
        }

        // The typing of an allocation *usage* is deliberately not judged: this publication has no
        // allocation-usage declaration kind, so there is no declaration whose typing to read.
        Ok(())
    }

    // ------------------------------------------------------------------------------------------
    // Inherited values
    // ------------------------------------------------------------------------------------------

    /// Reports a feature that overrides an inherited member without saying so.
    ///
    /// The resolver already derived which inherited member each such feature redefines, so this
    /// rule reads that relationship rather than re-walking the type hierarchy per diagnostic. Only
    /// a feature that authored a value is reported: redeclaring an inherited name to add structure
    /// is ordinary, while binding a value to it silently overrides the inherited one.
    pub(crate) fn collect_inherited_value_rules(
        &self,
        document: DocumentIdx,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        for relationship in self.resolution.implied_relationships.iter() {
            if relationship.kind != ReferenceKind::Redefinition {
                continue;
            }
            let source = relationship.source;
            let declaration = self
                .storage
                .declaration(source)
                .ok_or(ResolutionError::InvalidStorage)?;
            if declaration.document != document {
                continue;
            }
            // Only a value that overrides. `default = e` states a value the specialization may
            // replace, which is exactly what redeclaring an inherited parameter to give it a
            // default means, so it is not a silent override.
            // KerML 8.4.4.5 specifies implicit redefinition for the parameters of a specializing
            // behavior, so a specializing function restating `in v : T` or `return u : T = ...` is
            // the authored way to give a body, not a silent override.
            let is_parameter = self
                .storage
                .declaration_facts(source)
                .is_some_and(|facts| facts.direction.is_some());
            if is_parameter
                || matches!(
                    declaration.kind,
                    DeclarationKind::ParameterUsage | DeclarationKind::SubjectUsage
                )
            {
                continue;
            }
            let Some(value) = self
                .storage
                .feature_values
                .iter()
                .find(|value| value.declaration == source && !value.is_default)
            else {
                continue;
            };
            if !self
                .authored_references(source, &[ReferenceKind::Redefinition])
                .is_empty()
            {
                continue;
            }
            let target = relationship.target;
            // Only a redefinition the *name* implies. The resolver also derives positional
            // redefinitions -- a specializing function's `return u : T = ...` redefines the return
            // parameter it inherits whatever either is called -- and those are the authored way to
            // state a body, not a silently overridden member.
            let named_collision = self
                .storage
                .declaration(source)
                .and_then(|declaration| declaration.name)
                .is_some_and(|name| {
                    self.storage
                        .declaration(target)
                        .and_then(|declaration| declaration.name)
                        == Some(name)
                });
            if !named_collision {
                continue;
            }
            let mut diagnostic = self.declaration_message_diagnostic(
                source,
                DiagnosticCode::ImplicitRedefinitionWithoutOperator,
                DiagnosticSeverity::Error,
                Some(format!(
                    "'{}' overrides inherited member '{}' but is missing the explicit ':>>' \
                     operator.",
                    self.display_name(source),
                    self.display_name(target)
                )),
            )?;
            diagnostic.related = Box::from([self.related_declaration(target, RELATED_INHERITED)?]);
            diagnostics.push(diagnostic);

            // A string bound to a member the inherited declaration types by an enumeration names
            // no enumeration literal. Read from the settled value and the settled type, not from
            // the authored text.
            if declaration.kind != DeclarationKind::AttributeUsage {
                continue;
            }
            if !matches!(
                self.evaluation_for(source).value(),
                Some(EvaluatedScalar::String(_))
            ) {
                continue;
            }
            let enumerated = self
                .types
                .effective_types(target)
                .iter()
                .any(|(type_id, _)| {
                    self.kind_of(*type_id) == Some(DeclarationKind::EnumerationDefinition)
                });
            if !enumerated {
                continue;
            }
            let mut diagnostic = self.declaration_message_diagnostic(
                source,
                DiagnosticCode::InheritedAttributeValueTypeMismatch,
                DiagnosticSeverity::Error,
                Some(format!(
                    "'{}' is typed by an enumeration through '{}' but was assigned a string \
                     literal; use an enumeration value.",
                    self.display_name(source),
                    self.display_name(target)
                )),
            )?;
            diagnostic.location = DiagnosticLocation {
                document: writer::document_identity(self, document).into(),
                range: document_range(&self.storage, document, &value.span)?,
            };
            diagnostic.related = Box::from([self.related_declaration(target, RELATED_INHERITED)?]);
            diagnostics.push(diagnostic);
        }
        Ok(())
    }

    // ------------------------------------------------------------------------------------------
    // Analysis status
    // ------------------------------------------------------------------------------------------

    /// Reports the settled verdict of an authored analysis, and the analyses that could not settle.
    ///
    /// Only a declaration that authored the analysis is reported. Evaluation facts are broader on
    /// purpose -- an inherited requirement constraint stays queryable from every usage -- and
    /// reporting an inherited template's unresolved result on each use would turn reuse into an
    /// apparent local error.
    ///
    /// The unresolved rule fires only on a genuine evaluation failure. An expression that is
    /// correctly not constant, one whose shape this evaluator does not fold, one that depends on
    /// itself, and one that was never run are each their own published state and none of them is a
    /// fault in the model.
    pub(crate) fn collect_analysis_status(
        &self,
        declared: &[DeclarationId],
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        for id in declared.iter().copied() {
            if !self.authors_an_analysis(id) {
                continue;
            }
            match self.evaluation_for(id) {
                EvaluationState::Literal(EvaluatedScalar::Boolean(false))
                | EvaluationState::Evaluated(EvaluatedScalar::Boolean(false)) => {
                    diagnostics.push(self.declaration_message_diagnostic(
                        id,
                        DiagnosticCode::AnalysisConstraintFailed,
                        DiagnosticSeverity::Warning,
                        Some(format!(
                            "Analysis constraint on '{}' evaluated to false.",
                            self.display_name(id)
                        )),
                    )?);
                }
                EvaluationState::Failed(failure) => {
                    diagnostics.push(self.declaration_message_diagnostic(
                        id,
                        DiagnosticCode::AnalysisEvaluationUnresolved,
                        DiagnosticSeverity::Warning,
                        Some(format!(
                            "Analysis expression on '{}' could not be evaluated: {}.",
                            self.display_name(id),
                            failure.as_str()
                        )),
                    )?);
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// Reports a document whose imports cannot resolve because no library was admitted.
    ///
    /// Reporting policy stated as an owner fact rather than as host configuration: what makes the
    /// hint true is that this publication admitted no library or standard-library source, which is
    /// a property of the model state, not of a path setting a host happens to hold. The host still
    /// decides whether to show it.
    ///
    /// `already` is where this document's diagnostics start, so the rule reads the unresolved
    /// outcomes the earlier producers settled instead of re-deciding them.
    pub(crate) fn collect_library_context(
        &self,
        document: DocumentIdx,
        already: usize,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        if self
            .storage
            .documents
            .iter()
            .any(|admitted| admitted.role != source_identity::SourceRole::Workspace)
        {
            return Ok(());
        }
        if !diagnostics[already..].iter().any(|diagnostic| {
            matches!(
                diagnostic.code,
                DiagnosticCode::UnresolvedTypeReference
                    | DiagnosticCode::UnresolvedSpecializesReference
                    | DiagnosticCode::UnresolvedImportTarget
                    | DiagnosticCode::UnresolvedReference
            )
        }) {
            return Ok(());
        }
        // Reported at the first import the document authored: that is the declaration a reader
        // would add a library for. A document with no import states no external dependency.
        let mut first: Option<(TextRange, DeclarationId)> = None;
        for (index, reference) in self.storage.references.iter().enumerate() {
            if reference.import.is_none() {
                continue;
            }
            let source = self
                .storage
                .declaration(reference.source)
                .ok_or(ResolutionError::InvalidStorage)?;
            if source.document != document {
                continue;
            }
            let _ = index;
            let range = document_range(&self.storage, document, &reference.span)?;
            if first.is_none_or(|(current, _)| range < current) {
                first = Some((range, reference.source));
            }
        }
        let Some((range, source)) = first else {
            return Ok(());
        };
        diagnostics.push(Diagnostic {
            payload: None,
            message: DiagnosticCode::MissingLibraryContext.describe().into(),
            code: DiagnosticCode::MissingLibraryContext,
            severity: DiagnosticSeverity::Information,
            origin: DiagnosticOrigin::Semantic,
            subject: self.symbol_id(source),
            location: DiagnosticLocation {
                document: writer::document_identity(self, document).into(),
                range,
            },
            related: Box::default(),
        });
        Ok(())
    }

    /// Whether a declaration authored the analysis whose verdict is reported on it.
    ///
    /// A definition is excluded: `constraint def` and `calc def` state what *would* be evaluated,
    /// and reporting a verdict for one would report the template as passing or failing rather than
    /// its usages. A usage that inherits its expression is excluded too -- it authored no analysis,
    /// and the declaration that did is reported once, where it was written.
    pub(crate) fn authors_an_analysis(&self, id: DeclarationId) -> bool {
        self.kind_of(id).is_some_and(|kind| {
            matches!(
                kind,
                DeclarationKind::AnalysisCaseUsage
                    | DeclarationKind::VerificationCaseUsage
                    | DeclarationKind::ConstraintUsage
                    | DeclarationKind::AssertConstraintUsage
                    | DeclarationKind::RequireConstraintUsage
            )
        }) && self
            .storage
            .evaluation_facts
            .iter()
            .any(|fact| fact.declaration == id)
    }
}

/// Whether a directed graph reaches a cycle from `node`.
pub(crate) fn reaches_cycle(
    node: DeclarationId,
    edges: &BTreeMap<DeclarationId, Vec<DeclarationId>>,
    visited: &mut BTreeSet<DeclarationId>,
    stack: &mut BTreeSet<DeclarationId>,
) -> bool {
    if stack.contains(&node) {
        return true;
    }
    if !visited.insert(node) {
        return false;
    }
    stack.insert(node);
    let found = edges
        .get(&node)
        .into_iter()
        .flatten()
        .any(|next| reaches_cycle(*next, edges, visited, stack));
    stack.remove(&node);
    found
}
