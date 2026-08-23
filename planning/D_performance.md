# D — Performance and representation proposal

Scope: data layout of the published model, the query path through `sysml_query::resolved_slice`,
the construction path, benchmarks, and the facade abstraction needed so the representation can move
without touching consumers. No code changes; all claims carry `file:line` evidence.

## 0. Headline

**The internal representation is already close to the target.** `SemanticModelStorage` is
struct-of-arrays over `Box<[T]>` with `u32` ids, an interned symbol table and a path arena; every
resolver index is CSR-style (`ranges: Box<[(u32,u32)]>` + a flat payload array). The performance
debt is *not* in storage. It is in three places:

1. a per-declaration **canonical identity string** built by cloning the parent's identity;
2. the **public fact types**, which are `Box<str>`-per-field, so every query result re-allocates
   names, qualified names and document URIs that already exist interned in storage;
3. a handful of **O(n) full-storage scans** in query functions that a resident index already
   answers in O(1), plus **no workspace-side incremental reuse** (only the library stratum is
   reused).

Fixing 1–3 needs no arena migration. What it needs is a facade change: `SymbolIdentity` and the
`Box<str>` fields are what pin the representation, not the storage.

---

## 1. Data-layout audit

### 1.1 Storage (good — already the target shape)

`crates/sysml_resolution/src/model.rs:2977` — `SemanticModelStorage`:

```
documents / declarations / declaration_facts / memberships / references /
documentation / feature_values / unsupported / recovery /
evaluation_facts / unit_tokens / filter_conditions / invocations : Box<[T]>
symbols: SymbolTable,  paths: SymbolPathArena
```

- Ids are `#[repr(transparent)] struct X(u32)` minted by a macro (`model.rs:76-94`), with
  `DocumentId`, `DeclarationId`, `SymbolId`, `SymbolPathId`, `AuthoredReferenceId` (`model.rs:95-99`).
- `Declaration` (`model.rs:2842`) is 6 fields, all ids/enums/`Span` — **zero allocations per
  element**. Names are `Option<SymbolId>`, not `String`.
- `DeclarationFacts` (`model.rs:2435`) is a parallel dense side table; the doc comment at
  `model.rs:2392` explicitly rejects `Vec<String>` labels in favour of typed flag bits.
  `DeclarationModifiers` is 16 `bool`s — a candidate for a `u16` bitset, but that is noise-level.
- `MultiplicityRecord`, `MembershipRecord`, `AuthoredReference` — all id + enum + `Span`.

Per-element allocation count in storage: **0**. Names are interned once
(`SymbolTable`, `model.rs:15004`), qualified paths are arena-encoded (`SymbolPathArena`,
`model.rs:14920`). This is genuinely well done and should be stated as an invariant in `design.md`
so it is not regressed.

### 1.2 Resolver indexes (good)

`crates/sysml_resolution/src/model/resolver.rs:1779` — `ResolvedSemanticModel` holds twelve indexes,
all `Box<[..]>`:

- `NameIndex` (`resolver.rs:632`): `keys/ranges/candidates` — textbook CSR.
- `ReverseReferenceIndex` (`resolver.rs:644`), `EffectiveScopeIndex` (`resolver.rs:708`),
  `MembershipIndex` (`resolver.rs:780`) — CSR or dense-parallel.
- `SpanTree` (`resolver.rs:111`): `entries: Box<[(TextRange, DeclarationId)]>` + `subtree_end:
  Box<[u32]>` — a flattened interval tree, no `Box` nodes.
- `diagnostics_by_document: Box<[(u32,u32)]>` (`resolver.rs:1804`) — document-scoped diagnostics are
  a slice, not a scan.

### 1.3 Spans and ranges (mixed)

- Storage keeps parser `Span` (byte offsets) — compact.
- The public `TextRange`/`TextPosition` (`lib.rs`) is 4 × `u32` = 16 bytes. Fine as a value, but see
  §1.5: converting `Span` → `TextRange` is done lazily at query time and requires the parse tree.

### 1.4 The identity string — the one real layout defect

`resolver.rs:359` `IdentityIndex { text: Box<[Box<str>]>, … }`: **one heap `Box<str>` per
declaration**, and construction at `resolver.rs:398-415` does

```rust
text[owner.index()].to_string()      // full clone of the parent's identity
… push_identity_segment(…)           // then append this segment
```

Consequences:
- one `String` allocation + full parent copy per declaration → construction is
  **O(declarations × depth)** in both time and bytes;
- every identity begins with a length-prefixed **document identity** (`push_identity_field`,
  `resolver.rs:526-530`), i.e. the whole file URI is repeated inside every element's identity
  string. On the bundled standard library (tens of thousands of declarations, deep nesting) this is
  the single largest resident allocation in the model.

`IdentityIndex.text` should be the arena that `SymbolPathArena` already is: interned segments plus a
`(parent, segment, occurrence)` triple per declaration, with the string materialised only when a
consumer asks for one.

### 1.5 The parse tree is retained inside the published model

`CanonicalDocument` (`model.rs:2830`) holds `parsed: Arc<ParsedDocument>` plus
`parse_errors: Box<[ParseError]>`, and it survives into the published model — query-time code reads
it at `resolver.rs:4370`, `:4547`, `:4594` to turn a byte `Span` into a `TextRange`, and
`resolver.rs:4551`/`:4598` read the **source text** to locate an identifier inside a declaration
span.

This contradicts `design.md`'s "only the syntax service holds a parsed tree". More importantly it
means the AST *and* full source text of the entire standard library stay resident for the life of
every publication, and a navigation query does a text scan. The fix is to settle
`declaration_identifier_range` at the publication barrier (a dense `Box<[TextRange]>` parallel to
declarations, ~16 bytes/element) and drop the `Arc<ParsedDocument>` from the sealed model.

### 1.6 Public fact types — allocation per result element

This is where per-element allocation actually lives.

| Type | Site | Allocations per value |
|---|---|---|
| `SymbolIdentity(Box<str>)` | `lib.rs:263` | 1, minted fresh at `resolver.rs:1961-1965` (`text.into()`) |
| `SourceLocation` | `lib.rs:279` | 1 (`document: Box<str>` — the URI, cloned per result) |
| `NavigationTarget` | `lib.rs:286` | 3 (symbol + name + location.document) |
| `VisibleMember` | `lib.rs:333` | up to 5 (symbol, name, qualified_name, container_name, declaring_document) |
| `RenameOutcome::Ready` | `lib.rs:317` | 2 + one `SourceLocation` per occurrence |

`grep -c "Box<str>" crates/sysml_resolution/src/*.rs` → **65** field sites across the public
surface. Every one is a string that already exists interned in `SymbolTable` or in
`CanonicalDocument::identity`.

---

## 2. Query-path audit — worst 15

`resolved_slice.rs` itself is a thin delegating shim (each `pub fn` forwards to the authority), so
the cost is in the authority functions behind it. Ranked by (cost × call frequency).

| # | Facade signature (`resolved_slice.rs`) | Problem |
|---|---|---|
| 1 | `visible_members(...) -> QueryOutcome<Box<[VisibleMember]>>` :591 | `visible_member_records` (`resolver.rs:2329`) does, **per member**: `ids.to_vec()` + sort + dedup, `declaration_target` (which reads source text for the identifier range, `resolver.rs:4547`), `declaration_qualified_name` = `Vec<&str>` + `reverse` + `join("::")` (`resolver.rs:4674-4690`), plus 4 more `Box<str>`. ≈6 allocations + a text scan per member, on every completion keystroke. |
| 2 | `requirement_derived_fact(...)` :746 | `resolver.rs:3126` scans **all** declarations to find children of one declaration; `MembershipIndex`/`EffectiveScopeIndex` already answer owner→members in O(1). |
| 3 | `definition_usage_derived(...)` :728 | Same full scan, `resolver.rs:3066`. |
| 4 | `namespace_import_derived_elements(...)` :791 | Full scan for `owner == ns && kind == Import`, `resolver.rs:3542`. |
| 5 | `requirement_verifications(...)` :811 | Full scan filtering on `document.role == Workspace`, `resolver.rs:3800`. A `role`-partitioned id range would make this a slice. |
| 6 | `target_at` / `element_details_at` / `inspect_at` :554,:666,:645 | Each remints `SymbolIdentity` + `SourceLocation` and recomputes the identifier range from source text. |
| 7 | `references(...) -> QueryOutcome<Box<[SourceLocation]>>` :562 | One `Box<str>` document URI per occurrence; find-all-references on a library symbol allocates thousands of copies of the same handful of URIs. |
| 8 | `search_elements(ElementSearch) -> Box<[SymbolEntry]>` :680 | Full scan by construction; no kind- or source-partitioned index, and each hit mints 3–4 strings. |
| 9 | `document_symbols(&str) -> Box<[SymbolEntry]>` :675 | Document lookup is fine, but every entry re-mints identity/name/URI strings that could be ids. |
| 10 | `effective_features(...) -> Box<[SymbolEntry]>` :816 | Supertype walk materialising owned entries; no `impl Iterator` form. |
| 11 | `all_supertypes(...)` :466 | Returns an owned collection where a borrowed CSR slice of `TypeIndex` would do. |
| 12 | `satisfy_relationships()` / `binding_connectors()` :685,:799 | Whole-model owned collections with no document scoping — a host that wants one file's relationships pays for the library. |
| 13 | `source_digest()/model_digest() -> String` :540,:544 | Returns `String` where `&str` or a `[u8;32]` id would do. |
| 14 | `identity_declarations` (behind most `*_at` queries) `resolver.rs:1970` | Returns `Vec<DeclarationId>`; allocates a `Vec` for the overwhelmingly common single-hit case (`resolver.rs:501-524`). A `SmallVec`/`(head, next)` cursor removes it. |
| 15 | `affected_documents(...)` :317 | Owned document list per relink; a borrowed slice of the settled index would do. |

Pattern across all 15: **owned collections of owned strings, where the answer already exists as ids
inside a resident index.** None of these need the model rebuilt — they need a return type that can
carry ids and borrow.

---

## 3. Construction-path audit

- **Identity clone per declaration** — `resolver.rs:411` `text[owner.index()].to_string()`. The
  single largest construction allocation. (§1.4)
- **`clone()` density** — `model.rs` has 478 `.clone()` sites. Most are small parser values, but the
  lowering walk is where a `Span`/`Box<str>` clone per node multiplies. Worth a targeted pass, not a
  blanket rewrite.
- **BTreeMap ordinal counters rebuilt per build** — `SemanticModelBuilder`'s four
  `next_*_ordinals: BTreeMap<…>` (`model.rs:3040-3060`) plus `name_occurrences`'s
  `BTreeMap<(DocumentId, Option<DeclarationId>, DeclarationKind, SymbolId), u32>`
  (`resolver.rs:544`). A `BTreeMap` keyed on a 4-tuple, one insert per declaration, is a pointer-
  chasing hot loop; because declarations are lowered in owner order these can be a sorted-run
  counter or a `HashTable` with a precomputed hash.
- **`declarations_by_document` → `Vec<Vec<DeclarationId>>`** (`resolver.rs:2547`) — a `Vec` per
  document at the diagnostic barrier; should be CSR like every neighbouring index.
- **Parse tree retained** — see §1.5. Also `SettledLibrary` correctly hands trees over by
  `Arc::clone` (`resolver.rs:4776`), so library reuse does not copy.
- **No workspace-side incremental reuse.** `publication/mod.rs:121 publish` re-lowers and re-resolves
  every workspace document on each edit; only the library stratum is keyed and reused
  (`publication/mod.rs:181 library_stratum`, keyed by `library_key`, `:214`). For a workspace with
  many files, one keystroke costs the whole workspace. This is the largest single latency item and
  the hardest; it is listed last in §6 for that reason.

---

## 4. Benchmarks

**Exists:**
- `tools/semantic_benchmark` (355 lines) — fresh publications over checked-in snapshot SOURCE
  sections, `--libraries` to admit the bundled standard library, and `--reuse-stratum` to model an
  editor session. This is a real cold-build baseline and already covers the library corpus.
- `tools/resolution_benchmark` (1085 lines).
- No `criterion` or `divan` anywhere in the workspace; **no query-level benchmark at all**, and no
  checked-in baseline numbers.

**Proposed minimal set** (a `benches/` in `sysml_query`, divan preferred — lower ceremony, and the
facade is the only crate a bench may name):

1. `cold_build_stdlib` — `Services::in_memory()`, admit the bundled library, publish. Reports wall
   time **and peak resident bytes of the sealed model** (the memory number is what §1.4/§1.5 move).
2. `warm_relink_one_document` — settled library stratum + N workspace docs; edit one; republish.
   The regression guard for §3's incremental work.
3. `q_visible_members` — completion at a deep nesting point in a library-heavy scope.
4. `q_target_at` / `q_references` — go-to-definition and find-all-references on a widely used
   library symbol.
5. `q_document_symbols` — outline for one large document.
6. `q_diagnostics_for_document` — should already be a slice; pins that it stays one.

Plus an **allocation counter** (a counting global allocator behind a bench-only feature) so
"allocations per result element" is asserted, not estimated — that is the number every change below
is really about.

---

## 5. Target representation and the facade that hides it

### 5.1 Internal target (mostly already reached)

| Concern | Today | Target |
|---|---|---|
| Elements | `Box<[Declaration]>` + parallel facts, `u32` ids | keep as is — this is the target |
| Names | interned `SymbolTable` + `SymbolPathArena` | keep |
| Relationships | CSR `ranges` + payload | keep |
| Spans | parser byte `Span` | keep; add a dense `Box<[TextRange]>` of settled identifier ranges so queries stop reading source |
| Identity | `Box<[Box<str>]>`, parent-cloned | `Box<[IdentityNode { parent: DeclarationId, segment: SymbolId, occurrence: u32 }]>` — materialise the string only at the boundary |
| Parse trees | `Arc<ParsedDocument>` inside the sealed model | dropped at the barrier; owned solely by the syntax service, per `design.md` |

### 5.2 The facade must stop exposing

The blocker is not the internals — it is that `resolved_slice.rs:5-46` re-exports **~150 authority
types verbatim**, so every field of every fact type is public API.

Types that **pin the representation** and must become opaque:

- **`SymbolIdentity`** (`lib.rs:263`) — a public `Box<str>` newtype with `as_str()`. This is the
  single worst pin: it forces a string allocation per result element and forbids ever handing a
  consumer a `u32` handle. Target: opaque, `Copy`-able where possible, `Hash + Eq`, with
  `fn display(&self, model: &PublishedModel) -> impl Display` for the rare consumer that needs text.
  Stability across rebuilds (the property `resolver.rs:1957-1960` documents) is preserved by making
  the *encoding* stable, not the *representation* public.
- **`VisibleMember`, `SymbolEntry`, `NavigationTarget`, `SourceLocation`, `ElementDetails`,
  `ElementInspection`, `DiagramElement`/`DiagramEdge`** — all-public `Box<str>` fields. Target:
  accessor methods returning `&str` borrowed from the model, so the model can back them with
  interned ids.
- **`SourceLocation::document: Box<str>`** — should be an opaque `DocumentId` handle plus
  `fn uri(&self, model) -> &str`.
- **Every `-> Box<[T]>` return** — should become `-> impl Iterator<Item = T> + '_` or a borrowed
  slice view, so a caller that wants the first ten completions does not pay for ten thousand.
- **`QueryOutcome<T>`** stays — it is a status wrapper and carries no layout.
- Pure enums (`ElementKind`, `MembershipKind`, `RelationshipProvenance`, all the `*Kind`/`*Outcome`
  discriminants) are fine to re-export; they pin nothing.

The rule to write into `design.md`: *a facade type may expose an id, an enum, or a borrowed
accessor; it may not expose an owned `String`/`Box<str>`/`Vec` field.* An architecture test in
`crates/sysml_query/tests/architecture.rs` (which already runs a public-API visitor) can enforce it
mechanically.

---

## 6. Prioritised changes (win × ease)

Each line is phrased so it can be pasted into `design.md` as the "why" of an invariant.

1. **Settle identifier `TextRange`s at the publication barrier and drop `Arc<ParsedDocument>` from
   the sealed model.** *A sealed publication answers a location query from a settled fact, never by
   re-reading source text, so the parse tree is owned only by the syntax service.* (High win: frees
   the whole library AST + text; removes a text scan from every navigation query. Medium ease.)
2. **Make `SymbolIdentity` opaque and id-backed.** *An element handle is an identity, not a string;
   materialising its text is a boundary operation a consumer asks for explicitly.* (Highest win —
   removes one allocation per result element everywhere; easy mechanically, wide blast radius.)
3. **Arena the `IdentityIndex`.** *An element's identity is derived from its owner's, so it is
   stored as a link to the owner rather than a copy of it.* (High win on both build time and
   resident bytes. Easy once #2 lands.)
4. **Replace the five full-storage scans with the resident owner→member index.** *A query over one
   element's members costs its members, never the corpus.* (`resolver.rs:3066, 3126, 3542, 3800,
   4739`. High win, easy — the index already exists.)
5. **Borrowed accessors instead of `Box<str>` fields on `VisibleMember`/`SymbolEntry`/
   `NavigationTarget`/`SourceLocation`.** *A fact returned from an immutable publication borrows its
   names from the publication; it does not copy them.* (Very high win on completion; medium ease —
   touches every host.)
6. **Add the divan bench set and an allocations-per-element assertion.** *A representation change is
   admitted only with a benchmark showing it neutral-or-better on the bundled standard-library
   corpus.* (Enabling work for everything else; easy.)
7. **`-> impl Iterator` for the collection-returning queries.** *The facade returns a cursor over
   settled facts; the consumer decides how many it materialises.* (Medium win, medium ease; do it
   with #5 so hosts are touched once.)
8. **Per-document incremental reuse for workspace documents, mirroring the library stratum.**
   *Reuse is keyed by content digest at every provenance, not only at the library boundary.* (Largest
   latency win, hardest and riskiest — needs cold/warm parity coverage per `AGENTS.md`. Do it last,
   on top of the benches from #6.)

Items 1–5 are pure representation work behind the facade and change no semantic result; each should
land as a green commit with the §4 benchmarks as evidence.
