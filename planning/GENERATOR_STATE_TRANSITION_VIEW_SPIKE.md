# Typed state-transition view query spike for generators

Status: implemented exploratory spike; paired with
[`roc-spec42/STATE_TRANSITION_VIEW_SPIKE.md`](../../roc-spec42/STATE_TRANSITION_VIEW_SPIKE.md).

## Purpose

Add the smallest semantically owned generator capability needed for an external Wasm plugin to render
one model-authored SysML v2 `StateTransitionView`. This spike validates that specialized products can
consume the immutable `PublishedModel` through bounded typed queries without reviving the removed
diagram product or allowing a plugin to reconstruct semantics from generic element inspection.

The first consumer is the Roc plugin in the paired design, and the first interactive surface is the
Spec42 VS Code extension displaying the plugin's SVG artifact. The query is language-neutral and
belongs to Spec42's generator API; it must not contain SVG, Roc, ELK, VS Code, or layout policy.

## Architectural hypothesis

A generator-specific typed projection can preserve the one-semantic-system boundary:

```text
syntax + libraries
  -> immutable PublishedModel
  -> canonical state/view facts
  -> StateTransitionViewProjection query
  -> versioned generator wire DTO
  -> arbitrary generator implementation
```

If the projection cannot be produced without inspecting syntax or guessing from names in the query or
generator layer, the spike has found a missing fact in `sysml_resolution` or the typed query facade.
The fix belongs at that earliest owner, not in the projection adapter.

## Normative basis

The adopted OMG SysML v2 Language specification is authoritative, particularly clause 8.2.3.18
(States Graphical Notation), clause 8.2.3.26 (Views and Viewpoints Graphical Notation), and the Systems
Library `StateTransitionView` definition. The release's extracted graphical BNF and SVG productions
are implementation aids.

The standard view definition says a state-transition view presents states and their transitions,
including nested states, entry/do/exit actions, triggers, guards, effects, and compartments. The spike
publishes only the subset listed below and makes all other states explicit.

## Scope

The vertical slice supports one authored `ViewUsage` typed by `StateTransitionView` that exposes one
state definition with:

- one initial-state marker;
- ordinary state usages;
- one final state;
- transitions with resolved source and target states;
- an optional simple accept trigger; and
- stable identity, display label, source provenance, and deterministic order.

The query owns both discovery of eligible authored views and projection of a selected view. It does
not accept an arbitrary state definition and pretend it was an authored view.

## Non-goals

- Reintroducing built-in diagrams, renderer assets, or server-side ELK.
- Publishing the semantic graph or storage handles.
- A generic serialized graph DTO.
- SVG, layout, styling, fonts, or interaction.
- Exact graphical coverage of nested/parallel states, state actions, guards, effects, control nodes,
  termination, annotations, or dependencies in the first slice.
- Incremental projection caching.
- Coordinate or diagram-layout interchange.
- Live rendering of unsaved LSP buffers, automatic render-on-edit, or a general editor plugin
  registry in the first slice.

## Exemplar contract

Add a small snapshot fixture matching the paired `DoorController` model. It authors a `ViewUsage`
`lifecycle : StateTransitionView` exposing `DoorLifecycle`. The state machine has an initial marker,
`closed` and `open` states, final state `retired`, a triggered transition `closed -> open`, and an
untriggered transition `open -> retired`.

The fixture must first prove the underlying publication contains resolved view typing/exposure and
state-machine facts. The projection snapshot then pins the externally observable result.

## Canonical query result

Use domain types rather than exposing generic `ElementSummary` plus relationship-kind strings. Names
below are provisional, but the distinctions are required:

```rust
pub struct StateTransitionViewSummary {
    pub handle: StateTransitionViewHandle,
    pub semantic_id: String,
    pub name: String,
    pub exposed_machine: StateMachineIdentity,
    pub source: SourceReference,
}

pub struct StateTransitionViewProjection {
    pub schema_version: u32,
    pub model_digest: String,
    pub view: StateTransitionViewSummary,
    pub machine: StateMachineSummary,
    pub nodes: Vec<StateTransitionNode>,
    pub transitions: Vec<StateTransitionEdge>,
    pub completeness: ProjectionCompleteness,
}

pub enum StateTransitionNodeKind {
    Initial,
    State,
    Final,
}

pub struct StateTransitionNode {
    pub semantic_id: String,
    pub label: String,
    pub kind: StateTransitionNodeKind,
    pub source: SourceReference,
}

pub enum TransitionTrigger {
    None,
    Accept {
        label: String,
        target: Option<ElementIdentity>,
        source: SourceReference,
    },
    Unsupported { reason: UnsupportedReason },
}

pub struct StateTransitionEdge {
    pub semantic_id: String,
    pub label: Option<String>,
    pub source: String,
    pub target: String,
    pub trigger: TransitionTrigger,
    pub guard: ProjectionFeature,
    pub effect: ProjectionFeature,
    pub provenance: RelationshipProvenance,
    pub source_reference: SourceReference,
}
```

`ProjectionFeature` distinguishes absent, supported, unsupported, unresolved, ambiguous, and parser
recovery states. Even though the exemplar has no guard or effect, those fields prevent their silent
loss. If the owning semantic layer cannot yet distinguish every state, add the narrower fact required
or mark the entire projection incomplete; do not infer from source text.

Use a dedicated opaque query handle whose validity is scoped to the generator invocation. Stable
semantic IDs remain in the result for provenance and testing, but callers do not manufacture handles.

## Selection semantics

The catalog operation returns only authored `ViewUsage` instances whose resolved direct type has the
canonical identity of the standard `StateTransitionView` and whose exposure can be evaluated from
canonical facts. Supporting specializations through a bounded conformance query is deferred.

For the first slice:

- exactly one exposed state machine is required;
- zero or multiple exposed roots returns a typed unsupported result;
- unresolved or ambiguous typing/exposure remains explicit;
- library views may be identified but are excluded by a documented catalog option or origin field;
- order is canonical publication order with semantic identity as the final tie-breaker; and
- filters are either canonically evaluated or reported unsupported, never approximated by the guest.

Spike discovery: the implementation currently assembles the neutral projection in `generator_api`
over canonical `PublishedModel` inspection. The new semantic snapshot pins its required facts, but
moving projection ownership behind a dedicated `sysml_query` facade remains follow-up work.

This will likely reveal that view typing, expose evaluation, or filter evaluation needs a new owned
fact/query. That is an expected spike result.

## State-machine projection semantics

The projection is assembled only from canonical facts:

- membership/ownership selects direct state-machine members;
- the resolved initial-state target produces a synthetic notation node and edge only if the query
  contract explicitly models that transformation;
- state and final-state declaration kinds choose node variants;
- resolved transition source/target facts choose endpoints;
- resolved trigger facts choose the trigger variant;
- authored/implied provenance remains distinguishable; and
- source ranges refer to authored declarations or references, never synthesized display text.

The recommended representation includes the initial pseudostate as a node and its connection as a
typed edge so guests do not independently invent IDs or ordering. Synthetic identities must be
deterministic and scoped to the projection schema version.

Labels are presentation facts supplied by a canonical display-label policy. A guest must not choose
between declared name, effective name, qualified name, or trigger target spelling itself. If Spec42
does not yet own this policy, add a deliberately scoped notation label result rather than making it a
general semantic name.

## Query facade placement

The canonical projection should be exposed by `sysml_query` over `PublishedModel`, with no raw semantic
storage in its public API. When new result fields are not representable in snapshot output, extend the
owner-defined snapshot projection and standalone snapshot runner before adding fixtures, as required
by `crates/sysml_query/AGENTS.md`.

Candidate methods:

```rust
PublishedModelQueries::state_transition_views(options)
PublishedModelQueries::state_transition_view(handle, options)
```

Both operations are bounded. Limits cover catalog results, projection nodes, transitions, label bytes,
and total encoded response bytes. Limit failures report actual and configured values and never return
a truncated successful projection.

## Generator API and ABI changes

Add two declared generator operations:

```text
StateTransitionViews = next_code => (StateTransitionViewCatalogRequest,
                                     StateTransitionViewCatalogResponse)
StateTransitionView  = next_code => (StateTransitionViewRequest,
                                     StateTransitionViewProjection)
```

The existing single `spec42.query` import is sufficient; no new Wasm import is expected. This is a
wire-schema and operation-table change, so implementation must update the authoritative declaration
and regenerate or update all derived contract artifacts together:

- `generator_protocol` request/response and domain enums;
- the `abi_contract!` operation table;
- schema fingerprint and compatibility token;
- ABI/semantic version according to their policies;
- checked-in generator ABI manifest and documentation;
- host operation dispatch;
- `generator_api` implementation over the immutable publication;
- Rust SDK calls and public re-exports;
- generator host conformance plugins/cases; and
- downstream guest adapters, including `roc-spec42` on the new pinned revision.

Because incompatible guests are already rejected by compatibility token, the spike should use the
normal deliberate-breaking-change path rather than adding an untyped escape hatch. Whether the numeric
`ABI_VERSION` also increments must follow the repository's documented version policy; the compatibility
token necessarily changes.

## VS Code host integration

The vertical slice includes a viewer in `vscode/`, but it does not add editor concepts to the semantic
query or generator ABI. The extension contributes `Spec42: Open State Transition View`, sends bounded
base64 bytes for the packaged Roc plugin through `spec42/generate`, and displays the returned SVG in a
readonly webview panel.

Before generation, the extension calls `spec42/stateTransitionViews` against the same immutable
publication. It filters the typed catalog by the active document's source provenance. One match opens
directly; multiple matches are shown in a Quick Pick with view and exposed-machine labels. The chosen
opaque handle and catalog model digest are passed to `spec42/generate`, which rejects a stale selection
if the publication changed in between.

For the spike, generation operates on saved workspace files. If a relevant SysML document is dirty,
the command asks the user to save or cancel. Generation then consumes the current coherent LSP
`PublishedModel`; it never builds a second publication from disk.

Spec42 release packaging must provide an immutable way for the extension to locate the matching
generator plugin. Acceptable spike choices are:

- bundle the prebuilt plugin with the VS Code extension and record its Spec42 compatibility token; or
- bundle it in the Spec42 release and expose a deterministic installed path/manifest entry.

Do not compile or download the plugin when the command runs. A host/plugin compatibility failure is
shown as a rendering failure and must not silently fall back to an older artifact.

The persistent LSP generator service reuses the current immutable publication, one Wasmtime engine,
and prepared modules keyed by the SHA-256 digest of the exact guest bytes. The normal CLI uses the same
host and enables Wasmtime's cross-process compiled-module cache. Cache entries are disposable: a
missing, corrupt, unwritable, or incompatible entry falls back to canonical compilation.

The extension remains a thin adapter responsible for:

- selecting the active model and authored view;
- applying the saved-document policy;
- sending bounded plugin bytes and the saved model URI over the existing language client;
- rejecting superseded responses;
- selecting the expected `image/svg+xml`/`.svg` artifact;
- applying a restrictive webview content-security policy;
- displaying model digest and view identity;
- validating source-navigation messages before revealing a source range; and
- marking retained output stale after a failed or superseded regeneration.

The plugin's standalone SVG provenance attributes form the minimal viewer contract. They carry
semantic identity and bounded source location, not a second serialized semantic graph. The extension
must not infer meaning from CSS classes or SVG labels.

### Deferred unsaved-editor path

The generic LSP generator request returns bounded artifact bytes without renderer semantics. The
viewer deliberately retains its save-all policy. Supporting intentional generation from dirty buffers
later requires explicit UX and identity/staleness policy, not another diagram DTO or semantic path.

## Errors and completeness

Expected typed outcomes include:

- view not found or stale handle;
- wrong view kind;
- unresolved or ambiguous view typing;
- unsupported expose/filter expression;
- zero or multiple exposed machines;
- unresolved/ambiguous transition endpoint;
- unsupported trigger, guard, or effect form;
- parser recovery affecting a required fact; and
- result or encoded-response limit exceeded.

The design decision to validate in the spike is whether these travel as a successful projection with
typed per-item completeness or as a query error. The default is:

- no projection when selection or required graph topology is not trustworthy;
- a projection with explicit unsupported optional notation features when topology remains trustworthy;
- no silent omission in either case.

## Determinism and identity

The result is a pure function of the `PublishedModel` identity, query/projection schema version, and
explicit options. Canonicalize ordering at the projection owner. Do not depend on hash-map iteration,
query traversal order, cache warmth, or plugin request order.

No generated-result or semantic-projection cache is introduced. The Wasmtime native-code cache is
strictly a compilation accelerator keyed and compatibility-checked by Wasmtime; it never stores model
facts or generator results. The LSP's prepared-module map is similarly keyed only by exact guest bytes
and belongs to one engine lifetime.

## Verification

1. Extend standalone snapshot output for any newly exposed owned facts.
2. Add the exemplar fixture and snapshot its semantic publication.
3. Snapshot catalog and selected projection results, including canonical ordering and provenance.
4. Add negative fixtures for ambiguous exposure, unresolved endpoint, unsupported feature, and limit
   overflow.
5. Add generator API unit tests only for boundary mechanics that snapshots cannot own.
6. Add protocol round-trip, schema fingerprint, unknown-operation, and stale-guest rejection coverage.
7. Add generator conformance cases using a Rust guest before relying on the Roc adapter.
8. Run the paired Roc end-to-end generation and compare the SVG golden.
9. Package the compatible Roc plugin for VS Code and cover command invocation, failure, artifact
   selection, webview security, stale identity, and source navigation.

## Work sequence

1. Confirm the exemplar's current parser/resolution facts through `spec42-snapshot`.
2. Identify missing canonical view typing/exposure/state facts.
3. Add facts at their semantic owner and snapshot them.
4. Add the typed `sysml_query` projection and projection snapshots.
5. Extend the generator protocol, API, SDK, host dispatch, manifest, and conformance suite.
6. Publish or pin the revision consumed by `roc-spec42`.
7. Complete the paired Roc plugin and deterministic SVG artifact.
8. Add the VS Code command, packaged-plugin discovery, readonly webview, and extension tests.
9. Feed discovered semantic, ABI, packaging, and editor-host contract gaps back here while active.

## Exit criteria

- A generator discovers the authored exemplar view without name or metaclass guessing.
- The selected projection comes only from one immutable `PublishedModel` identity.
- Nodes, initial/final semantics, transition endpoints, trigger, provenance, order, and completeness are
  typed and covered by snapshots.
- The ABI change is declared once, compatibility-checked, documented, and covered by conformance tests.
- The Roc guest renders the projection end to end without generic semantic reconstruction.
- The VS Code extension invokes the compatible packaged guest against saved exemplar sources,
  displays the resulting SVG, and navigates validated provenance back to source.
- Failed or superseded generation cannot make an older preview appear current.
- Negative semantic states cannot be mistaken for a complete renderable projection.
- Resource limits and deterministic ordering are executable contracts.

After completion, move enduring query/ABI decisions to their owning documentation, retain only active
gaps in planning, and delete this spike document when it has no remaining decisions or work.
