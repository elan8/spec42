//! Phase 2: the authored-fact records lowering produces.

use std::sync::Arc;

use source_identity::ContentDigest;
use source_identity::SourceRole;
use sysml_v2_parser::{
    ast::{
        BasicFeaturePrefix, BasicUsagePrefix, DefinitionPrefix, Expression, FeaturePrefix,
        FeaturePrefixHead, FeatureVariability, InOut, KermlClassifierKeyword, KermlFeatureKind,
        Multiplicity, Node, OccurrencePortionKind as ParserOccurrencePortionKind,
        OccurrenceUsagePrefix, QualifiedReferenceId, Span,
    },
    ParseError, ParsedDocument,
};

use crate::model::{
    AuthoredReferenceId, DeclarationId, DeclarationKind, DocumentIdx, MembershipKind, NameId,
    ReferenceKind, SymbolPathId, Visibility,
};
use crate::{MembershipRole, TextPosition, TextRange};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AuthoredImportShape {
    Membership,
    Namespace,
    Filter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct AuthoredImportFacts {
    pub(crate) shape: AuthoredImportShape,
    pub(crate) recursive: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct RelationshipFlags {
    /// The authored target is a dotted feature chain (`a.b`, KerML `FeatureChain`), so it is
    /// resolved segment by segment through each hop's type rather than as a `::` qualified
    /// name. Set by `push_reference` from the parser's typed separators; never inferred from text.
    pub(crate) dotted: bool,
    pub(crate) conjugated: bool,
    pub(crate) implied: bool,
    pub(crate) recursive: bool,
    pub(crate) wildcard: bool,
    pub(crate) direction: Option<ParameterDirection>,
    /// Mirrors the `variation` keyword prefix (BNF `BasicDefinitionPrefix`, `DefinitionPrefix::
    /// Variation`) on the owning `part`/`part def` declaration whose `FeatureTyping`/
    /// `Subclassification` reference carries this flag -- the same convention `conjugated` uses
    /// for a port's typing target polarity, rather than inventing a new relationship kind. Set on
    /// `lower_part_usage`'s own typing reference (e.g. `variation part transmission :
    /// Transmission;`); the sibling `abstract` prefix is deliberately left unrepresented, as
    /// before this slice.
    pub(crate) variation: bool,
}

/// The `in`/`out`/`inout` direction prefix on a directed parameter declaration (BNF `InOutDecl`),
/// carried as a fact on the declaration's `FeatureTyping` reference (see
/// `DeclarationKind::ParameterUsage`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParameterDirection {
    In,
    Out,
    InOut,
}

/// One authored multiplicity bound (`[lower..upper]`).
///
/// The parser stores each bound as a full `Expression` (`ast::Multiplicity`), and a bound written
/// as `*` -- or omitted entirely, as in `[1..*]`'s upper and both of `[*]`'s -- is `None` on that
/// side. A missing bound in an authored multiplicity is therefore the unbounded bound, which is
/// why absence and `Unbounded` are the same fact here; a declaration with no `[...]` at all
/// carries no `MultiplicityRecord` in the first place.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MultiplicityBound {
    /// No bound authored on this side: unbounded.
    Unbounded,
    /// A bound that folds to a literal integer from literals alone (`[3]`, `[0..4]`).
    Literal(i64),
    /// A bound authored as a non-literal expression (`[1..n]`, `[a#(0)]`). Published as an explicit
    /// non-literal fact -- its effective value needs operand resolution, which this fact family
    /// deliberately does not perform, and it is never recovered by re-reading authored text.
    Expression,
}

/// The authored multiplicity of one declaration (BNF `MultiplicityBounds`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MultiplicityRecord {
    pub(crate) lower: MultiplicityBound,
    pub(crate) upper: MultiplicityBound,
    pub(crate) span: Span,
}

/// The `snapshot`/`timeslice` portion prefix on an occurrence usage (`ast::OccurrencePortionKind`).
///
/// Note the bare `portion` keyword is a separate, unrelated modifier. In KerML scope it is
/// reachable as `ast::KermlFeatureMember::is_portion` and lowers to `DeclarationModifiers::portion`;
/// only the two portion *kinds* below are reachable here, and only on `OccurrenceUsage`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PortionKind {
    Snapshot,
    Timeslice,
}

/// The closed set of declaration modifier prefixes the pinned parser can express.
///
/// Deliberately not a `Vec<String>` of labels: each flag is a typed fact with exactly one parser
/// field behind it. Modifiers the parser cannot represent at all -- SysML `readonly`, SysML
/// `variable`, `unique`, and the bare `portion` prefix -- are absent from this set by construction
/// rather than defaulted to `false`; see `planning/UPSTREAM_PARSER_GAPS.md`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct DeclarationModifiers {
    /// `abstract` (`ast::DefinitionPrefix::Abstract`, or a bare `is_abstract` field).
    pub(crate) is_abstract: bool,
    /// `variation` (`ast::DefinitionPrefix::Variation`, or a bare `is_variation` field).
    pub(crate) variation: bool,
    /// `individual`.
    pub(crate) individual: bool,
    /// `derived`.
    pub(crate) derived: bool,
    /// `end` used as a feature prefix (distinct from the `end name : T;` declaration form, which
    /// is its own `DeclarationKind`).
    pub(crate) end: bool,
    /// `ref` used as a feature prefix (distinct from the `ref name : T;` declaration form).
    pub(crate) reference: bool,
    /// `constant`.
    pub(crate) constant: bool,
    /// `event`.
    pub(crate) event: bool,
    /// `standard`, on a library package.
    pub(crate) standard: bool,
    /// `all`, the sufficiency prefix.
    pub(crate) all: bool,
    /// KerML `composite`.
    pub(crate) composite: bool,
    /// KerML `portion` (reachable only through `KermlFeatureMember`).
    pub(crate) portion: bool,
    /// KerML `var`.
    pub(crate) var: bool,
    /// KerML `member`.
    pub(crate) member: bool,
    /// `parallel`, the state body modifier (`StateDefBody`/`StateUsageBody`, SysML BNF 1192:
    /// `( isParallel ?= 'parallel' )?`), which is `StateDefinition::isParallel` /
    /// `StateUsage::isParallel`.
    pub(crate) parallel: bool,
    /// `ordered`, the collection modifier.
    pub(crate) ordered: bool,
    /// `nonunique`, the collection modifier.
    pub(crate) nonunique: bool,
}

/// The authored presentation-adjacent facts of one declaration, recorded at the point its typed
/// parser node is lowered.
///
/// Held in a dense side table parallel to `SemanticModelStorage::declarations` rather than widened
/// into `Declaration`, which stays the compact identity/ownership record every resolution index is
/// built over.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct DeclarationFacts {
    /// The `<shortName>` identification prefix, where the owning parser node has the field.
    pub(crate) short_name: Option<NameId>,
    pub(crate) modifiers: DeclarationModifiers,
    pub(crate) portion_kind: Option<PortionKind>,
    pub(crate) direction: Option<ParameterDirection>,
    pub(crate) multiplicity: Option<MultiplicityRecord>,
    /// Authored negation for a declaration whose exact metaclass owns an `isNegated` fact.
    ///
    /// Satisfy, assert, and invariant spell the polarity at different grammar positions, but
    /// generated library-specialization predicates consume this one canonical semantic fact.
    pub(crate) negated: Option<bool>,
    /// Whether this `AcceptActionUsage` is the typed trigger action of a transition. The parser
    /// records the trigger shape on `TransitionAccept`; lowering publishes it on the synthesized
    /// accept action so generated rules never infer it from an owner name or reference spelling.
    pub(crate) is_trigger_action: Option<bool>,
    /// Whether an `AcceptActionUsage`'s first input parameter owns an authored value expression.
    /// Typed payload declarations still synthesize the required payload parameter, but have no
    /// argument expression; shorthand/time triggers do.
    pub(crate) accept_has_payload_argument: Option<bool>,
    /// Whether an `AcceptActionUsage` has the optional receiver input parameter/argument authored
    /// by a trailing `via` clause.
    pub(crate) accept_has_receiver_argument: Option<bool>,
    /// Whether a `SendActionUsage` owns its optional sender argument (`via`).
    pub(crate) send_has_sender_argument: Option<bool>,
    /// Whether a `SendActionUsage` owns its optional receiver argument (`to`).
    pub(crate) send_has_receiver_argument: Option<bool>,
    /// Whether this `IfActionUsage` has its typed `elseAction` branch. The parser records this as
    /// `IfStmt::else_body`; lowering publishes the presence bit so generated specialization rules
    /// select their anchor without reconstructing control-flow syntax.
    pub(crate) has_else_action: Option<bool>,
    /// One-based `ActionUsage::inputParameter(i)` position when this declaration is the single
    /// action selected by a control-action branch/body syntax production.
    pub(crate) action_input_parameter_position: Option<u32>,
    /// The number of direct, typed `from`/`to` endpoints on a lowered anonymous `FlowUsage`.
    ///
    /// KerML's `ownedEndFeatures` collection is represented by these two parser fields for the
    /// only flow-use form this lowering admits. It is intentionally absent for unsupported named
    /// or typed flow forms, so generated specialization predicates cannot turn an incomplete
    /// lowering into a positive result.
    pub(crate) owned_end_feature_count: Option<u32>,
    /// This declaration's position among its owner's authored connector ends (BNF `EndDecl`).
    ///
    /// Present only on a declaration lowered from an `end` member of a connection/interface/
    /// occurrence definition body, and it is what makes a connector end *positional*: KerML orders
    /// a connector's ends, and the source/target distinction of a binary connection-like
    /// definition is that order and nothing else. An `end` member's declared label is optional and
    /// carries no ordering, so recovering the position from a name would be a guess.
    ///
    /// Absent on every other declaration, including a feature carrying the `end` modifier prefix:
    /// that prefix says a feature *is* an end, while this fact says which end of its owner it is.
    /// The two are distinct and both are needed.
    pub(crate) positional_end: Option<u32>,
    /// One-based position among the owner's authored FeatureMemberships.
    pub(crate) owned_feature_position: Option<u32>,
    /// The two independently named endpoints of KerML `Feature::crossFeature` and
    /// `Feature::ownedCrossFeature()` when lowering an authored owned-cross feature.
    ///
    /// The Pilot's `addCrossingSpecialization` creates the implied CrossSubsetting whose second
    /// chained feature is the owned cross feature. Publishing both identities here preserves that
    /// transformation result without asking a validation consumer to reconstruct parser syntax.
    pub(crate) cross_feature_projection: Option<CrossFeatureProjection>,
    /// The canonical result Feature owned by this Expression.
    pub(crate) expression_result: Option<DeclarationId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CrossFeatureProjection {
    pub(crate) cross_feature: DeclarationId,
    pub(crate) owned_cross_feature: DeclarationId,
}

impl DeclarationFacts {
    /// Facts for a declaration with no authored modifier, multiplicity, direction, or short name.
    ///
    /// Used both by synthesized anonymous scopes -- `BareConnect`, `Transition`, the state action
    /// bindings and other declarations the lowering mints to give nested references a lexical
    /// scope, which have no authored declaration syntax at all -- and by authored declaration
    /// forms whose parser node carries none of these fields.
    pub(crate) fn none() -> Self {
        Self::default()
    }
}

/// Which annotation production a `DocumentationRecord` came from.
///
/// The parser discards `/** ... */` and `//` as lexer trivia, so only the three keyworded forms
/// survive into the AST at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AnnotationForm {
    /// `doc /* ... */` (`ast::DocComment`).
    Documentation,
    /// `comment /* ... */` (`ast::CommentAnnotation`).
    Comment,
    /// `rep <language> "..." /* ... */` (`ast::TextualRepresentation`).
    TextualRepresentation,
}

/// One documentation/comment/textual-representation annotation bound to its annotated declaration.
///
/// The parser models these as *sibling* body elements with no parent link, so lowering binds the
/// annotations at the head of a declaration's own body to that declaration. A declaration may
/// carry several, which is why this is a table rather than a field on `DeclarationFacts`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DocumentationRecord {
    pub(crate) declaration: DeclarationId,
    pub(crate) form: AnnotationForm,
    pub(crate) locale: Option<NameId>,
    /// The `rep` language string; always `None` for the other two forms.
    pub(crate) language: Option<NameId>,
    pub(crate) text: NameId,
    pub(crate) span: Span,
}

/// Whether a feature value binds (`=`) or assigns (`:=`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FeatureValueKind {
    Bind,
    Assign,
}

/// The authored feature value of one declaration (`ast::FeatureValue`).
///
/// Keeps all five authored spellings distinguishable: `= e`, `:= e`, `default = e`, `default := e`,
/// and the operator-less `default e`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FeatureValueRecord {
    pub(crate) declaration: DeclarationId,
    pub(crate) value: DeclarationId,
    pub(crate) result: DeclarationId,
    pub(crate) kind: FeatureValueKind,
    pub(crate) is_default: bool,
    pub(crate) has_operator: bool,
    pub(crate) span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OperatorExpressionKind {
    Index,
    Select,
}

/// One authored SelectExpression or IndexExpression and its canonical result Feature.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OperatorExpressionRecord {
    pub(crate) expression: DeclarationId,
    pub(crate) result: DeclarationId,
    pub(crate) kind: OperatorExpressionKind,
}

/// One ordered argument Expression owned by an operator expression.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ExpressionArgumentRecord {
    pub(crate) expression: DeclarationId,
    pub(crate) argument: DeclarationId,
    pub(crate) result: DeclarationId,
    pub(crate) ordinal: u32,
}

/// One authored MetadataFeature instance and the Element it annotates.
///
/// The annotation declaration owns its typing reference and body. This record owns the opposite
/// endpoint, so consumers never have to infer the annotated Element from containment or from a
/// presentation relationship whose source was rewritten for display.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MetadataAnnotationRecord {
    pub(crate) annotation: DeclarationId,
    pub(crate) annotated_element: DeclarationId,
}

/// Builds the multiplicity fact for a declaration whose parser node carries a `multiplicity` field.
///
/// A declaration with no `[...]` written yields `None`; that is genuinely "no multiplicity
/// authored", distinct from `[*]`, which yields a record with both bounds `Unbounded`.
pub(crate) fn multiplicity_facts(
    multiplicity: Option<&Node<Multiplicity>>,
) -> Option<MultiplicityRecord> {
    let multiplicity = multiplicity?;
    Some(MultiplicityRecord {
        lower: multiplicity_bound(multiplicity.value.lower.as_deref()),
        upper: multiplicity_bound(multiplicity.value.upper.as_deref()),
        span: multiplicity.value.span,
    })
}

pub(crate) fn multiplicity_bound(expression: Option<&Node<Expression>>) -> MultiplicityBound {
    let Some(expression) = expression else {
        return MultiplicityBound::Unbounded;
    };
    match literal_bound_value(&expression.value) {
        Some(value) => MultiplicityBound::Literal(value),
        None => MultiplicityBound::Expression,
    }
}

/// Folds a multiplicity bound expression to a literal integer, or reports that it is not one.
///
/// Deliberately narrow: only an integer literal (optionally parenthesised) is a literal bound.
/// Everything else -- a feature reference, an arithmetic expression, an index -- is published as
/// `MultiplicityBound::Expression` rather than guessed at, because folding it needs operand
/// resolution this fact family does not perform.
pub(crate) fn literal_bound_value(expression: &Expression) -> Option<i64> {
    match expression {
        Expression::LiteralInteger(value) => Some(*value),
        Expression::Sequence { operands, .. } => match operands.value.elements.as_slice() {
            [only] => literal_bound_value(&only.expression.value),
            _ => None,
        },
        _ => None,
    }
}

/// Splits a `definition_prefix`/`usage_prefix` field into its two independent modifier facts.
pub(crate) fn definition_prefix_modifiers(prefix: Option<&DefinitionPrefix>) -> (bool, bool) {
    match prefix {
        Some(DefinitionPrefix::Abstract) => (true, false),
        Some(DefinitionPrefix::Variation) => (false, true),
        None => (false, false),
    }
}

/// The [`Node`]-wrapped spelling of [`definition_prefix_modifiers`], for the definition kinds
/// whose `definition_prefix` slot carries the authored keyword's own span.
pub(crate) fn definition_prefix_node_modifiers(
    prefix: Option<&Node<DefinitionPrefix>>,
) -> (bool, bool) {
    definition_prefix_modifiers(prefix.map(|prefix| &prefix.value))
}

/// Splits the shared `OccurrenceUsagePrefix` (BNF `OccurrenceUsagePrefix`, SysML 564) into the
/// independent modifier facts this model records.
///
/// The occurrence-usage families (`PartUsage`/`ItemUsage`/`PortUsage`/`OccurrenceUsage`) used to
/// carry `usage_prefix`/`is_individual`/`is_reference`/`is_derived`/`is_constant` as five separate
/// fields; upstream folded them into the one component the grammar spells, where presence of the
/// authored keyword's span *is* the property. Callers add the multiplicity keyword facts on top
/// through struct update syntax, since those come from `MultiplicityPart` rather than the prefix.
pub(crate) fn occurrence_prefix_modifiers(prefix: &OccurrenceUsagePrefix) -> DeclarationModifiers {
    let basic = prefix.basic();
    let (is_abstract, variation) = definition_prefix_node_modifiers(
        basic.and_then(|basic| basic.ref_prefix.variance.as_ref()),
    );
    DeclarationModifiers {
        is_abstract,
        variation,
        individual: prefix.individual_span().is_some(),
        derived: basic.is_some_and(|basic| basic.ref_prefix.derived_span.is_some()),
        // `EndUsagePrefix` is the head alternative that excludes every basic slot, so `end` and
        // `ref`/`derived`/`constant`/direction are never both recorded from one prefix.
        end: prefix.end().is_some(),
        reference: basic.is_some_and(|basic| basic.reference_span.is_some()),
        constant: basic.is_some_and(|basic| basic.ref_prefix.constant_span.is_some()),
        ..DeclarationModifiers::default()
    }
}

/// The modifier facts of a bare `BasicUsagePrefix` (SysML BNF 281), for the productions that
/// spell it without the surrounding `OccurrenceUsagePrefix`: `OwnedCrossFeature` and the basic
/// alternative of `UnextendedUsagePrefix`.
pub(crate) fn basic_usage_prefix_modifiers(prefix: &BasicUsagePrefix) -> DeclarationModifiers {
    let (is_abstract, variation) =
        definition_prefix_node_modifiers(prefix.ref_prefix.variance.as_ref());
    DeclarationModifiers {
        is_abstract,
        variation,
        derived: prefix.ref_prefix.derived_span.is_some(),
        reference: prefix.reference_span.is_some(),
        constant: prefix.ref_prefix.constant_span.is_some(),
        ..DeclarationModifiers::default()
    }
}

/// The authored `in`/`out`/`inout` direction of an `OccurrenceUsagePrefix`. Only the
/// `BasicUsagePrefix` head has the slot; an `end`-headed prefix has no direction.
pub(crate) fn occurrence_prefix_direction(
    prefix: &OccurrenceUsagePrefix,
) -> Option<ParameterDirection> {
    direction_node_fact(
        prefix
            .basic()
            .and_then(|basic| basic.ref_prefix.direction.as_ref()),
    )
}

/// Whether an `OccurrenceUsagePrefix` authored the `variation` alternative of its
/// `abstract | variation` slot, which is what makes the usage's typing a variation typing.
pub(crate) fn occurrence_prefix_is_variation(prefix: &OccurrenceUsagePrefix) -> bool {
    matches!(
        prefix
            .basic()
            .and_then(|basic| basic.ref_prefix.variance.as_ref())
            .map(|prefix| prefix.value),
        Some(DefinitionPrefix::Variation)
    )
}

/// The [`Node`]-wrapped spelling of [`direction_fact`], for the prefix components that carry the
/// authored `in`/`out`/`inout` keyword's own span.
pub(crate) fn direction_node_fact(direction: Option<&Node<InOut>>) -> Option<ParameterDirection> {
    direction_fact(direction.map(|direction| &direction.value))
}

/// The [`Node`]-wrapped spelling of [`portion_kind_fact`], for `OccurrenceUsagePrefix::portion`.
pub(crate) fn portion_kind_node_fact(
    kind: Option<&Node<ParserOccurrencePortionKind>>,
) -> Option<PortionKind> {
    portion_kind_fact(kind.map(|kind| &kind.value))
}

pub(crate) fn portion_kind_fact(kind: Option<&ParserOccurrencePortionKind>) -> Option<PortionKind> {
    match kind? {
        ParserOccurrencePortionKind::Snapshot => Some(PortionKind::Snapshot),
        ParserOccurrencePortionKind::Timeslice => Some(PortionKind::Timeslice),
    }
}

/// Maps a bodied KerML classifier's keyword to the metaclass it denotes.
///
/// `assoc` and `association` are the short and spelled-out spellings of one keyword (see
/// `KermlClassifierKeyword`'s own doc comments), so both denote KerML `Association`; every other
/// keyword denotes a distinct metaclass.
pub(crate) fn kerml_classifier_kind(keyword: &KermlClassifierKeyword) -> DeclarationKind {
    match keyword {
        KermlClassifierKeyword::Type => DeclarationKind::KermlType,
        KermlClassifierKeyword::Classifier => DeclarationKind::KermlClassifier,
        // Upstream routed `class` through the shared KerML classifier declaration, deleting the
        // dedicated `ClassDef` node, so this keyword is now the only spelling a `class` reaches
        // and it keeps the `ClassDefinition` kind the dedicated production used to publish.
        // `DeclarationKind::KermlClass` is consequently unreachable.
        KermlClassifierKeyword::Class => DeclarationKind::ClassDefinition,
        KermlClassifierKeyword::Struct => DeclarationKind::KermlStructure,
        KermlClassifierKeyword::Assoc | KermlClassifierKeyword::Association => {
            DeclarationKind::KermlAssociation
        }
        KermlClassifierKeyword::AssocStruct => DeclarationKind::KermlAssociationStructure,
        KermlClassifierKeyword::Datatype => DeclarationKind::KermlDataType,
        KermlClassifierKeyword::Metaclass => DeclarationKind::KermlMetaclass,
        KermlClassifierKeyword::Behavior => DeclarationKind::KermlBehavior,
        KermlClassifierKeyword::Function => DeclarationKind::KermlFunction,
        KermlClassifierKeyword::Predicate => DeclarationKind::KermlPredicate,
        KermlClassifierKeyword::Interaction => DeclarationKind::KermlInteraction,
        KermlClassifierKeyword::Multiplicity => DeclarationKind::KermlMultiplicity,
    }
}

/// Maps a KerML feature member's kind keyword to the metaclass it denotes.
/// The declaration kind a KerML feature member's authored kind keyword names.
///
/// `None` is the keyword-less prefixed spelling (`portion redefines portionOfLife = ...;`), where
/// the grammar implies `Feature`, so it maps to the same kind a written `feature` does.
pub(crate) fn kerml_feature_kind(kind: Option<&Node<KermlFeatureKind>>) -> DeclarationKind {
    match kind.map(|kind| kind.value) {
        None | Some(KermlFeatureKind::Feature) => DeclarationKind::KermlFeature,
        Some(KermlFeatureKind::Step) => DeclarationKind::KermlStep,
        Some(KermlFeatureKind::Expr) => DeclarationKind::KermlExpression,
        Some(KermlFeatureKind::Bool) => DeclarationKind::KermlBooleanExpression,
    }
}

/// Splits `BasicFeaturePrefix` (KerML BNF 577) into the independent modifier facts this model
/// records.
///
/// `var` stays the authored `var` keyword rather than the metamodel's derived `isVariable` (which
/// `const` also sets), matching what [`DeclarationModifiers::var`] has always meant; `const` now
/// lands on `constant`, which the old `is_const` field never reached.
pub(crate) fn basic_feature_prefix_modifiers(prefix: &BasicFeaturePrefix) -> DeclarationModifiers {
    DeclarationModifiers {
        is_abstract: prefix.is_abstract(),
        derived: prefix.is_derived(),
        composite: prefix.is_composite(),
        portion: prefix.is_portion(),
        var: matches!(
            prefix.variability.as_ref().map(|slot| slot.value),
            Some(FeatureVariability::Var)
        ),
        constant: prefix.is_constant(),
        ..DeclarationModifiers::default()
    }
}

/// Splits the shared KerML `FeaturePrefix` (KerML BNF 584) into the independent modifier facts
/// this model records.
///
/// Upstream folded the seven booleans `KermlFeatureMember` used to carry into the choice the
/// grammar writes, so `end`-ness is the alternative taken rather than a flag beside it, and
/// `composite`/`portion` and `var`/`const` are each one slot. The `EndFeaturePrefix` alternative
/// carries no direction, abstractness or portioning at all, which is what makes `in end feature
/// x;` unauthorable rather than merely unparsed (gap 59).
pub(crate) fn kerml_feature_prefix_modifiers(prefix: &FeaturePrefix) -> DeclarationModifiers {
    match &prefix.head {
        FeaturePrefixHead::Basic(basic) => basic_feature_prefix_modifiers(basic),
        FeaturePrefixHead::End { prefix, .. } => DeclarationModifiers {
            end: true,
            constant: prefix.is_constant(),
            ..DeclarationModifiers::default()
        },
    }
}

pub(crate) fn direction_fact(direction: Option<&InOut>) -> Option<ParameterDirection> {
    match direction? {
        InOut::In => Some(ParameterDirection::In),
        InOut::Out => Some(ParameterDirection::Out),
        InOut::InOut => Some(ParameterDirection::InOut),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ParserReferenceId {
    pub(crate) document: DocumentIdx,
    pub(crate) local: QualifiedReferenceId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum UnsupportedFamily {
    PackageMember,
    PartDefinitionMember,
    PartUsageMember,
    AttributeMember,
    RequirementDefinitionMember,
    PortDefinitionMember,
    /// No remaining `PortBodyElement` fallback variant constructs this family (every variant is
    /// now dispatched to a lowering function), but the diagnostic code itself remains part of
    /// `writer.rs`'s exhaustive mapping for schema stability.
    #[allow(dead_code)]
    PortUsageMember,
    ActionDefinitionMember,
    ActionUsageMember,
    /// Shared by `state def` and `state` usage bodies (both use `StateDefBody`/
    /// `StateDefBodyElement` in the typed AST -- there is no separate `StateUsageBody`).
    StateDefinitionMember,
    /// Shared by `connection def` and `connection` usage bodies (both use `ConnectionDefBody`/
    /// `ConnectionDefBodyElement` in the typed AST -- there is no separate `ConnectionUsageBody`).
    ConnectionDefinitionMember,
    /// Shared by `occurrence def` and `occurrence` usage bodies (both share `OccurrenceBodyElement`
    /// -- `OccurrenceDef.body` wraps it in the generic `DefinitionBody`/`DefinitionBodyElement`,
    /// while `OccurrenceUsage.body` (`OccurrenceUsageBody`) holds it directly). Occurrence-specific
    /// semantics -- individual/portion-of-life, time-slicing, snapshot facts, `exhibit`/
    /// `succession`/`satisfy`/`allocate`/connector-end body constructs -- are the out-of-scope
    /// surface for this slice.
    OccurrenceDefinitionMember,
    /// `analysis def` body members not modeled by this slice (subject/actor/objective/first-
    /// succession/return/nested case structure); shares `UseCaseDefBody`/`UseCaseDefBodyElement`
    /// with `case`/`verification` case bodies in the typed AST, but this family name is scoped to
    /// `analysis def` specifically since `analysis` usage lowering is deferred.
    AnalysisCaseDefinitionMember,
    /// `constraint def`/`constraint` usage body members not modeled by this slice (constraint
    /// expression content, nested constraint members). Shared by both forms since
    /// `ConstraintDefBody`/`ConstraintDefBodyElement` is the same typed AST shape for both.
    ConstraintDefinitionMember,
    /// `calc def`/`calc` usage body members not modeled by this slice (calculation expression
    /// content, in/out/return parameters, nested calc structure). Shared by both forms since
    /// `CalcDefBody`/`CalcDefBodyElement` is the same typed AST shape for both.
    CalcDefinitionMember,
    /// `interface def` body members not modeled by this slice; shares the same `end`/`connect`/
    /// attribute/item/port/flow member set as `ConnectionDefinitionMember`, kept as its own family
    /// name so interface def diagnostics stay distinct from connection def ones at the same span
    /// shape.
    InterfaceDefinitionMember,
    /// `view def` body members not modeled by this slice: `render` (`ViewRenderingUsage`),
    /// `filter` (view composition), `expose`/`satisfy` are `view` usage-body-only constructs
    /// (`ViewBodyElement`, not `ViewDefBodyElement`) and don't appear here at all.
    ViewDefinitionMember,
    /// `rendering def` body members not modeled by this slice: shares the same `filter`/`render`
    /// member set as `ViewDefinitionMember` (`RenderingDefBody`/`RenderingDefBodyElement` mirror
    /// `ViewDefBody`/`ViewDefBodyElement` exactly), kept as its own family name so `rendering def`
    /// diagnostics stay distinct from `view def` ones at the same span shape.
    RenderingDefinitionMember,
    /// `case def` body members not modeled by this slice (subject/actor/objective/first-
    /// succession/return/nested case structure); shares `UseCaseDefBody`/`UseCaseDefBodyElement`
    /// with `analysis`/`verification`/`use case` case bodies in the typed AST, but this family
    /// name is scoped to `case def` specifically since `case` usage lowering is deferred.
    CaseDefinitionMember,
    /// `verification def` body members not modeled by this slice; shares the same
    /// `UseCaseDefBody`/`UseCaseDefBodyElement` shape as `CaseDefinitionMember`, kept as its own
    /// family name so verification def diagnostics stay distinct from case/analysis/use-case def
    /// ones at the same span shape.
    VerificationCaseDefinitionMember,
    /// `use case def` body members not modeled by this slice; shares the same `UseCaseDefBody`/
    /// `UseCaseDefBodyElement` shape as `CaseDefinitionMember`, kept as its own family name so use
    /// case def diagnostics stay distinct from case/analysis/verification def ones at the same
    /// span shape.
    UseCaseDefinitionMember,
    /// Members inside a `ref { ... }` body that this slice does not model. A `ref` body is the
    /// general usage-member set (`RefBody = Body<PartUsageBodyElement>`, `UsageBody =
    /// DefinitionBody` per SysML 8.2.2.6.2), so it is dispatched through the same
    /// `lower_part_usage_body_element` walker every other usage body uses; this family keeps its
    /// unsupported members distinguishable from a `part` usage body's. The `ref` declaration
    /// itself (name, typing, redefines, subsets) is lowered via `DeclarationKind::ReferenceUsage`
    /// regardless of what its body holds.
    ReferenceUsageMember,
    /// Members of a KerML `RelationshipBody` (`import`/`dependency`/`alias`/plain `connect`
    /// bodies) that this slice does not model. The body's annotating members are recorded against
    /// the relationship's own declaration; this family covers unmodeled facts of an owned
    /// `feature` member (BNF `RelationshipBody`'s `ownedRelatedElement`).
    RelationshipBodyMember,
    ParserUnsupported,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct UnsupportedRecord {
    pub(crate) document: DocumentIdx,
    pub(crate) family: UnsupportedFamily,
    pub(crate) span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RecoveryRecord {
    pub(crate) document: DocumentIdx,
    pub(crate) span: Span,
}

/// One admitted document's parse product, held by the construction phases and by nothing else.
///
/// `design.md`: a sealed publication holds no parse tree. The tree and the source text are the
/// syntax service's to own; the phases that lower, evaluate and settle facts from them read them
/// through this record, which is dropped at the publication barrier. What survives into the sealed
/// model is [`CanonicalDocument`] -- an identity, a role, and the line index a settled span needs.
#[derive(Debug)]
pub(crate) struct AdmittedDocument {
    pub(crate) identity: Box<str>,
    /// The role this source plays in the build, carried through from the admitted
    /// [`OwnedSourceRecord`]. Library sources participate in one semantic system with workspace
    /// sources, so this is never an admission filter; it is what lets owner-defined projections
    /// report the authored workspace without also reporting the whole standard library.
    pub(crate) role: SourceRole,
    /// The digest of the text this tree was parsed from: the complete key of this document's
    /// lowering product, since the lowering walk reads the tree and nothing else.
    pub(crate) digest: ContentDigest,
    pub(crate) parsed: Arc<ParsedDocument>,
    pub(crate) parse_errors: Box<[ParseError]>,
}

/// One document as the sealed publication holds it: no tree, no source text.
#[derive(Debug)]
pub(crate) struct CanonicalDocument {
    pub(crate) identity: Box<str>,
    /// See [`AdmittedDocument::role`].
    pub(crate) role: SourceRole,
    /// Where each line of the admitted source began, so a settled byte span still projects to a
    /// line/column range once the text itself is gone.
    pub(crate) lines: LineIndex,
}

/// The byte offset of every line start in one admitted source.
///
/// Four bytes per line, against the whole source text and AST it replaces in the sealed model. A
/// settled parser `Span` is projected through this rather than by re-reading the document, which is
/// what lets the publication answer a location query without a parse tree.
#[derive(Debug, Default)]
pub(crate) struct LineIndex {
    /// Byte offset of the first byte of every line; always begins with `0`.
    starts: Box<[u32]>,
    /// Byte length of the source, so an out-of-bounds span is rejected rather than clamped.
    length: u32,
}

impl LineIndex {
    pub(crate) fn build(text: &str) -> Self {
        let mut starts = vec![0u32];
        starts.extend(text.bytes().enumerate().filter_map(|(index, byte)| {
            (byte == b'\n')
                .then(|| u32::try_from(index + 1).ok())
                .flatten()
        }));
        Self {
            starts: starts.into_boxed_slice(),
            length: u32::try_from(text.len()).unwrap_or(u32::MAX),
        }
    }

    /// The zero-based line and byte column of `offset`, or `None` past the end of the source.
    pub(crate) fn position(&self, offset: usize) -> Option<TextPosition> {
        let offset = u32::try_from(offset).ok()?;
        if offset > self.length {
            return None;
        }
        let line = self
            .starts
            .partition_point(|start| *start <= offset)
            .checked_sub(1)?;
        Some(TextPosition {
            line: u32::try_from(line).ok()?,
            character: offset - self.starts.get(line)?,
        })
    }

    /// The zero-based range a settled byte span covers.
    pub(crate) fn range(&self, span: &Span) -> Option<TextRange> {
        let end = span.offset.checked_add(span.len)?;
        Some(TextRange {
            start: self.position(span.offset)?,
            end: self.position(end)?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Declaration {
    pub(crate) document: DocumentIdx,
    pub(crate) owner: Option<DeclarationId>,
    pub(crate) name: Option<NameId>,
    pub(crate) anonymous_ordinal: Option<u32>,
    pub(crate) kind: DeclarationKind,
    pub(crate) span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MembershipRecord {
    pub(crate) member: DeclarationId,
    pub(crate) kind: MembershipKind,
    pub(crate) visibility: Visibility,
    /// A role authored by the membership production rather than implied by the member metaclass.
    pub(crate) role: Option<MembershipRole>,
    pub(crate) span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AuthoredReference {
    pub(crate) source: DeclarationId,
    pub(crate) kind: ReferenceKind,
    pub(crate) target: ParserReferenceId,
    pub(crate) path: SymbolPathId,
    pub(crate) ordinal: u32,
    pub(crate) import: Option<AuthoredImportFacts>,
    pub(crate) flags: RelationshipFlags,
    pub(crate) span: Span,
}

pub(crate) struct PendingReference {
    pub(crate) source: DeclarationId,
    pub(crate) kind: ReferenceKind,
    pub(crate) document: DocumentIdx,
    pub(crate) local: QualifiedReferenceId,
    pub(crate) flags: RelationshipFlags,
    pub(crate) span: Span,
    pub(crate) import: Option<AuthoredImportFacts>,
}

/// Which expression grammar an authored expression was written in.
///
/// The two share almost all of their productions, but not all of them: a constraint body admits
/// the comparison and logical operators a calc body does not, and only the constraint traversal
/// builds `EvalNode::Comparison`/`EvalNode::Logical`. Recording which grammar the author wrote in
/// is what lets phase 5 pick the matching traversal without asking the owning declaration's kind
/// to stand in for the syntax.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ExpressionGrammar {
    /// A constraint body, an action guard, an assignment right-hand side, a `filter` condition --
    /// anything lowered through `lower_constraint_expression`.
    Constraint,
    /// A calculation body or a `return` value, lowered through `lower_calc_expression`.
    Calc,
}

/// One authored expression, recorded at its site: which document it was written in, which grammar
/// it was written in, where its operand ordinals start, and the expression itself.
///
/// This is an *authored* fact -- what the source says -- and deliberately not a classified one.
/// Lowering used to call `classify_expression` here and
/// store the resulting evaluation shape, which made phase 2 a writer of evaluation's own
/// vocabulary: two phases decided what an expression means, and only one of them was named
/// evaluation. The classification now happens exactly once, in `evaluate/`, over this record.
///
/// `operand_start` is genuinely a lowering-time fact and cannot be recovered later: a view owning
/// two `filter` statements lowers both against the view, so the second condition's operand
/// references are numbered after the first's. Classifying from zero would pair every leaf with the
/// wrong reference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AuthoredExpression {
    /// The document whose parser arena the expression's source-backed nodes belong to. A quantity
    /// literal's unit is a qualified reference into that arena rather than copied text, so the
    /// arena is required to read the expression's value at all.
    pub(crate) document: DocumentIdx,
    pub(crate) grammar: ExpressionGrammar,
    /// The ordinal the expression's first operand reference was lowered under.
    pub(crate) operand_start: u32,
    pub(crate) node: Expression,
}

/// An evaluation candidate: the declaration a constraint/calc expression belongs to, and the
/// expression as authored.
///
/// Every authored expression is recorded, including shapes evaluation does not support. "This
/// declaration authored an expression whose shape is outside the evaluated slice" and "this
/// declaration authored no expression at all" are different facts, and dropping the first would
/// publish the second in its place -- so the decision is evaluation's to make and to publish, not
/// lowering's to make and to discard.
#[derive(Debug, Clone)]
pub(crate) struct PendingEvaluationFact {
    pub(crate) declaration: DeclarationId,
    pub(crate) expression: AuthoredExpression,
}

/// One authored unit token: the `kg` in `10 [kg]`.
///
/// Kept apart from the quantity value it qualifies. The value carries the token's spelling because
/// a consumer rendering `10 [kg]` needs it; this record carries the token's *identity site* -- the
/// document and the exact range inside the brackets -- which is what a diagnostic about the unit
/// must point at. Neither is derivable from the other.
#[derive(Debug, Clone)]
pub(crate) struct AuthoredUnitToken {
    /// The declaration whose expression the token was written in.
    pub(crate) declaration: DeclarationId,
    pub(crate) document: DocumentIdx,
    /// Authored order within `declaration`, left to right, assigned in lockstep with lowering.
    pub(crate) ordinal: u32,
    /// The token exactly as the author wrote it, never normalized to a canonical unit identity.
    pub(crate) text: NameId,
    /// The token's own range: the text between `[` and `]`, excluding the brackets.
    pub(crate) span: Span,
}

/// Which member form a `filter` statement was written in.
///
/// The rule that a filter condition must be Boolean is authored per SysML §on view definitions; a
/// package-level `filter` is the import-filtering production and is a different statement with a
/// different owner. Recording the form is what lets a rule address one of them without asking the
/// owner's metaclass to stand in for the syntax the author used.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FilterForm {
    /// A `filter` inside a `view def` or `view` body.
    View,
    /// A `filter` inside a `rendering def` body.
    Rendering,
    /// A `filter` written directly in a package body, filtering its imports.
    PackageImport,
}

/// One authored `filter` condition.
///
/// A filter's condition is lowered against its owning declaration rather than a declaration of its
/// own -- the parser gives it no identity, and minting one would invent an element the author did
/// not write -- so it needs its own fact to carry the range and the authored expression a rule
/// about *this* condition must read.
#[derive(Debug, Clone)]
pub(crate) struct AuthoredFilterCondition {
    /// The view, rendering or package the filter is written in.
    pub(crate) owner: DeclarationId,
    pub(crate) document: DocumentIdx,
    pub(crate) form: FilterForm,
    /// The condition expression's own range.
    pub(crate) span: Span,
    pub(crate) expression: AuthoredExpression,
    pub(crate) predicate: FilterPredicate,
}

/// Candidate-dependent portion of a view condition. Unlike ordinary constant evaluation, an
/// `@Metadata` test has no truth value until an exposed element is supplied.
#[derive(Debug, Clone)]
pub(crate) enum FilterPredicate {
    Boolean(bool),
    Metadata(u32),
    And(Box<FilterPredicate>, Box<FilterPredicate>),
    Or(Box<FilterPredicate>, Box<FilterPredicate>),
    Unsupported,
}

/// One authored invocation, paired with the callee reference that names what it invokes.
///
/// The argument count is a property of the call site and exists nowhere else: the callee's own
/// declaration says how many parameters it has, and the resolved reference says which callee it
/// is, but only this record says how many arguments were written.
#[derive(Debug, Clone)]
pub(crate) struct AuthoredInvocation {
    /// The declaration the invocation was written in.
    pub(crate) declaration: DeclarationId,
    pub(crate) document: DocumentIdx,
    /// The `ReferenceKind::InvocationCallee` reference naming the callee, so the settled callee is
    /// read from that reference's resolution outcome rather than re-resolved here.
    pub(crate) callee: AuthoredReferenceId,
    /// How many arguments the author wrote.
    pub(crate) argument_count: u32,
    /// The invocation expression's own range.
    pub(crate) span: Span,
}
