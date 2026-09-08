//! The published connection-topology contract.
//!
//! [`crate::PublishedResolution::binding_connectors`] pairs the two ends of every `bind`; this
//! is its sibling for the structural connectors -- `connect` and `interface` usages. A consumer
//! that has to *interpret* a model (emit a wiring table, an interface stub, an analysis model
//! assembled from the connection graph) needs the topology as resolved structure, not a diagram
//! projection or a set of `ConnectedElement` pairs.
//!
//! It adds no analysis: every connector end is an authored reference resolution already settled.
//! A dotted end (`a.b.port`) carries the resolved *terminal* feature plus the path exactly as
//! authored; the intermediate hops are not yet resolved to identities (the resolver keeps no
//! per-hop state), so a consumer that needs them reads the authored segments.

use crate::inspection::RelationshipTarget;
use crate::{MultiplicityFacts, SourceLocation, SymbolId};

/// Whether a connector is a `connect` (connection) or an `interface`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectorKind {
    Connection,
    Interface,
}

impl ConnectorKind {
    /// A stable kebab-case name, for snapshot output.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Connection => "connection",
            Self::Interface => "interface",
        }
    }
}

/// One resolved end of a connector.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectorEndpoint {
    /// A single-segment end (`connect a to b`): the feature the end names.
    Feature(RelationshipTarget),
    /// A dotted end (`connect a.b.port to ...`): the resolved terminal feature, plus the path
    /// as authored. Intermediate hops are not resolved to identities.
    FeatureChain {
        terminal: RelationshipTarget,
        authored: Box<str>,
    },
    /// An end declared but not wired to a target — a `connection def` / `interface def` body's
    /// participant slot (`end from : PowerPort;` with no `::>` / inline `connect`). Its type and
    /// multiplicity are on [`PublishedConnectorEnd::declaration`].
    Unconnected,
}

/// One end of a connector.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublishedConnectorEnd {
    /// The end's own feature, for a named end (`connect x references a.b`, `end x ::> a.b;`, or a
    /// `connection def` participant slot); `None` for a bare positional end (`connect a to b`).
    /// Read its name and modifiers from this identity.
    pub declaration: Option<SymbolId>,
    /// The end's authored multiplicity; [`MultiplicityFacts::Absent`] for a bare end.
    pub multiplicity: MultiplicityFacts,
    pub endpoint: ConnectorEndpoint,
}

/// One `connect` / `interface` connector with its type and resolved ends.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublishedConnector {
    /// The connector declaration (or the synthesized scope of a keyword-less `connect a to b;`).
    /// Distinct authored connectors stay distinct even with identical endpoints.
    pub identity: SymbolId,
    pub kind: ConnectorKind,
    /// The connector's `:` declared type (a `connect` / `interface` usage's typing).
    /// [`RelationshipTarget::Unsupported`] when none was authored — including for a `connection
    /// def` / `interface def`, whose `:>` supertype is read through the type queries
    /// (`types().direct_types(..)`) rather than reported here.
    pub declared_type: RelationshipTarget,
    /// The ends in authored order. An end whose path is a shape the resolver does not model
    /// (`connect a to f()`) produces no resolvable reference and is omitted — the resolver
    /// reports it as an `unsupported` diagnostic instead.
    pub ends: Box<[PublishedConnectorEnd]>,
    pub location: SourceLocation,
}

/// The `connect` / `interface` topology reachable from one root element.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublishedConnectionGraph {
    pub root: SymbolId,
    /// Every connector owned (directly or transitively) by `root`, in canonical order.
    pub connectors: Box<[PublishedConnector]>,
}
