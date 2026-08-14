# Investigation and design brief: the semantic resolution layer

Status: kickoff brief for a detailed investigation and design activity
Audience: whoever picks up the two open resolution defects
Related: `UNIFY_CACHE_PLAN.md`, `ROUNDTRIP_SEMGRAPH_PREREQS.md`, `UNIFY_CACHE_PROGRESS.md`

---

## How to use this document

This brief exists so the investigation does not repeat work already done. Sections 1–4 are
**established findings with citations** — treat them as given unless you find direct evidence to
the contrary, in which case say so explicitly and show the evidence. Section 5 is the actual
assignment. Section 6 records what has already been ruled out, so you do not re-explore it.

The deliverable is **a design, not a patch**. Do not begin implementing either fix. The output is
a written proposal with enough specificity that implementation can be split into independently
reviewable pieces afterwards.

---

## 1. Why this investigation exists

Two independent semantic defects in Spec42's resolution layer were surfaced while implementing the
unified semantic cache. Neither was caused by that work, and neither is a cache problem. Both were
found because caching forces questions that were previously avoidable: what exactly does a build
produce, and does every path produce the same thing?

They are being investigated together because they live in the same layer, are likely to interact,
and a fix for either that ignores the other is likely to be wrong.

The cache work continues in parallel and does not currently block on these. The reverse is not
true: the cache's central correctness claim depends on them.

### Why this matters to the cache specifically

`UNIFY_CACHE_PLAN.md` §2.2 requires that "cold, warm, parallel, incremental, and cache-disabled
paths are semantically equivalent," and `AGENTS.md` states that "at the same declared phase and
model identity, full, incremental, cached, and parallel paths must be observably equivalent."

That guarantee is only meaningful if the underlying resolution is itself well-defined and single-
sourced. Caching a graph produced by one of two divergent engines, under a resolution rule that
does not match the specification, would preserve and redistribute the divergence rather than
expose it. A cache is a disposable accelerator; it must never be the thing that makes an
inconsistency permanent.

---

## 2. Defect A: two independently implemented resolution engines

### What is established

Whole-graph linking and the scoped/incremental resolver are two separate implementations of
cross-document reference resolution. They are not a shared core with different entry points.

Evidence: fixes have had to be applied to both separately. The in-code documentation at
`crates/sysml_model/src/semantic/relationships/cross_document.rs`, on
`resolve_typing_edge_cross_document_inner`, records exactly this — the `~P` port-conjugation fix
had to be duplicated across both paths.

This was confirmed during the B1 work (typed edge construction ownership, now merged). B1 required
only that the two engines agree on *ownership*, which it achieved structurally rather than by
making the engines agree: ownership is assigned at the single edge-insertion choke point
(`add_semantic_edge_once` in `crates/sysml_model/src/semantic/relationships.rs`) by comparing
source and target URIs, so every construction path converges on identical ownership regardless of
which engine produced the edge.

B1 deliberately did **not** attempt to unify the algorithms. That is this investigation's problem.

### What is not established

- Whether the two engines currently produce different *results* (as opposed to merely being
  different code). B1 proved ownership parity, not resolution parity.
- The full inventory of behavioural differences between them.
- Whether the divergence is incidental drift or reflects a genuine difference in what each path
  can know at the time it runs.

### Entry points

- Whole-graph: `link_workspace_relationships_pass` and `link_workspace_relationships`,
  `crates/sysml_model/src/semantic/relationships.rs:225`.
- Scoped/incremental: `add_cross_document_edges_for_uri`, `resolve_cross_document_edges_for_uri`,
  `crates/sysml_model/src/semantic/relationships/cross_document.rs`.
- Parallel: `link_parsed_documents_parallel`, `link_parsed_documents_parallel_from`,
  `crates/sysml_model/src/semantic/pipeline.rs`.
- Shared insertion point: `add_semantic_edge_once`, `crates/sysml_model/src/semantic/relationships.rs`.

---

## 3. Defect B: type-reference resolution ignores KerML scoping

### What is established

Spec42 resolves type references **workspace-wide**. The specification requires resolution scoped
by namespace containment, with visibility and import filtering, falling back to the global index
only at the namespace root. This affects **every typing kind**, not one construct.

This was settled against the OMG pilot implementation at
`/Users/luke/Documents/GitHub/SysML-v2-Pilot-Implementation`. The decisive citations:

- `KerMLScopeProvider.xtend:96-97` routes a `FeatureTyping` context to
  `scope_nonExpressionNamespace`.
- That descends `scope_Namespace → scopeFor` (`KerMLScopeProvider.xtend:137-173`), walking up
  `NamespaceUtil.getParentNamespaceOf`, and reaches the global/workspace index only at the
  namespace root (`:166-167`).
- `SysMLScopeProvider.xtend:55-78` inherits this behaviour unchanged for attributes — there is no
  special case.

Spec42's `link_workspace_relationships_pass`
(`crates/sysml_model/src/semantic/relationships.rs:225`) instead walks every node and calls
`add_typing_edge_for_node` using workspace-wide resolution for each entry in
`declared_facts.relationships.typing`.

### The related question that is also settled

Attribute typing is an ordinary `FeatureTyping`, semantically identical to typing on `part`,
`port` or `item`, differing only in that the type is a `DataType`:

- `AttributeUsage`, `PartUsage`, `PortUsage` and `ItemUsage` all route through one shared grammar
  chain (`Usage → UsageDeclaration → FeatureDeclaration → FeatureSpecializationPart →
  FeatureSpecialization → Typings → TypedBy → FeatureTyping`) terminating at
  `org.omg.sysml.xtext/.../SysML.xtext:437-444`. There is no attribute-specific typing rule.
- `AttributeUsage.java:32` extends `Usage`; `Feature.java:549` (`getOwnedTyping`) and `:418`
  (`getType`) are inherited uniformly. `FeatureTyping.java:29` extends `Specialization`.
- The `DataType` restriction is enforced **after** resolution by a separate validator:
  `SysMLValidator.xtend:559-562`, `checkAttributeUsage`, "An attribute must be typed by attribute
  definitions."
- The pilot's own expectation test `AttributeUsage_invalid.sysml.xt:37-61` shows
  `attribute a : A;` where `A` is a `part def` **resolving successfully** and only then failing
  validation.

In Spec42, attribute typing is currently written only as an untyped presentation string
(`attributeType`/`dataType`/`type`), not into `relationships.typing`. That is an implementation
artifact and is wrong, but correcting it is blocked on the scoping fix, because routing attribute
typing through `relationships.typing` sends it through the over-broad workspace-wide pass.

### One unresolved loose end

An earlier attempt to record attribute typing into `relationships.typing` reportedly flipped a
semantic-graph golden fixture — an edge expected to remain unresolved became resolved. A later
agent could **not reproduce** this: with a read-only `symbols.rs` change in place,
`cargo test -p workspace --test sysml_compatibility_corpus` passed unchanged. The original attempt
was never committed, so the interaction could not be reconstructed.

Establishing whether that fixture flip is real is part of the assignment. Note that the scoping
defect stands on the pilot citations regardless of how this resolves — the fixture is evidence
about *impact*, not about whether the defect exists.

### Consumers currently depending on the artifact

`crates/lsp_server/src/lsp_runtime/symbols.rs:150` falls back to `attributeType`/`dataType`/`type`
for symbol kind. Retiring that fallback is blocked on this work. It is one of the last items
holding open the `SemanticNode.attributes` removal (`ROUNDTRIP_SEMGRAPH_PREREQS.md` B9).

---

## 4. Adjacent context you should have

### Recently landed, build on it — do not rework

- **B1** — every `SemanticEdge` carries a typed `ConstructionOwner` (document construction,
  workspace cross-document linking, derivation linking, pending resolution, universal implied-rule
  construction), kept separate from `RelationshipProvenance`. Cross-document ownership is rebuilt
  from edge owner plus source identity rather than persisted.
- **B3** — `NodeId`'s `Ord` is now the single ordering-policy owner (normalized URI, then qualified
  name). Every qualified-name and alias insertion routes through
  `graph.rs::insert_canonical`, so lookup vectors no longer depend on merge order. The complete
  `Workspace`/`StandardLibrary`/`Library`/`External` classification is recorded as
  `SemanticGraphData::source_origins`.
- **B4** — `SemanticPublication { root_digest, phase, completeness, semantic_contract }` in
  `crates/sysml_model/src/semantic/publication.rs`. Phase ordering is structural (`Ord`-derived,
  `advance_phase` takes the max), and structural mutation retreats the phase so a stale settled
  claim cannot survive.

### A known residual gap, likely in your path

`resolve_subsetting_family_target` (`crates/sysml_model/src/semantic/relationships.rs`) selects
with `.find()` over a qualified-name bucket. After B3 the bucket is canonically ordered, so the
choice is deterministic — but it still **cannot report a genuine multi-candidate collision as
ambiguous**, because `Subsetting`/`Redefinition`/`ReferenceSubsetting`/`CrossSubsetting` edges have
no ambiguous-target representation in the edge model. B3 documented this in place and left it:
fixing it requires an edge-model change.

If your design introduces a scope-aware resolution result, decide whether it subsumes this.

### Normative source

`/Users/luke/Documents/GitHub/SysML-v2-Pilot-Implementation` — `sysml.library/` (normative model
library), `org.omg.sysml/` (metamodel and derived-property implementations), the Xtext grammars,
and `org.omg.sysml.xpect.tests/` + `org.omg.kerml.xpect.tests/`, which frequently encode exactly
the edge cases in dispute.

Distinguish throughout between what the specification states normatively and what the pilot merely
happens to do. Cite file and line.

### Repository rules that constrain the design

From `AGENTS.md`, all directly applicable:

- Every semantic fact category has one authoritative owner. Indexes, projections, DTOs and caches
  must not become competing truth stores.
- Give every derived fact one canonical derivation owner at the earliest semantic layer holding all
  prerequisites; consumers use the canonical result rather than reimplementing it.
- Keep unresolved, ambiguous, unsupported, partial, cancelled and failed states explicit and typed.
- Semantic results must not depend on traversal order, unordered-map iteration, cache warmth or
  insertion order.
- Diagnostics are public behavior: stable codes, precise locations, severity and ordering are
  contracts. An intentional change updates them with stated rationale and consumer coverage; never
  weaken assertions to hide a regression.
- Fix the owning abstraction, not the visible symptom — but do not expand a change to clean up
  unrelated legacy debt.

### Test baseline

`cargo test -p workspace --test snapshot_single_build` has 22 pre-existing failures on `main`,
unrelated to this work. That is the accepted baseline. Everything else is green and must stay
green. Do not "fix" those 22 by weakening assertions.

Semantic-graph golden fixtures live under
`crates/workspace/tests/fixtures/sysml_compatibility/**/*.md` as Markdown with an s-expression
`# SMG` section. **A fixture may be changed only as a deliberate, cited correction, never to keep a
build green.** If a fixture change looks like a regression rather than a correction, stop and
report.

---

## 5. The assignment

Produce a written design proposal covering both defects. Specifically:

### 5.1 Establish the actual behaviour

1. Inventory every behavioural difference between the whole-graph and scoped/incremental
   resolution engines. Build a differential harness that resolves the same source set through both
   and compares results, rather than reasoning from the code alone. Report whether they currently
   diverge in observable output, and where.
2. Determine the true scope rule Spec42 should implement for each reference kind — not just
   `FeatureTyping`. Cite the pilot. Cover at minimum typing, specialization, the subsetting family,
   subject, and imports, noting where the rules genuinely differ between kinds.
3. Quantify the blast radius of correcting the scoping: how many corpus fixtures change, and for
   each, whether the change is a correction or a regression. This is the single most important
   input to sequencing, because it determines whether the fix can land at once or must be staged.
4. Settle the attribute-typing fixture question — reproduce the flip or establish that it does not
   occur.

### 5.2 Design the target state

5. Propose the canonical resolution architecture: one owner for scope computation, consumed by
   every path. State explicitly how whole, parallel, incremental and cache-decoded builds are made
   to use it without a second implementation, and what structurally prevents a third engine from
   appearing later.
6. Define the typed resolution result, covering resolved, unresolved and ambiguous outcomes, with
   all candidates retained in canonical order for the ambiguous case. Say whether it subsumes the
   `resolve_subsetting_family_target` gap.
7. Say how scope-aware resolution interacts with incremental invalidation. A reference's resolution
   now depends on its enclosing namespace, visibility and imports, so the set of edits that can
   invalidate it is larger than a name match. This is the part most likely to be got wrong, and it
   bears directly on cache key completeness — if resolution depends on inputs the cache key does
   not commit, the cache is unsound.
8. Identify what this implies for `SemanticPublication` and the graph record: does a scope-resolved
   graph need identity that a workspace-wide-resolved one does not?

### 5.3 Sequence the work

9. Propose an implementation sequence broken into independently reviewable pieces, with the
   dependency order stated, and identify which pieces can proceed in parallel.
10. State the risks, and for each, what evidence would falsify your design. Call out anything you
    could not settle and what it would take to settle it.

### Deliverable

A design document in the repository root, following the conventions of `UNIFY_CACHE_PLAN.md` and
`ROUNDTRIP_SEMGRAPH_PREREQS.md`: normative statements, explicit scope and non-goals, a verification
plan, and an implementation sequence. Include the differential-harness results as evidence.

**Do not implement either fix as part of this activity.** Exploratory prototyping to answer a
question is fine and encouraged, but it is evidence for the design, not a deliverable — say clearly
what was prototyped and what was thrown away.

---

## 6. Already ruled out — do not re-explore

- **Attribute typing is not a distinct relationship kind.** Refuted by the shared grammar chain and
  the uniform metamodel inheritance cited in §3. Do not model it separately.
- **The golden fixture is not simply stale.** It was exposing the scoping gap. Do not update it to
  accommodate attribute typing.
- **Serializing the cross-document ownership map as a second authority.** Explicitly rejected by
  `ROUNDTRIP_SEMGRAPH_PREREQS.md` B1, and already solved by rebuilding from edge owner plus source
  identity.
- **Making lookup-vector order the fix.** B3 already made ordering canonical and single-owned.
  Ordering is not the remaining problem; scope is.
- **`nodes_by_uri` insertion order.** Deliberately left as insertion order. A URI's own vector
  reflects that document's deterministic AST traversal order, so it carries no cross-document
  ordering dependency, and hover's deepest-node tie-break depends on it. Canonicalizing it broke
  `hover_resolves_requirement_subject_in_context` and was reverted. Leave it alone.
