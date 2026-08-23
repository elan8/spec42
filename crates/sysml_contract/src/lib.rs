//! The vocabulary every SysML answer is spoken in, and nothing that computes one.
//!
//! This crate defines the semantic value types -- element kinds, visibilities, relationship
//! families, outcome and prerequisite enums, positions and ranges, diagnostic severities and
//! codes -- together with the opaque identity newtypes and the sealed traits that let an
//! authority hand back a borrowed view instead of an owned collection. It computes no semantic
//! fact, holds no state, performs no I/O, and names no authority: `sysml_resolution`
//! *implements* this contract, `sysml_query` *re-exports* it verbatim, and consumers depend on
//! it only through the facade. A rename inside the authority is then invisible; a change here is
//! a deliberate, versioned contract change.

#![forbid(unsafe_code)]

mod diagnostic;
mod element;
mod element_kind;
mod position;
mod relationship;
mod version;

pub use diagnostic::{DiagnosticCategory, DiagnosticOrigin, DiagnosticSeverity};
pub use element::{
    AnnotationForm, AuthoredValue, ElementDerivedDocumentationCollection, ElementModifier,
    ElementSearch, ElementSource, FeatureDirection, MembershipFacts, MembershipKind,
    MultiplicityBound, MultiplicityFacts, PortionKind, ValueKind, Visibility, VisibilityProvenance,
};
pub use element_kind::{
    ElementKind, MembershipRole, RequirementConstraintKind, StateSubactionKind,
};
pub use position::{OccurrenceRole, TextPosition, TextRange};
pub use relationship::{
    BindingConnectorValidationOutcome, BindingConnectorValidationPrerequisite, Conformance,
    ConformanceObstacle, RedefinitionCheckOutcome, RedefinitionCheckPrerequisite,
    RelationshipOutcome, RelationshipProvenance, SatisfyPolarity, SpecializationCheckOutcome,
    SpecializationCheckPrerequisite, SpecializationScope, SubsettingConformance,
    TypeFeaturingCheckOutcome, TypeFeaturingCheckPrerequisite,
};
pub use version::{SemanticContractVersion, SEMANTIC_CONTRACT_VERSION};
