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
is semantic-ID order unless a view owns a stronger authored order. Diagram JSON schema version 6
is the sole renderer product for this contract.

Projected element `name` is the publication's canonical effective name (SysML 7.6.5), so an
unnamed redefinition displays the name it inherits from its first redefined feature. This does
not replace the syntax-owned authored identification or its naming provenance. An absent or
unresolved effective name stays absent; consumers do not reconstruct it from qualified names,
redefinition edges, source text, or product-local layout keys.

A custom view definition inherits the most specific standard diagram kinds reachable through its
canonical subclassification closure: `ActionFlowView` and `StateTransitionView` specialize
`InterconnectionView`, so a view typed by either is not also an interconnection view. Multiple
paths to one kind yield one catalog entry. Effective view
conditions include inherited conditions; metaclass tests include specializing kinds, and Boolean
negation preserves unresolved, ambiguous and unsupported predicate outcomes.

## View inputs and projections

| View | Authoritative semantic inputs | Projection |
| --- | --- | --- |
| General | Resolved exposed roots; owned declarations; containment; published relationships and authored FeatureTyping; relationship provenance; source locations | Exposed roots plus their owned semantic scope as typed nodes; containment edges; and, between two projected elements, an edge for each authored subclassification, subsetting, redefinition, or feature typing (SysML 8.2.3.6). Implied library subsetting is a query-only fact, not part of the product; an unresolved, ambiguous, or out-of-view end yields no edge |
| Interconnection | Exposed parts; ports; nested ownership; connector declarations and resolved connector ends; authored direction/conjugation when published; the direction a port's directed features give it (`featureDirection`), derived and kept apart from the authored one; provenance | Nested part nodes with ports on the node boundary and connector edges (SysML 8.2.3.11). Ports and connector usages are not peer boxes. Missing or unsupported connector ends/direction remain typed incomplete facts |
| Action flow | Exposed action definitions/usages and control nodes; resolved successions/flows, including the source the lowering settles for a `then` shorthand (SysML 7.17.4); the inherited `start`/`done` where a succession begins or ends at them; directed features (parameters) with their authored direction; the guard of a conditional succession as written, or `else` (`successionGuard`); effects when published | Action/control nodes, start and done nodes, parameters on the boundary of their action, and ordered flow edges. A consumer does not work out the source of a `then` from member order. Unsupported guards or effects are explicit reasons |
| State transition | Exposed state definitions; states and the states they own; final nodes; initial/succession and transition endpoints; trigger/guard/effect facts; the entry, do and exit action of a state with its role (`stateSubaction`); provenance | State/control nodes, a state's nested states inside it, its state actions, and transition edges with typed trigger, guard, effect, source, and completeness |
| Sequence | Exposed participants/lifelines; authored sends/messages and resolved ends; authoritative ordering; activations/fragments only when published | Lifeline columns and message edges. No activation, fragment, or message order is fabricated |
| Browser | Resolved exposure plus canonical ownership facts and source identities | Collapsible ownership tree rooted only at exposed scope: each element's owned members, siblings in declaration order. Features a usage has only through its types are not repeated beneath it (browse the definition for them), and lowering structure (flow ends and their descendants, value expressions) is not a member to browse. Authored anonymous features remain members; an absent name does not identify lowering structure |
| Grid | Explicit exposed row elements and published typed relationships suitable for selected matrix cells | Typed rows, columns, and cells. Labels never create a cell or column |
| Geometry | Authored or evaluated geometry facts and their source/provenance | Provisional 2D primitives only for authoritative coordinates. Otherwise the exposed elements remain present with typed geometry incompleteness |

## Explicit degraded states

Unresolved and ambiguous exposure, unsupported syntax or features, parser recovery, non-converged
publication, absent authoritative geometry, and incomplete relationship endpoints are distinct
typed reasons. An empty resolved exposure is a valid complete empty projection when the view kind
permits it. Elements outside the transitive semantic scope of resolved exposures are never added.
An empty drawing explains that no elements are selected and directs the reader to the view's
exposes and filters. It does not turn an empty selection into an incomplete result or render
elements excluded by the view's filters.

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

The product's `relationships` are the projection's **authored** relationships. A projection also
publishes the relationships the library implies for each element (`Parts::parts` subsetting, type
featuring, inherited redefinitions); those stay available through the semantic query and
inspection, but the product leaves them out. No drawing reads them, and with the references,
sources and library documents they pulled in they were most of every product's bytes (schema 6;
see the CHANGELOG). The language server sends the product as compact JSON; `spec42 diagram
--format json` and checked-in products are pretty-printed. Both are the same JSON value.

Graph endpoints and view-specific metadata use node or relationship indexes rather than repeating
qualified names. The renderer expands those local joins once into layout keys such as `n:17`; this
is presentation adaptation, not semantic reconstruction. A future incremental product must use a
base `modelDigest` and public semantic references, never indexes from an earlier product.

Interconnection containment distinguishes noncomposite memberships just as General View does.
The renderer keeps item occurrences that own shown parts as item containers, including items
reached through boundary ports. A nested port uses its supplied identity and owner chain to attach
to the nearest drawn structural node with a qualified boundary label. This display attachment
does not change semantic ownership or connector endpoints. Closing a structural node hides its
complete item/part subtree; a descendant never becomes a loose root because its owner is an item.

## Generated consumer contract

The schema-6 wire shape is declared in `crates/diagram_product/src/wire.rs`. Its `projection` is a
discriminated union keyed by the view kind, so each view carries only its own metadata and scene
shape. The Rust publisher validates every product against this contract before returning it.

`cargo run -p diagram_product --bin diagram_bindings` generates the checked-in TypeScript types in
`vscode/src/generated/diagram-product`. Both the VS Code product parser and the diagram renderer
import these generated declarations. `scripts/minici.sh lint` runs the generator in check mode and
fails when the Rust contract and checked-in TypeScript files differ.

Schema 6 keeps the schema-5 field names and shapes; it changes what `relationships` contains (see
above), and a schema-5 product is no longer accepted. The renderer's legacy pre-schema-5 adapters
remain boundary-only compatibility code. A future incompatible wire change must increment `schemaVersion`, regenerate
the bindings, update the checked-in product corpus, and migrate consumers in the same change.

## Root-view framing

The native renderer owns framing for General, Interconnection, Action Flow, State Transition,
and Sequence. Each top-level drawing has exactly one rectangular view frame with the header
`«view» <selected view name>`, following OMG SysML v2 formal/2026-03-02 §§8.2.3.1 and 8.2.3.26
(`root-view = framed-view`). Frameless graphical compartments inside element symbols do not
create additional root frames. Action (§8.2.3.17), state (§8.2.3.18), package and part boundaries
remain semantic containers, even when their names equal the view name. An exposed machine's
name is not the selected view's identity.

Frame layout happens after content placement. Node and route extents plus the actual drawing's
translated text, container rectangles, boundary ports and labels contribute to conservative
frame bounds. The header has reserved space and is never truncated; viewer fit and SVG/PNG
exports use the resulting bounds. Disclosure changes content visibility, not the view identity
or the existence of the root frame. The presentation uses existing schema-5 product fields;
there is no semantic selection, wire-schema or request-shape change. Browser, Grid and Geometry
are outside this standard graphical framing policy.
