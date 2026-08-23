# Proposal A — a contract-types crate for the facade vocabulary

## 1. Inventory of what `sysml_query` re-exports

Counted from the `pub use` blocks in each facade module (workspace at `sole-authority-pipeline`).

| Module | Re-exported names | Source |
|---|---|---|
| `resolved_slice.rs` | 142 (+ `SourceKind`) | `sysml_resolution::{…}` |
| `source.rs` | 17 + `identity` module alias | `sysml_resolution::source` |
| `syntax.rs` | 13 (incl. `RESERVED_KEYWORDS`, 2 fns) | `sysml_resolution::syntax` |
| `publication.rs` | 8 + `Session<PublishedModel>` alias | `sysml_resolution::publication` |
| `library.rs` | 3 | `sysml_resolution::library` |
| **Total** | **~184 names** | |

Of the 142 in `resolved_slice.rs`: **83 enums**, **51 structs**, 8 names that are aliases/trait-ish
(the `*CheckKind` / `*DerivedFactKind` families defined via macro or re-export chains).

### Classification

**(a) Pure contract value types — ~104.** Field-free or scalar-field enums (`ElementKind`,
`Visibility`, `FeatureDirection`, `DiagnosticSeverity`, `SatisfyPolarity`, `PortionKind`,
`MembershipKind`, `RelationshipProvenance`, all the `*Kind` / `*Outcome` / `*Prerequisite`
families), plus `Copy` structs whose fields are enums (`ElementSearch`, `EffectiveTypeEntry`
minus its name, `MultiplicityBound`). These are the real contract: they name semantic
distinctions, not storage. `QueryOutcome<T>` is contract-shaped except for its `Ambiguous(Box<[T]>)`
arm.

**(b) Handle / opaque — 12.** `PublishedModel`, `ParsedSource`, `SourceDocument`, `LibraryClosure`,
`PublicationSession`/`Session<_>`, `BuildToken`, `RelinkToken`, `PublicationToken`,
`SourceAuthority`, `ContentDigest`, `RootDigest`, `SymbolIdentity`. Correctly opaque already,
except `SourceAuthority` (an authority type visible on the facade purely so external providers
can admit) and `SymbolIdentity`, whose `as_str()` pins it to a string forever.

**(c) Leaks representation — 34 named below.** Two leak families:

*C1 — owned collection fields (`Box<[T]>`) that force the authority to materialise a vector per
query and forbid a dense/arena layout (14):*

| Type | Leaking field(s) |
|---|---|
| `Diagnostic` | `related: Box<[RelatedLocation]>` |
| `PublishedDiagnostics` | `diagnostics: Box<[Diagnostic]>` |
| `ElementDetails` | `inherited_features`, `metadata`, `incoming`, `outgoing` |
| `ElementInspection` | `documentation`, `modifiers`, `relationships` |
| `EffectiveTyping` | `types`, `candidates` |
| `RelationshipFamily` | `targets`, `candidates` |
| `ElementEvaluation` | `units: Box<[AuthoredUnit]>` |
| `ResolvedUnit` | `dimensions: Box<[SymbolIdentity]>` |
| `DiagramViewProjection` | `exposed_roots`, `elements`, `relationships`, `edges`, `incomplete_reasons` |
| `DiagramStateTransitionScene` | `vertices`, `transitions` |
| `DiagramElement` | `compartments` |
| `DiagramCompartment` | `members` |
| `DiagramOccurrenceIdentity` | `semantic_path: Box<[SymbolIdentity]>` |
| `QueryOutcome<T>` | `Ambiguous(Box<[T]>)` |

*C2 — owned string fields that make every read an allocation and freeze the interning strategy (20):*
`AffectedDocument.identity`, `AuthoredUnit.authored`, `Diagnostic.message`,
`DiagnosticLocation.document`, `DiagramEdge.semantic_id`, `DiagramElement.name`,
`DiagramRelationship.{semantic_id,kind}`, `DiagramStateTransition.{semantic_id,label}`,
`DiagramStateVertex.label`, `DiagramViewCatalogEntry.name`, `Documentation.{locale,language,text}`,
`ElementInspection.{name,short_name,qualified_name}`, `ElementRelationship.authored`,
`NavigationTarget.name`, `QualifiedElementReference.{document,qualified_name}`,
`QualifiedReferenceTarget.qualified_name`, `RelatedLocation.message`, `SourceLocation.document`,
`SymbolEntry.{name,qualified_name}`, `VisibleMember.{name,qualified_name,container_name}`.

`DiagramRelationship.kind: Box<str>` is additionally *stringly typed dispatch* — an AGENTS.md
violation on the facade surface.

Also worth flagging in (c): `TextPosition{line: u32, character: u32}` is fine as a value but is
LSP-shaped; `LibraryClosureService::seed_signature -> Vec<String>` returns a raw signature the
consumer could be tempted to interpret; and `BuildMeasurements` exposes timing as plain numbers.

## 2. Proposed crate

**Decision: a new crate, `sysml_contract`. Do not overload `source_identity`.**

`source_identity` owns one thing — content digests and the std-only authority guard home — and it
is deliberately dependency-free below `sysml_source`. Moving 180 semantic vocabulary types into it
would make "identity" mean two unrelated things and would put semantic contract types *below* the
source authority, where they do not belong.

| Crate | Role | May depend on |
|---|---|---|
| `sysml_contract` | the vocabulary every SysML answer is spoken in, and nothing that computes one. It defines the semantic value types — element kinds, visibilities, relationship families, outcome and prerequisite enums, positions and ranges, diagnostic severities and codes — together with the opaque identity newtypes and the sealed traits that let an authority hand back a borrowed view instead of an owned collection. It computes no semantic fact, holds no state, performs no I/O, and names no authority: `sysml_resolution` *implements* this contract, `sysml_query` *re-exports* it verbatim, and consumers depend on it only through the facade. A rename inside the authority is then invisible; a change here is a deliberate, versioned contract change. | `source_identity` only |

Dependency position: leaf beside `sysml_source`, above `source_identity`.

```text
source_identity ──► sysml_contract ──► sysml_source ──► sysml_resolution ──► sysml_query
```

`sysml_contract` depends on `source_identity` so `ContentDigest`/`RootDigest` can appear in
contract types (`SourceLocation`, `PublicationIdentity`) without duplication. `sysml_source` and
`sysml_resolution` both depend on it.

**May live there:** plain enums; `Copy` value structs; newtype ids; `TextPosition`/`TextRange`/
`SourceLocation`; `DiagnosticSeverity`/`Category`/`Code`; the sealed view traits; the
`SEMANTIC_CONTRACT_VERSION` constant; `serde` impls behind a feature.

**May not live there:** anything with a method that derives a fact; any `Vec`/`HashMap` field on a
public struct; any type holding an `Arc` to authority state; parser, `sysml_source`, or tokio
dependencies; any I/O.

**Consumer rule stays unchanged:** `sysml_query` is still the only crate a consumer *names*. Add a
`deny.toml` ban `sysml_contract` with `wrappers = ["sysml_source", "sysml_resolution", "sysml_query"]`
so consumers reach the vocabulary only through the facade's re-export.

## 3. Category (c) replacements

The pattern: an owned struct becomes an opaque handle plus borrowed accessors; a `Box<[T]>` field
becomes an `impl Iterator` or an indexed sealed view; a `Box<str>` becomes `&str` on the view (or
`Arc<str>` where the value genuinely escapes).

```rust
// 1. Owned collection field -> borrowed view + iterator
// before
pub struct PublishedDiagnostics { pub diagnostics: Box<[Diagnostic]>, /* … */ }
// after
pub struct DiagnosticsView<'m> { /* private */ }
impl<'m> DiagnosticsView<'m> {
    pub fn len(&self) -> usize;
    pub fn get(&self, i: usize) -> Option<DiagnosticRef<'m>>;
    pub fn iter(&self) -> impl Iterator<Item = DiagnosticRef<'m>> + 'm;
}

// 2. Owned string field -> borrowed accessor
// before
pub struct Diagnostic { pub message: Box<str>, pub related: Box<[RelatedLocation]>, /* … */ }
// after
pub struct DiagnosticRef<'m> { /* private */ }
impl<'m> DiagnosticRef<'m> {
    pub fn severity(&self) -> DiagnosticSeverity;      // (a) value type, by value
    pub fn code(&self) -> DiagnosticCode;
    pub fn message(&self) -> &'m str;                   // no allocation
    pub fn related(&self) -> impl Iterator<Item = RelatedLocationRef<'m>> + 'm;
}

// 3. Public identity string -> opaque newtype id
// before
pub struct SymbolIdentity(Box<str>);
impl SymbolIdentity { pub fn as_str(&self) -> &str; }
// after
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SymbolId(NonZeroU32);                        // dense index today, anything tomorrow
impl PublishedModel { pub fn qualified_name(&self, id: SymbolId) -> &str; }
// serialisation crosses the boundary explicitly:
impl PublishedModel { pub fn symbol_token(&self, id: SymbolId) -> SymbolToken; } // stable across a publication

// 4. Stringly typed dispatch -> exhaustive enum
// before
pub struct DiagramRelationship { pub kind: Box<str>, pub semantic_id: Box<str>, /* … */ }
// after
pub enum DiagramRelationshipKind { Specialization, Subsetting, Redefinition, Featuring, /* … */ }
impl<'m> DiagramRelationshipRef<'m> { pub fn kind(&self) -> DiagramRelationshipKind; }

// 5. Nested owned scene -> lazily walked projection
// before
pub struct DiagramViewProjection {
    pub elements: Box<[DiagramElement]>, pub relationships: Box<[DiagramRelationship]>, /* … */ }
// after
pub struct DiagramViewProjection<'m> { /* private */ }
impl<'m> DiagramViewProjection<'m> {
    pub fn elements(&self) -> impl ExactSizeIterator<Item = DiagramElementRef<'m>> + 'm;
    pub fn relationships(&self) -> impl ExactSizeIterator<Item = DiagramRelationshipRef<'m>> + 'm;
    pub fn incomplete_reasons(&self) -> impl Iterator<Item = DiagramIncompleteReason> + 'm; // (a) Copy
}

// 6. Generic outcome with an owned ambiguity list
// before
pub enum QueryOutcome<T> { Resolved(T), Ambiguous(Box<[T]>), Unresolved, /* … */ }
// after
pub enum QueryOutcome<T, A = Empty> { Resolved(T), Recovered(T), Ambiguous(A), Unresolved, /* … */ }
// callers instantiate A = impl Iterator<Item = T>, so ambiguity costs nothing when absent.

// 7. Labels that genuinely escape the borrow -> Arc<str>, never String
// before
pub struct VisibleMember { pub name: Box<str>, pub qualified_name: Box<str>, /* … */ }
// after
impl<'m> VisibleMemberRef<'m> {
    pub fn name(&self) -> &'m str;
    pub fn name_owned(&self) -> Arc<str>;   // explicit, opt-in, shares the authority's intern pool
}

// 8. Sealed trait so a view type cannot be implemented or forged outside the authority
mod sealed { pub trait Sealed {} }
pub trait SemanticView<'m>: sealed::Sealed + Copy { fn model_identity(&self) -> ModelIdentityRef<'m>; }
```

`Ref` views are `Copy` and lifetime-bound to the `PublishedModel`, so a consumer cannot outlive
the publication it read from — the "readers never observe a half-applied state" invariant becomes
a borrow-checker fact rather than a convention.

## 4. The semantic contract version

**It exists, as a named constant, in the wrong crate.**

- `crates/sysml_resolution/src/lib.rs:35` — `pub const RESOLVED_CONTRACT: &str = "parser-owned-resolution-v1";`
- It is stored per publication as `PublicationIdentity.semantic_contract_version: Box<str>`
  (`lib.rs:363`), read via `semantic_contract_version()` (`:375`) and hashed into `model_digest()`
  (`:389`), and written into the serialised model (`model/resolver/writer.rs:14,270,289`).
- It is **not re-exported by `sysml_query`** — no consumer can name it, and nothing outside
  `sysml_resolution` asserts its value. The unrelated `workspace::version` constants
  (`PROJECTION_SCHEMA_VERSION = 17`, `COMPARISON_SCHEMA_VERSION = 2`) are host artefact schemas,
  not the semantic contract.

**Proposal.** Move it to `sysml_contract` as
`pub const SEMANTIC_CONTRACT_VERSION: SemanticContractVersion;` with a typed newtype rather than a
bare `&str`, and make `PublicationIdentity` hold that `Copy` newtype instead of a `Box<str>`.
This is exactly the AGENTS.md rule "declare each repository-owned contract value once": the crate
that *defines* the vocabulary should carry the version of that vocabulary, and `sysml_resolution`
should be unable to bump it without touching the contract crate. Add a test in `sysml_contract`
asserting the literal value, so a change is a visible diff, and re-export it from `sysml_query`.
Keep `RESOLVED_CONTRACT` as a deprecated alias for one commit.

## 5. Migration order (each commit compiles and is green)

1. **Create `sysml_contract`** — empty crate, workspace member, `source_identity` dependency only.
   Add it to the `design.md` crate table and the mermaid map. No code moves.
2. **Move the version constant.** `SEMANTIC_CONTRACT_VERSION` + `SemanticContractVersion` newtype
   into `sysml_contract`; `sysml_resolution` re-exports under the old name; `PublicationIdentity`
   still stores `Box<str>`. Add the value-assertion test.
3. **Move the leaf value types (category (a), no dependencies).** The ~83 enums and the `Copy`
   structs, in 3–4 commits grouped by domain (diagnostics; element/relationship; diagram;
   derived-fact families). `sysml_resolution` re-exports each verbatim, so `sysml_query` and every
   consumer are untouched. This is the bulk of the win and carries almost no risk.
4. **Flip the facade's `pub use` to `sysml_contract`** for the moved names; delete the pass-through
   re-exports from `sysml_resolution` for anything the authority does not itself use. After this
   commit an authority rename can no longer break the facade for those names.
5. **Introduce `SymbolId`** alongside `SymbolIdentity`, with `PublishedModel` translation methods.
   Migrate consumers crate by crate (`sysml_diagnostics`, `sysml_tokens`, `language_service`,
   `kpar`, generators, hosts). Retire `SymbolIdentity::as_str()` last.
6. **Convert the C1 collection types to views**, one product at a time, `PublishedDiagnostics`
   first (smallest surface, most call sites), then `ElementDetails`/`ElementInspection`, then the
   diagram projection. Each is: add the `*Ref`/`*View` type, port consumers, delete the owned
   struct.
7. **Convert the C2 string fields** as a side effect of step 6 — every one of them lives on a type
   that step 6 already touches.
8. **`DiagramRelationship.kind` -> enum**, with an exhaustive-match test.
9. **Typed `seed_signature`** (`LibraryClosureSignature` newtype instead of `Vec<String>`).

### Guards to update

- `deny.toml`: add the `sysml_contract` ban with
  `wrappers = ["sysml_source", "sysml_resolution", "sysml_query"]` in commit 1 — before anything
  moves, so no consumer can pick up a direct dependency during the migration.
- `crates/source_identity/tests/authority_chain.rs`: the chain assertions must accept the new node
  and assert its position (below `sysml_source`, above `source_identity`, and that
  `sysml_contract`'s manifest names no other SysML crate).
- `crates/sysml_query/tests/architecture.rs`:
  - `facade_depends_only_on_the_immutable_resolution_owner` — now `sysml_resolution` **and**
    `sysml_contract`; tighten it to forbid any third.
  - `query_facade_public_api_contains_no_raw_semantic_storage` — extend the raw-storage scan to
    `crates/sysml_contract/src`, which is where the storage-free rule now has teeth; and after
    step 6 make it fail on a `pub` field of type `Box<[_]>` or `Box<str>` in the contract crate.
  - `designated_consumers_use_the_query_facade_and_direct_model_dependencies_do_not_expand` — add
    `sysml_contract` to the forbidden direct-dependency list for consumers.
- New test in `sysml_contract/tests/`: the crate's `Cargo.toml` has exactly one SysML dependency
  (`source_identity`) and no `serde`/tokio/parser dependency outside an optional feature.
