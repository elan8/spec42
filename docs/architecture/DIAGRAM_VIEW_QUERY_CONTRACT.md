# Diagram view query contract

The immutable resolved publication owns diagram-view selection and meaning. The generator ABI
exposes one catalog and one view-specific projection operation. A catalog handle is scoped to the
publication identity; it is opaque to guests and is rejected when stale or foreign. Renderers may
lay out and style a projection, but may not infer relationships, membership, geometry, direction,
or ordering from labels. Projected node `typing` is the authored FeatureTyping family on every
view that carries element nodes, not the effective-type closure: compact labels must not dump
implied library types inherited through `Parts::parts`, `Items::items`, and the rest of the
kernel chain.

Every catalog entry shares a typed public semantic reference, standard view kind, authored display
name, source URI/range, and completeness. Named elements use a document-scoped qualified name;
authoritative tooling/library IDs may be used when published; unnamed elements use an explicit
source/ownership anchor. Publication-scoped `SymbolId` handles and opaque `SymbolToken` encodings
never cross into diagram JSON. Every projection repeats its public reference and carries ordered
elements, relationships, typed view metadata, provenance, and typed incomplete reasons. Ordering
is semantic-ID order unless a view owns a stronger authored order. Diagram JSON schema version 5
is the sole renderer product for this contract.

## View inputs and projections

| View | Authoritative semantic inputs | Projection |
| --- | --- | --- |
| General | Resolved exposed roots; owned declarations; containment; published relationships and authored FeatureTyping; relationship provenance; source locations | Exposed roots plus their owned semantic scope as typed nodes; containment edges; and, between two projected elements, an edge for each authored subclassification, subsetting, redefinition, or feature typing (SysML 8.2.3.6). Implied library subsetting stays a relationship-only fact; an unresolved, ambiguous, or out-of-view end yields no edge |
| Interconnection | Exposed parts; ports; nested ownership; connector declarations and resolved connector ends; authored direction/conjugation when published; provenance | Nested part nodes with ports on the node boundary and connector edges (SysML 8.2.3.11). Ports and connector usages are not peer boxes. Missing or unsupported connector ends/direction remain typed incomplete facts |
| Action flow | Exposed action definitions/usages and control nodes; resolved successions/flows; guards/effects when published | Action/control nodes and ordered flow edges. Unsupported decisions, merges, forks, joins, guards, or effects are explicit reasons |
| State transition | Exposed state definitions; states; final nodes; initial/succession and transition endpoints; trigger/guard/effect facts; provenance | State/control nodes and transition edges with typed trigger, guard, effect, source, and completeness |
| Sequence | Exposed participants/lifelines; authored sends/messages and resolved ends; authoritative ordering; activations/fragments only when published | Lifeline columns and message edges. No activation, fragment, or message order is fabricated |
| Browser | Resolved exposure plus canonical membership/ownership facts and source identities | Collapsible ownership tree rooted only at exposed scope |
| Grid | Explicit exposed row elements and published typed relationships suitable for selected matrix cells | Typed rows, columns, and cells. Labels never create a cell or column |
| Geometry | Authored or evaluated geometry facts and their source/provenance | Provisional 2D primitives only for authoritative coordinates. Otherwise the exposed elements remain present with typed geometry incompleteness |

## Explicit degraded states

Unresolved and ambiguous exposure, unsupported syntax or features, parser recovery, non-converged
publication, absent authoritative geometry, and incomplete relationship endpoints are distinct
typed reasons. An empty resolved exposure is a valid complete empty projection when the view kind
permits it. Elements outside the transitive semantic scope of resolved exposures are never added.

Projection completeness is scoped to facts the selected query consumes. Publication-wide parser
recovery or unsupported syntax is not itself a diagram reason: a selected exposure, required
relationship, filter, or geometry fact must carry the affected outcome. A resolved exposure of a
typed usage traverses the publication's canonical effective-feature query, preserving authored
versus implied containment provenance. Until the semantic evaluation layer publishes an
The semantic evaluation layer applies all effective view conditions before projection. Conditions
from separate filter memberships are conjunctive, while Boolean composition within a condition
retains its authored meaning. Candidate classification uses canonical semantic kinds and resolved
metadata relationships, never display names. An unresolved, ambiguous, or unsupported predicate
excludes that candidate and publishes the corresponding typed filter-incompleteness reason; no
consumer guesses or independently reevaluates a filter.

The JSON product contains `schemaVersion`, dependency-complete `modelDigest`, `selectedView`,
`documents`, `sources`, `references`, `completeness`, and `projection`. Documents, source ranges,
and public semantic references are interned once. All integer references are zero-based foreign
keys local to that one product: they are not semantic identities, selection handles, or values that
may be persisted independently. Nodes/elements, edges/relationships, rows, columns, cells, and
diagnostics are canonically ordered before indexes are assigned. `selectedView` contains reference
and source indexes, kind, and name; every projected fact retains a typed public reference and
provenance through the normalized tables.

Graph endpoints and view-specific metadata use node or relationship indexes rather than repeating
qualified names. The renderer expands those local joins once into layout keys such as `n:17`; this
is presentation adaptation, not semantic reconstruction. A future incremental product must use a
base `modelDigest` and public semantic references, never indexes from an earlier product.
