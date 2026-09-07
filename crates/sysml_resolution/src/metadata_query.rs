//! The published metadata-annotation contract.
//!
//! [`crate::ElementDetails::metadata`] answers only "which metadata definitions annotate this
//! element", collapsing every authored annotation into a deduplicated set of definition
//! identities. That is enough to *detect* a tag; it is not enough to *act* on one. Method and
//! traceability tooling that reads a model-level tag needs the annotation as it was written: how
//! it was applied, what else it is `about`, and the resolved values its body redefines
//! (`@Risk { probability = 0.3; }`).
//!
//! This contract adds no analysis. The annotation-to-definition reference, the `about`
//! references, and the body's redefining `AttributeUsage` members with their value expressions
//! are all lowered and resolved already; the only change is that the settled shape is retained
//! and handed back. A body value that is itself a non-constant expression is returned as the
//! same [`PublishedExpression`] tree [`crate::PublishedResolution::resolved_expression`]
//! publishes.

use crate::inspection::RelationshipTarget;
use crate::{PublishedExpression, SourceLocation, SymbolId};

/// Which production applied a metadata feature to its annotated element.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetadataAnnotationForm {
    /// A `#Tag` usage-prefix keyword or a body-less `#tag` prefix member. Carries no `about`
    /// clause and no body.
    PrefixKeyword,
    /// An `@Tag { ... }` / `@Tag;` / `metadata Tag { ... }` annotating body member.
    AnnotatingMember,
    /// A `metadata m : Tag;` named feature member, which is also a feature of its owner.
    Usage,
}

impl MetadataAnnotationForm {
    /// A stable kebab-case name, for snapshot output.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PrefixKeyword => "prefix-keyword",
            Self::AnnotatingMember => "annotating-member",
            Self::Usage => "usage",
        }
    }
}

/// One authored metadata annotation bound to an element, resolved and settled.
#[derive(Debug, Clone, PartialEq)]
pub struct PublishedMetadataAnnotation {
    /// The element the annotation applies to (the owner of the annotating feature, or an
    /// explicit `about` target — an annotation with `about X, Y` yields one
    /// `PublishedMetadataAnnotation` per bound element).
    pub annotated_element: SymbolId,
    /// The annotating feature's own identity — usually anonymous, named for `@t : Tag;` /
    /// `metadata t : Tag;`. Distinct authored annotations stay distinct even when identical.
    pub annotation: SymbolId,
    pub form: MetadataAnnotationForm,
    /// The annotating metadata definition (`Tag` in `@Tag`).
    pub definition: RelationshipTarget,
    /// The authored `about` targets in order, empty for the `#`-prefix form and for an
    /// annotation with no `about` clause.
    pub about: Box<[RelationshipTarget]>,
    /// The redefined feature values in the annotation body, in authored order.
    pub body: Box<[MetadataAnnotationValue]>,
    /// The annotation's own source span.
    pub location: SourceLocation,
}

/// One `feature = value;` (or bare `feature;`) entry in a metadata annotation body.
#[derive(Debug, Clone, PartialEq)]
pub struct MetadataAnnotationValue {
    /// The inherited feature of the annotating definition this entry redefines
    /// (`probability` in `@Risk { probability = 0.3; }`). Its authored spelling is recovered from
    /// the resolved feature; when it did not resolve, [`crate::PublishedResolution::inspect`] on
    /// [`Self::value`]'s `element` still carries the entry's own location.
    pub redefined_feature: RelationshipTarget,
    /// The resolved value expression. [`crate::ExpressionOutcome::NotApplicable`] when the entry
    /// wrote no value (a bare `feature;` reopening a nested body).
    pub value: PublishedExpression,
    /// A nested metadata body owned by this entry (`@Risk { totalRisk { probability = 0.3; } }`).
    pub nested: Box<[MetadataAnnotationValue]>,
}
