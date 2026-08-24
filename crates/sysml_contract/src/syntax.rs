//! The syntax-fidelity vocabulary: what the grammar saw, named rather than spelled.
//!
//! These are the plain value types the syntax service answers in. They carry no owned text: a
//! [`SyntaxOutlineKind`] is a `Copy` enum whose [`keyword`](SyntaxOutlineKind::keyword) accessor
//! borrows the authored keyword, so a host that wants to print it can, and a host that wants to
//! classify it matches instead of comparing strings.

/// What a declaration in the outline *is*, named by the grammar production it came from.
///
/// The authored keyword is an accessor, not the identity: mapping a kind to an editor symbol
/// category is presentation policy, and a host that matches on this enum gets a compile error
/// when a new declaration form is published rather than a silent fall-through.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SyntaxOutlineKind {
    Package,
    LibraryPackage,
    Namespace,
    PartDef,
    PartUsage,
    ItemDef,
    ItemUsage,
    PortDef,
    PortUsage,
    InterfaceDef,
    AttributeDef,
    AttributeUsage,
    FeatureDecl,
    ClassifierDecl,
    ActionDef,
    ActionUsage,
    RequirementDef,
    RequirementUsage,
    AnalysisDef,
    AnalysisUsage,
    VerificationDef,
    VerificationUsage,
    ViewDef,
    ViewpointDef,
    RenderingDef,
    ViewUsage,
    ViewpointUsage,
    RenderingUsage,
    Ref,
}

impl SyntaxOutlineKind {
    /// The authored declaration keyword, for hosts that print it.
    pub fn keyword(self) -> &'static str {
        match self {
            Self::Package => "package",
            Self::LibraryPackage => "library package",
            Self::Namespace => "namespace",
            Self::PartDef => "part def",
            Self::PartUsage => "part",
            Self::ItemDef => "item def",
            Self::ItemUsage => "item",
            Self::PortDef => "port def",
            Self::PortUsage => "port",
            Self::InterfaceDef => "interface",
            Self::AttributeDef => "attribute def",
            Self::AttributeUsage => "attribute",
            Self::FeatureDecl => "feature decl",
            Self::ClassifierDecl => "classifier decl",
            Self::ActionDef => "action def",
            Self::ActionUsage => "action",
            Self::RequirementDef => "requirement def",
            Self::RequirementUsage => "requirement",
            Self::AnalysisDef => "analysis def",
            Self::AnalysisUsage => "analysis",
            Self::VerificationDef => "verification def",
            Self::VerificationUsage => "verification",
            Self::ViewDef => "view def",
            Self::ViewpointDef => "viewpoint def",
            Self::RenderingDef => "rendering def",
            Self::ViewUsage => "view",
            Self::ViewpointUsage => "viewpoint",
            Self::RenderingUsage => "rendering",
            Self::Ref => "ref",
        }
    }

    /// Whether this kind declares a definition (a `… def`) rather than a usage.
    pub fn is_definition(self) -> bool {
        matches!(
            self,
            Self::PartDef
                | Self::PortDef
                | Self::ItemDef
                | Self::InterfaceDef
                | Self::AttributeDef
                | Self::ActionDef
                | Self::RequirementDef
                | Self::AnalysisDef
                | Self::VerificationDef
                | Self::ViewDef
                | Self::ViewpointDef
                | Self::RenderingDef
                | Self::ClassifierDecl
        )
    }
}

impl std::fmt::Display for SyntaxOutlineKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.keyword())
    }
}

/// The shape of an import target: what the authored suffix admits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ImportScope {
    /// `import A::B;` — the named element alone.
    Element,
    /// `import A::B::*;` — the members of the named namespace.
    Members,
    /// `import A::B::**;` — the named namespace and everything beneath it.
    Recursive,
}

impl ImportScope {
    /// The authored suffix this scope is written with, empty for [`ImportScope::Element`].
    pub fn suffix(self) -> &'static str {
        match self {
            Self::Element => "",
            Self::Members => "::*",
            Self::Recursive => "::**",
        }
    }
}
