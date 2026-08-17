//! Typed feature-conformance decisions, settled at the publication barrier.
//!
//! Every rule here is a function of facts the publication already owns: a declaration's metaclass,
//! its authored modifiers, multiplicity and direction, the settled outcome of an authored
//! reference, and the specialization closure. None of them re-resolves a name, reads authored
//! text, scans the model, or accepts a caller-chosen list of acceptable kinds.
//!
//! # Why kinds are classified into families
//!
//! The legacy check asked its caller for a slice of acceptable `ElementKind`s per rule, which made
//! the metamodel rule a property of the call site. The relation is really between *metaclass
//! families*: a `part` usage is typed by a part-family definition, and the fact that `part def`,
//! `item def` and `occurrence def` are all acceptable there is a statement about the SysML
//! metamodel's occurrence tower, not about who is asking. [`classify`] is therefore exhaustive over
//! `DeclarationKind`, and the tables below are keyed by family and role.
//!
//! # What "not classified" means
//!
//! [`classify`] returns `None` for a declaration outside the SysML definition/usage space --
//! namespaces, imports, aliases, KerML classifiers and features, and the anonymous scopes lowering
//! mints for nested references. An unclassified side settles *no* decision: it is not a silent
//! pass, it is the explicit statement that this rule's operands are not both SysML metaclasses.
//! This is what keeps a `part x : SomeKermlClass;` -- legal, and common in the standard library,
//! where the SysML families bottom out in KerML ones -- from being reported against a table that
//! does not describe KerML at all.

use super::*;

/// The SysML metaclass family a declaration belongs to.
///
/// One variant per definition/usage pair the language provides. Two declarations in the same
/// family are the definition and usage halves of one concept (`part def` / `part`), which is what
/// makes a family the right key for every rule below.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Family {
    Part,
    Attribute,
    Enumeration,
    Item,
    Occurrence,
    Action,
    State,
    Port,
    Requirement,
    Concern,
    UseCase,
    Case,
    AnalysisCase,
    VerificationCase,
    View,
    Viewpoint,
    Rendering,
    Metadata,
    Connection,
    Interface,
    Flow,
    Allocation,
    Constraint,
    Calc,
}

/// Which half of its family a declaration is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Role {
    Definition,
    Usage,
}

/// The family and role of one declaration, or `None` when it is outside the SysML metaclass space.
///
/// Exhaustive by construction: a new `DeclarationKind` fails to compile until it is classified, so
/// a kind cannot silently acquire "no conformance rule applies" by omission.
pub(super) fn classify(kind: DeclarationKind) -> Option<(Family, Role)> {
    use DeclarationKind as K;
    use Family as F;
    use Role::{Definition, Usage};
    Some(match kind {
        K::PartDefinition => (F::Part, Definition),
        K::PartUsage => (F::Part, Usage),
        K::AttributeDefinition => (F::Attribute, Definition),
        K::AttributeUsage => (F::Attribute, Usage),
        K::EnumerationDefinition => (F::Enumeration, Definition),
        K::EnumerationUsage => (F::Enumeration, Usage),
        K::ItemDefinition => (F::Item, Definition),
        K::ItemUsage => (F::Item, Usage),
        K::OccurrenceDefinition => (F::Occurrence, Definition),
        K::OccurrenceUsage => (F::Occurrence, Usage),
        K::ActionDefinition => (F::Action, Definition),
        K::ActionUsage => (F::Action, Usage),
        K::StateDefinition => (F::State, Definition),
        K::StateUsage => (F::State, Usage),
        K::PortDefinition => (F::Port, Definition),
        K::PortUsage => (F::Port, Usage),
        K::RequirementDefinition => (F::Requirement, Definition),
        K::RequirementUsage => (F::Requirement, Usage),
        K::ConcernDefinition => (F::Concern, Definition),
        K::ConcernUsage => (F::Concern, Usage),
        K::UseCaseDefinition => (F::UseCase, Definition),
        K::UseCaseUsage => (F::UseCase, Usage),
        K::CaseDefinition => (F::Case, Definition),
        K::CaseUsage => (F::Case, Usage),
        K::AnalysisCaseDefinition => (F::AnalysisCase, Definition),
        K::AnalysisCaseUsage => (F::AnalysisCase, Usage),
        K::VerificationCaseDefinition => (F::VerificationCase, Definition),
        K::VerificationCaseUsage => (F::VerificationCase, Usage),
        K::ViewDefinition => (F::View, Definition),
        K::ViewUsage => (F::View, Usage),
        K::ViewpointDefinition => (F::Viewpoint, Definition),
        K::ViewpointUsage => (F::Viewpoint, Usage),
        K::RenderingDefinition => (F::Rendering, Definition),
        K::RenderingUsage => (F::Rendering, Usage),
        K::MetadataDefinition => (F::Metadata, Definition),
        K::MetadataUsage => (F::Metadata, Usage),
        K::ConnectionDefinition => (F::Connection, Definition),
        K::ConnectionUsage => (F::Connection, Usage),
        K::InterfaceDefinition => (F::Interface, Definition),
        K::InterfaceUsage => (F::Interface, Usage),
        K::FlowDefinition => (F::Flow, Definition),
        K::Flow => (F::Flow, Usage),
        K::AllocationDefinition => (F::Allocation, Definition),
        K::Allocate => (F::Allocation, Usage),
        K::ConstraintDefinition => (F::Constraint, Definition),
        K::ConstraintUsage => (F::Constraint, Usage),
        K::CalcDefinition => (F::Calc, Definition),
        K::CalcUsage => (F::Calc, Usage),

        // Outside the definition/usage kind space this family of rules describes.
        //
        // Namespaces and their members are not typed. The KerML metaclasses are `Type`s, but the
        // SysML family tables do not describe them, and the SysML families bottom out in them --
        // reporting against a table that does not model KerML would flag most of the standard
        // library. The remaining entries are usage forms whose metaclass is fixed by their
        // syntax (`subject`, `actor`, a constraint assertion), anonymous scopes the lowering mints
        // to give nested references somewhere to resolve, and control-flow members that carry
        // references rather than a typing of their own.
        K::Namespace
        | K::Package
        | K::LibraryPackage
        | K::Import
        | K::Alias
        | K::EnumerationLiteral
        | K::ClassDefinition
        | K::ExtendedDefinition
        | K::IndividualDefinition
        | K::Succession
        | K::EntryActionBinding
        | K::DoActionBinding
        | K::ExitActionBinding
        | K::InitialState
        | K::FinalState
        | K::ParameterUsage
        | K::SubjectUsage
        | K::PerformActionUsage
        | K::Transition
        | K::Satisfy
        | K::Bind
        | K::ReferenceUsage
        | K::Decide
        | K::Merge
        | K::Fork
        | K::Join
        | K::ThenContinuation
        | K::StakeholderUsage
        | K::RequirementActor
        | K::CaseActor
        | K::Frame
        | K::VerifyRequirement
        | K::AssertConstraintUsage
        | K::AssumeConstraintUsage
        | K::RequireConstraintUsage
        | K::DefaultReferenceUsage
        | K::Assign
        | K::While
        | K::Loop
        | K::If
        | K::ForLoop
        | K::ForLoopVariable
        | K::Dependency
        | K::BareConnect
        | K::PerformParameterBinding
        | K::KermlType
        | K::KermlClassifier
        | K::KermlClass
        | K::KermlStructure
        | K::KermlAssociation
        | K::KermlAssociationStructure
        | K::KermlDataType
        | K::KermlMetaclass
        | K::KermlBehavior
        | K::KermlFunction
        | K::KermlPredicate
        | K::KermlInteraction
        | K::KermlMultiplicity
        | K::KermlFeature
        | K::KermlStep
        | K::KermlExpression
        | K::KermlBooleanExpression
        | K::KermlConnector
        | K::KermlBinding
        | K::KermlInvariant
        | K::KermlEnd => return None,
    })
}

/// The family a family specialises in the SysML metamodel, or `None` at a root.
///
/// This is the metamodel's own generalization hierarchy -- SysML §7's `PartUsage :> ItemUsage :>
/// OccurrenceUsage`, `StateUsage :> ActionUsage`, `RequirementUsage :> ConstraintUsage` and so on.
/// It is a static property of the language, not of any admitted library, so reading it here is not
/// a name lookup: nothing consults what `Occurrence` happens to be called or where it is declared.
///
/// A flat per-family allowlist could not express it. `action substates : StateAction[0..*];` types
/// an action usage with a state definition, which is well-formed precisely because a state *is* an
/// action; a list that did not happen to name `State` under `Action` reported it as a violation.
fn parent(family: Family) -> Option<Family> {
    use Family as F;
    Some(match family {
        F::Occurrence
        | F::Attribute
        | F::Port
        | F::Metadata
        | F::View
        | F::Viewpoint
        | F::Rendering
        | F::Constraint
        | F::Calc => return None,
        F::Item => F::Occurrence,
        F::Part => F::Item,
        F::Action => F::Occurrence,
        F::State => F::Action,
        F::Case => F::Action,
        F::UseCase => F::Case,
        F::AnalysisCase => F::Case,
        F::VerificationCase => F::Case,
        F::Connection => F::Occurrence,
        F::Interface => F::Connection,
        F::Flow => F::Connection,
        F::Allocation => F::Connection,
        F::Enumeration => F::Attribute,
        F::Requirement => F::Constraint,
        F::Concern => F::Requirement,
    })
}

/// Whether `family` is an occurrence family: `Occurrence` itself or one specialising it.
///
/// SysML's flow payload rule is about occurrences, and the occurrence families are exactly those
/// under `Occurrence` in the hierarchy above -- part, item, action, state, connection and the rest.
/// Asking the hierarchy keeps the rule from naming a library type.
pub(super) fn descends_from_occurrence(family: Family) -> bool {
    descends_from(family, Family::Occurrence)
}

/// Whether `descendant` is `ancestor` or specialises it, transitively.
fn descends_from(descendant: Family, ancestor: Family) -> bool {
    let mut cursor = Some(descendant);
    while let Some(current) = cursor {
        if current == ancestor {
            return true;
        }
        cursor = parent(current);
    }
    false
}

/// Whether a reference from a `source`-family declaration may target a `target`-family one.
///
/// The two must be comparable in the hierarchy above -- one specialises the other, in either
/// direction. Both directions occur and both are well-formed:
///
/// - *downwards*, the target is more specific than the source's family, as in `action substates :
///   StateAction;` or `occurrence zeroCrossingEvents : ZeroCrossingEventDef;`, where the type is a
///   definition of a family that specialises the usage's own;
/// - *upwards*, the target is more general, as in `part def Part :> Item` -- the Systems library's
///   own declaration -- or a part usage typed by an item definition.
///
/// What it rejects is a target in an unrelated branch: `part process : Process;` where `Process` is
/// an `action def`, `port out1 : DataPort;` where `DataPort` is a `part def`, or `enum color :
/// Color;` where `Color` is one. Those are the violations the rule exists to find, and they are
/// exactly the pairs with no path between them.
pub(super) fn families_are_comparable(source: Family, target: Family) -> bool {
    descends_from(target, source) || descends_from(source, target)
}

impl ResolvedSemanticModel {
    /// The family and role of one declaration, as opposed to of one `DeclarationKind`.
    ///
    /// A positional connector end is deliberately unclassified. The lowering gives every `end`
    /// member `DeclarationKind::ConnectionUsage` whatever its authored form -- `end p : P;` and
    /// `end port p : P;` both land there -- because an end is a connector's feature. That kind is
    /// a structural placeholder, not the end's metaclass, and judging `end drivePwrPort :
    /// DrivePwrPort;` against the connection family's typing rule would report the lowering rather
    /// than the model. The end's own type is whatever it connects.
    pub(super) fn declaration_family(&self, declaration: DeclarationId) -> Option<(Family, Role)> {
        if self
            .storage
            .declaration_facts(declaration)
            .is_some_and(|facts| facts.positional_end.is_some())
        {
            return None;
        }
        classify(self.storage.declaration(declaration)?.kind)
    }

    /// Appends every kind- and type-conformance diagnostic authored in `document`.
    ///
    /// Ordering is the caller's: [`Self::derive_diagnostics`] sorts each document's diagnostics by
    /// range and code once every producer has contributed.
    pub(super) fn collect_conformance(
        &self,
        document: DocumentId,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        self.collect_reference_kind_conformance(document, diagnostics)?;
        self.collect_implied_conformance(document, diagnostics)?;
        self.collect_type_relationship_cardinality(document, diagnostics)?;
        self.collect_specialization_cycles(document, diagnostics)?;
        Ok(())
    }

    /// Reports a type that owns exactly one `unions`, `intersects` or `differences` operand.
    ///
    /// KerML permits zero or at least two. One operand makes the operation the identity, so the
    /// declaration states a generalization the author did not write; the rule is a cardinality
    /// constraint on the relationship, not a conformance question, which is why it reads the
    /// authored references rather than the entailment.
    ///
    /// `disjoint from` is deliberately absent: disjointness is a pairwise statement about the
    /// owner and each target, so one target is meaningful.
    fn collect_type_relationship_cardinality(
        &self,
        document: DocumentId,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        for index in 0..self.storage.declarations.len() {
            let id = DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
            let declaration = self
                .storage
                .declaration(id)
                .ok_or(ResolutionError::InvalidStorage)?;
            if declaration.document != document {
                continue;
            }
            for kind in [
                ReferenceKind::Unioning,
                ReferenceKind::Intersecting,
                ReferenceKind::Differencing,
            ] {
                // Counted over authored references rather than settled operands: whether the
                // author wrote one target does not depend on whether it resolved.
                let mut authored = self
                    .storage
                    .references
                    .iter()
                    .filter(|reference| reference.source == id && reference.kind == kind);
                let Some(only) = authored.next() else {
                    continue;
                };
                if authored.next().is_some() {
                    continue;
                }
                diagnostics.push(self.reference_diagnostic(
                    only,
                    DiagnosticCode::SingleTypeRelationshipOperand,
                    DiagnosticSeverity::Error,
                    None,
                )?);
            }
        }
        Ok(())
    }

    /// Applies the redefinition rules to the redefinitions the resolver derived.
    ///
    /// A feature that redeclares an inherited feature's name redefines it without writing `:>>`,
    /// and that implied redefinition is where narrowing is usually authored -- `part def
    /// NarrowedFleet :> Fleet { part vehicles[3..3] : Chassis; }` states the whole redefinition in
    /// the multiplicity. Checking only authored references would leave the common case unchecked.
    ///
    /// Reported at the redefining declaration rather than at a reference span, because there is no
    /// authored reference: the relationship's provenance is `Implied`, and pointing at a range the
    /// author did not write would misattribute it.
    fn collect_implied_conformance(
        &self,
        document: DocumentId,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        for relationship in self.resolution.implied_relationships.iter() {
            if relationship.kind != ReferenceKind::Redefinition {
                continue;
            }
            let source = self
                .storage
                .declaration(relationship.source)
                .ok_or(ResolutionError::InvalidStorage)?;
            if source.document != document {
                continue;
            }
            if self
                .storage
                .declaration_facts(relationship.source)
                .is_some_and(|facts| facts.modifiers.derived)
            {
                continue;
            }
            let mut report = |code, severity| -> Result<(), ResolutionError> {
                diagnostics.push(Diagnostic {
                    code,
                    severity,
                    origin: DiagnosticOrigin::Semantic,
                    location: DiagnosticLocation {
                        document: writer::document_identity(self, document).into(),
                        range: document_range(&self.storage, document, &source.span)?,
                    },
                    related: Box::from([self.declaration_location(relationship.target)?]),
                });
                Ok(())
            };
            if self.typing_conformance(relationship.source, relationship.target)
                == Conformance::DoesNotConform
            {
                report(
                    DiagnosticCode::RedefinitionTypeIncompatible,
                    DiagnosticSeverity::Error,
                )?;
            }
            if self.multiplicity_widens(relationship.source, relationship.target) {
                report(
                    DiagnosticCode::RedefinitionMultiplicityWidened,
                    DiagnosticSeverity::Error,
                )?;
            }
        }
        Ok(())
    }

    fn collect_reference_kind_conformance(
        &self,
        document: DocumentId,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        for (index, reference) in self.storage.references.iter().enumerate() {
            let source = self
                .storage
                .declaration(reference.source)
                .ok_or(ResolutionError::InvalidStorage)?;
            if source.document != document {
                continue;
            }
            let id =
                AuthoredReferenceId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
            // Only a settled single target carries a kind to judge. Unresolved, ambiguous,
            // unsupported and non-converged outcomes are already their own published diagnostic,
            // and adding a kind verdict on top would report a second failure for one cause.
            let Some(ResolutionStatus::Resolved(target)) = self.resolution.outcome(id) else {
                continue;
            };
            self.check_reference_kind(reference, target, diagnostics)?;
            self.check_reference_conformance(reference, target, diagnostics)?;
        }
        Ok(())
    }

    fn check_reference_kind(
        &self,
        reference: &AuthoredReference,
        target: DeclarationId,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        let (Some((source_family, source_role)), Some((target_family, target_role))) = (
            self.declaration_family(reference.source),
            self.declaration_family(target),
        ) else {
            return Ok(());
        };

        let code = match reference.kind {
            ReferenceKind::FeatureTyping => {
                // A usage is typed by a definition. A typing whose target is another usage is a
                // different relationship shape, not a kind violation this rule can judge.
                if source_role != Role::Usage || target_role != Role::Definition {
                    return Ok(());
                }
                // A metadata definition classifies elements rather than describing one family's
                // structure, so it types a usage of any family. This is the reflective idiom the
                // SysML abstract-syntax library is written in: `derived item ownedActorParameter :
                // PartUsage[1..1];` types an item usage by the `metadata def` standing for the
                // PartUsage metaclass. The legacy check reached the same conclusion by testing
                // whether the authored path spelled `SysML::`, which is the spelling-driven
                // inference this owner does not do.
                if target_family == Family::Metadata && source_family != Family::Metadata {
                    return Ok(());
                }
                if families_are_comparable(source_family, target_family) {
                    return Ok(());
                }
                DiagnosticCode::IncompatibleTypeKind
            }
            ReferenceKind::Subclassification => {
                if source_role != Role::Definition || target_role != Role::Definition {
                    return Ok(());
                }
                if families_are_comparable(source_family, target_family) {
                    return Ok(());
                }
                DiagnosticCode::IncompatibleSpecializationKind
            }
            ReferenceKind::Subsetting
            | ReferenceKind::Redefinition
            | ReferenceKind::References
            | ReferenceKind::Crosses => {
                if source_role != Role::Usage {
                    return Ok(());
                }
                if families_are_comparable(source_family, target_family) {
                    return Ok(());
                }
                DiagnosticCode::IncompatibleSubsettingKind
            }
            _ => return Ok(()),
        };

        diagnostics.push(self.reference_diagnostic(
            reference,
            code,
            DiagnosticSeverity::Warning,
            Some(target),
        )?);
        Ok(())
    }

    /// KerML §7.4.12 and §8.4.3.4: a specializing feature's types and multiplicity must not admit
    /// what the feature it specializes excludes.
    fn check_reference_conformance(
        &self,
        reference: &AuthoredReference,
        target: DeclarationId,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        let code = match reference.kind {
            ReferenceKind::Redefinition => DiagnosticCode::RedefinitionTypeIncompatible,
            ReferenceKind::Subsetting | ReferenceKind::References | ReferenceKind::Crosses => {
                DiagnosticCode::SubsettingTypeIncompatible
            }
            _ => return Ok(()),
        };
        // A derived feature states a computation rather than a narrowing, so its types are not
        // required to conform to the feature it specializes.
        if self
            .storage
            .declaration_facts(reference.source)
            .is_some_and(|facts| facts.modifiers.derived)
        {
            return Ok(());
        }
        if self.typing_conformance(reference.source, target) == Conformance::DoesNotConform {
            diagnostics.push(self.reference_diagnostic(
                reference,
                code,
                DiagnosticSeverity::Error,
                Some(target),
            )?);
        }
        if reference.kind == ReferenceKind::Redefinition
            && self.multiplicity_widens(reference.source, target)
        {
            diagnostics.push(self.reference_diagnostic(
                reference,
                DiagnosticCode::RedefinitionMultiplicityWidened,
                DiagnosticSeverity::Error,
                Some(target),
            )?);
        }
        Ok(())
    }

    /// Whether the redefining feature admits a cardinality the redefined feature excludes.
    ///
    /// Both sides must have authored a multiplicity whose bounds are literal. A bound authored as
    /// an expression is published as exactly that, and comparing it would require evaluating an
    /// operand this fact family deliberately does not resolve; an absent multiplicity is absent,
    /// not `[1..1]`. Either case leaves the question unanswered rather than answered as "narrows".
    fn multiplicity_widens(&self, specific: DeclarationId, general: DeclarationId) -> bool {
        let (Some(specific), Some(general)) = (
            self.literal_multiplicity(specific),
            self.literal_multiplicity(general),
        ) else {
            return false;
        };
        if specific.0 < general.0 {
            return true;
        }
        match (specific.1, general.1) {
            (None, Some(_)) => true,
            (Some(specific_upper), Some(general_upper)) => specific_upper > general_upper,
            _ => false,
        }
    }

    /// The authored `[lower..upper]` of one declaration when both bounds fold to literals, with
    /// `None` as the upper bound standing for unbounded.
    fn literal_multiplicity(&self, declaration: DeclarationId) -> Option<(i64, Option<i64>)> {
        let multiplicity = self
            .storage
            .declaration_facts(declaration)?
            .multiplicity
            .as_ref()?;
        // The single-bound spelling `[3]` reaches this fact family already expanded to `[3..3]`,
        // so an `Unbounded` upper here is always an authored `*`, never an omitted bound.
        let (lower, upper) = match (multiplicity.lower, multiplicity.upper) {
            // An expression bound needs operand resolution to compare, which this fact family does
            // not perform, so the question stays unanswered on either side.
            (MultiplicityBound::Expression, _) | (_, MultiplicityBound::Expression) => return None,
            (MultiplicityBound::Literal(lower), MultiplicityBound::Unbounded) => (lower, None),
            (MultiplicityBound::Literal(lower), MultiplicityBound::Literal(upper)) => {
                (lower, Some(upper))
            }
            (MultiplicityBound::Unbounded, MultiplicityBound::Unbounded) => (0, None),
            (MultiplicityBound::Unbounded, MultiplicityBound::Literal(upper)) => (0, Some(upper)),
        };
        if lower < 0 || upper.is_some_and(|upper| upper < lower) {
            return None;
        }
        Some((lower, upper))
    }

    /// Reports every declaration in `document` that reaches itself through specialization.
    ///
    /// The closure already computed this while it saturated; the legacy check rediscovered it with
    /// a depth-first search per node over the whole graph.
    fn collect_specialization_cycles(
        &self,
        document: DocumentId,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        for index in 0..self.storage.declarations.len() {
            let id = DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
            let declaration = self
                .storage
                .declaration(id)
                .ok_or(ResolutionError::InvalidStorage)?;
            if declaration.document != document || !self.types.specialization().is_cyclic(id) {
                continue;
            }
            diagnostics.push(Diagnostic {
                code: DiagnosticCode::SpecializationCycle,
                severity: DiagnosticSeverity::Error,
                origin: DiagnosticOrigin::Semantic,
                location: DiagnosticLocation {
                    document: writer::document_identity(self, document).into(),
                    range: document_range(&self.storage, document, &declaration.span)?,
                },
                related: Box::default(),
            });
        }
        Ok(())
    }

    /// The authored-reference identity of one stored reference.
    pub(super) fn reference_index(
        &self,
        reference: &AuthoredReference,
    ) -> Result<AuthoredReferenceId, ResolutionError> {
        let index = self
            .storage
            .references
            .iter()
            .position(|candidate| std::ptr::eq(candidate, reference))
            .ok_or(ResolutionError::InvalidStorage)?;
        AuthoredReferenceId::from_index(index).map_err(|_| ResolutionError::Capacity)
    }

    /// One diagnostic reported at a declaration's own range.
    pub(super) fn declaration_diagnostic(
        &self,
        declaration: DeclarationId,
        code: DiagnosticCode,
        severity: DiagnosticSeverity,
    ) -> Result<Diagnostic, ResolutionError> {
        Ok(Diagnostic {
            code,
            severity,
            origin: DiagnosticOrigin::Semantic,
            location: self.declaration_location(declaration)?,
            related: Box::default(),
        })
    }

    /// One diagnostic reported at an authored reference, optionally pointing at what it resolved
    /// to.
    pub(super) fn reference_diagnostic(
        &self,
        reference: &AuthoredReference,
        code: DiagnosticCode,
        severity: DiagnosticSeverity,
        related: Option<DeclarationId>,
    ) -> Result<Diagnostic, ResolutionError> {
        let source = self
            .storage
            .declaration(reference.source)
            .ok_or(ResolutionError::InvalidStorage)?;
        Ok(Diagnostic {
            code,
            severity,
            origin: DiagnosticOrigin::Semantic,
            location: DiagnosticLocation {
                document: writer::document_identity(self, source.document).into(),
                range: document_range(&self.storage, source.document, &reference.span)?,
            },
            related: match related {
                Some(target) => Box::from([self.declaration_location(target)?]),
                None => Box::default(),
            },
        })
    }

    /// The declaration site one diagnostic points at as related information.
    pub(super) fn declaration_location(
        &self,
        declaration: DeclarationId,
    ) -> Result<DiagnosticLocation, ResolutionError> {
        let declaration = self
            .storage
            .declaration(declaration)
            .ok_or(ResolutionError::InvalidStorage)?;
        Ok(DiagnosticLocation {
            document: writer::document_identity(self, declaration.document).into(),
            range: document_range(&self.storage, declaration.document, &declaration.span)?,
        })
    }
}
