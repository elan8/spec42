# Spec42 standalone snapshots

`spec42-snapshot` is the source-to-golden harness for semantic-model snapshots. It is a separate
command-line runner, not a Rust integration test and not an `insta` assertion layer.

Each Markdown file is a test case. The runner reads its `# SOURCE` section, builds the opaque
parser-owned resolved publication through the immutable `sysml_query` facade, and rewrites the owned `SMG`, `DIAGNOSTICS`, and
`NAVIGATION` sections. Its manifest has no direct semantic-model, diagnostics, or formatter
implementation dependency, and its transitive dependency graph cannot reach the mutable model or
legacy diagnostic crates. The facade streams canonical semantic, diagnostic, and navigation
S-expressions through caller-provided writers; the runner cannot obtain graph nodes, resolution indexes, or fact
collections and never rebuilds a mutable graph for validation.

Unsupported semantic families and parser recovery do not fall back to another engine. They publish
an explicitly incomplete model, stable typed diagnostics, and every supported fact recovered from
the same parser-owned document. Their visible snapshots are the migration inventory for completing
canonicalization; they must not be hidden by skips or converted into successful resolved facts.

Each section has one responsibility. `SMG` records semantic identity, kind, ownership, typed facts,
provenance, settled outcomes, candidates, and relationships. It does not repeat routine source
ranges for every element or reference. Exact locations belong in `DIAGNOSTICS` when reporting is
location-sensitive and in `NAVIGATION` when source-to-target mapping is under test. A semantic
source span is rendered in `SMG` only when the span itself is a named semantic fact that cannot be
observed through those sections. This keeps formatting-only movement from obscuring semantic diffs.

The canonical top-level section order is `META`, `SOURCE`, authored `EXPECTED DIAGNOSTICS` and
`EXPECTED SEMANTICS`, `DIAGNOSTICS`, `SMG`, `TYPES`, `NAVIGATION`, with optional editor-query
sections, `HOVER MARKDOWN`, and `GENERATED` last.
`SOURCE` is authored; generated sections are rewritten to this order with one final newline.
Semantic and reporting-result sections use canonical `sexpr` fences. `HOVER MARKDOWN` is the
dedicated renderer projection and contains one labelled `markdown` fence per requested probe.
Only sections in this contract are retained during normalization. Unknown or future sections
should be added to the explicit ordering table before they become part of the corpus contract.

Each readable fixture is built with both sequential and parallel construction. The runner compares
the complete owned `SMG`, `DIAGNOSTICS`, and `NAVIGATION` renderings, including the publication
state embedded in `SMG`, before checking or writing goldens. Sequential output is the canonical rendering only after
this parity check succeeds; there is no strategy override that can bypass it.

Fixtures are evaluated concurrently with Rayon’s bounded global worker pool. Results are sorted by
path, errors and stale paths are reported in that order, and update writes occur only after the
complete worker batch succeeds.

Run it from the repository root:

`cargo snapshot` is an alias in `.cargo/config.toml` for `cargo run --release -p spec42-snapshot --`;
the tool builds every fixture's publication, and the debug profile is 10-30x slower on that.

```sh
cargo snapshot update
git diff -- tests/snapshots
cargo snapshot check
cargo snapshot report
cargo snapshot report --format json
```

To inspect one fixture:

```sh
cargo snapshot update --fixture resolution/imports.md
cargo snapshot check --fixture resolution/imports.md
```

`check` never writes files. A stale fixture is a failure and the Markdown diff is the review
surface. `update` is deliberately explicit so generated sections cannot change as a side effect of
normal Rust test execution.

The runner accepts both one fenced SOURCE document and named multi-document SOURCE sections:

```markdown
# SOURCE
## library.sysml
~~~sysml
package Library {}
~~~
## model.sysml
~~~sysml
package Model {}
~~~
```

The parser/updater unit tests cover only Markdown mechanics. Semantic behavior belongs in the
checked-in source snapshots and their canonical S-expression sections.

## Editor and hover probes

`EDITOR QUERIES` uses zero-based line and UTF-16 character positions. A normal probe exercises the
canonical editor queries. Add the `hover` option to also append the presentation-independent
`hover-report` for that occurrence in `HOVER RESULTS` and generate its exact renderer output in
`HOVER MARKDOWN`:

```text
probe model.sysml 4 9 hover
```

The report and Markdown are produced from the same typed `HoverReport`; snapshots therefore expose
both semantic/presentation structure and the text an editor receives without maintaining parallel
hover implementations. A probe with no hover records `(status none)` and an empty Markdown fence.

## Validation expectations

A fixture may add an authored `EXPECTED DIAGNOSTICS` section. Its fenced S-expression must match
the complete canonical `DIAGNOSTICS` projection exactly, including codes, severities, source
ranges, ordering, and related information. `update` never rewrites this expectation, so adding or
changing compiler diagnostics cannot bless a validation result accidentally:

```markdown
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics ...)
~~~
```

Repository-level admission fixtures may add typed publication ratchets in `META`. These are
authored assertions evaluated against the publication and diagnostic contracts, not against the
rendered snapshot text; `update` cannot bless a regression:

```ini
require_complete_publication=true
require_no_diagnostics=true
```

New normative fixtures declare their source contract, rule family, authored expectation, and stable
rule ID in `META`. `rule_id` may repeat when one fixture intentionally supplies evidence for more
than one rule:

```ini
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.3.3.8:validateRedefinitionEndConformance
```

One fixture is the canonical evidence for each rule.  A deliberately complementary regression
case for that same rule declares `coverage_role=secondary`; `primary` is the default.  The report
rejects duplicate primary evidence and secondary evidence without a primary fixture.

The open-issue registry at `tests/snapshots/issues.toml` owns stable blocker IDs, their closed
category, owner, summary, and optional tracking link. A blocked expectation refers only to that
ID; the runner rejects unknown references, duplicate IDs, and registry entries not used by any
fixture:

```ini
blocked_by=diagnostic-gap-17
```

An `expectation=semantics` fixture supplies an authored, non-rewritten `EXPECTED SEMANTICS`
section. Each relationship names the canonical relationship family, source and target qualified
names, provenance, and explicit typed outcome. The runner resolves those names through
`sysml_query`, compares the resulting opaque identities and relationship fact, and checks the
same queries across canonical, sequential, parallel, and warm-library publications. It never
searches `SMG` or `TYPES` text:

```markdown
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship
    (kind specialization)
    (source "Model::Component")
    (target "Parts::Part")
    (provenance implied)
    (outcome resolved)))
~~~
```

The five exact pinned-XMI `Feature` derived-relationship collections have a separate compact
assertion. `rule_id` is one of the five full canonical KerML IDs below; it selects the closed
`sysml_query::FeatureDerivedRelationshipCollection` API directly, rather than deriving a query
from a name. `source`, `kind`, `target`, `provenance`, and `outcome` have the same typed meaning
as a normal relationship assertion. A collection relationship with an unresolved target omits
`target` but still names its provenance:

```markdown
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (derived-relationship-collection
    (rule_id "kerml-1.0:8.3.3.3.4:deriveFeatureOwnedTyping")
    (source "Model::Vehicle::mass")
    (kind feature_typing)
    (target "Model::Mass")
    (provenance authored)
    (outcome resolved)))
~~~
```

The accepted IDs are `deriveFeatureOwnedFeatureChaining`,
`deriveFeatureOwnedRedefinition`, `deriveFeatureOwnedSubsetting`,
`deriveFeatureOwnedTyping`, and `deriveFeatureOwnedTypeFeaturing`, all under
`kerml-1.0:8.3.3.3.4`. Their asserted relationship kinds include `feature_chaining` and
`type_featuring` in addition to `feature_typing`, `subsetting`, and `redefinition`.

The exact `Type` collection and operand contracts use the parallel
`type-derived-relationship-collection` assertion. Its `rule_id` is one of
`deriveTypeOwnedSpecialization`, `deriveTypeOwnedUnioning`,
`deriveTypeOwnedIntersecting`, `deriveTypeOwnedDifferencing`,
`deriveTypeOwnedDisjoining`, `deriveTypeUnioningType`, `deriveTypeIntersectingType`, or
`deriveTypeDifferencingType` under `kerml-1.0:8.3.3.1.10`. These are queried only through
`sysml_query::TypeDerivedRelationshipCollection`; operand derivations retain the same canonical
relationship fact and therefore its target outcome and provenance.

`deriveTypeOwnedFeature` and `deriveTypeOwnedEndFeature` are final element-valued projections and
use the closed
`type-derived-element` assertion:

```sexpr
(type-derived-element
  (rule_id "kerml-1.0:8.3.3.1.10:deriveTypeOwnedFeature")
  (source "Model::Container")
  (target "Model::Container::owned")
  (outcome resolved))
```

It is evaluated only through `PublishedModel::type_derived_elements`. The resolver selects direct
canonical Feature-membership member identities; the end-feature projection additionally reads the
canonical `end` modifier fact. It does not materialize a public `FeatureMembership` relationship,
infer inherited members, or recover a target from text.
`absent`, `incomplete`, and `unsupported` omit `target`.

The remaining exact `Type` derivations use the closed `type-derived-fact` assertion. Its
`rule_id` selects the manifest-owned fact shape; fixtures still author the desired normative
result even while the canonical query returns a typed unavailable prerequisite. Member and
conjugator results name their canonical endpoint, while scalar multiplicity intentionally has no
synthetic target identity:

```sexpr
(type-derived-fact
  (rule_id "kerml-1.0:8.3.3.1.10:deriveTypeMultiplicity")
  (source "Model::Sized")
  (outcome resolved))
```

The runner evaluates this only through `PublishedModel::type_derived_fact`. `resolved` requires
an endpoint for every collection except multiplicity; `absent`, `incomplete`, and `unsupported`
omit it. A blocker therefore becomes stale when the canonical owner publishes the authored
desired value—it never converts a typed unavailable outcome into an asserted success.

Exact Systems `Actions` derivations use the analogous closed `action-derived-fact` assertion.
The rule ID selects the manifest-owned result shape and evaluation goes only through
`PublishedModel::action_derived_fact`:

```sexpr
(action-derived-fact
  (rule_id "sysml-2.0:8.3.17.3:deriveActionDefinitionAction")
  (source "Actions::Procedure")
  (target "Actions::Procedure::step")
  (outcome resolved))
```

Arguments and parameters may not yet have public canonical identities, so a resolved desired
result may omit `target` to require a nonempty canonical result without inventing an identity.
`absent`, `incomplete`, and `unsupported` omit it. A typed unavailable prerequisite does not
pass a resolved assertion; its precise fixture blocker remains visible until the canonical owner
publishes the authored desired fact.

The exact Root `Element` owner derivation uses a scalar `element-owner` assertion:

```sexpr
(element-owner
  (rule_id "kerml-1.0:8.3.2.1.2:deriveElementOwner")
  (source "Model::Vehicle::mass")
  (owner "Model::Vehicle")
  (outcome resolved))
```

Only that full rule ID is accepted. `outcome=absent` asserts the settled null owner of a root
element and omits `owner`; `incomplete` also omits it. The runner evaluates this only through
`sysml_query::PublishedModel::derived_element_owner`, never from a qualified-name prefix or the
fixture syntax tree.

`deriveElementDocumentation` and `deriveElementTextualRepresentation` use the parallel
`element-documentation` assertion. Its rule ID selects the closed collection through the
manifest; a resolved assertion names its exact typed form, locale, language, and text:

```sexpr
(element-documentation
  (rule_id "kerml-1.0:8.3.2.1.2:deriveElementTextualRepresentation")
  (source "Model::Vehicle")
  (form textual_representation)
  (locale none)
  (language "Alf")
  (text " implementation ")
  (outcome resolved))
```

`none` means the canonical optional field is absent. `outcome=absent` and `incomplete` omit the
form, locale, language, and text fields. The runner asks only
`PublishedModel::element_derived_documentation`; comments and other annotation relationships are
not inferred into these two exact collections.

The exact Namespace element-valued derivations use `namespace-derived-element`:

```sexpr
(namespace-derived-element
  (rule_id "kerml-1.0:8.3.2.4.5:deriveNamespaceOwnedMember")
  (source "Model")
  (target "Model::Owned")
  (outcome resolved))
```

Only `deriveNamespaceOwnedMember` and `deriveNamespaceOwnedImport` select this query family.
They are evaluated through `PublishedModel::namespace_derived_elements`, over canonical
declaration ownership and membership facts. `absent`, `incomplete`, and `unsupported` omit
`target`; the runner never reconstructs Namespace membership from a scope label or source text.

`deriveNamespaceImportImportedElement` is a scalar on an anonymous import declaration, so its
owner-scoped assertion identifies the owning Namespace and preserves the canonical authored
reference target:

```sexpr
(namespace-import-derived-element
  (rule_id "kerml-1.0:8.3.2.4.6:deriveNamespaceImportImportedElement")
  (owner "Model")
  (target "Library")
  (provenance authored)
  (outcome resolved))
```

The runner obtains the import identity only from `PublishedModel::namespace_import_derived_elements`;
it never gives an anonymous import a made-up display name. `target` outcome and provenance use the
same closed relationship contract as other semantic assertions.

`outcome` is one of `resolved`, `unresolved`, `ambiguous`, `unsupported`, `absent`, or
`incomplete`; no outcome is inferred from diagnostic text. A typed blocker whose diagnostic or
semantic expectation starts passing is `stale` and fails, requiring `blocked_by` and eventually
the resolved registry entry to be removed. A `by_construction` expectation remains blocked until
an owned executable evidence reference is available. Its closed metadata syntax is
`evidence_reference=test:<repository-relative-path>` or
`evidence_reference=file:<repository-relative-path>`; an unblocked by-construction assertion
passes only when that file exists. A blocked by-construction assertion may omit evidence only when
its typed issue is `abstract_syntax_coverage_gap`.

Semantic and by-construction fixtures may additionally contain `EXPECTED DIAGNOSTICS` to pin an
actionable diagnostic caused by the asserted fact state (for example, a missing library anchor).
This is a supplemental assertion, not `expectation=diagnostics`: the primary family combination
remains closed. Both the primary assertion and the exact diagnostics must pass before a typed
blocker is stale; if either remains unmet, the fixture remains blocked.

`report` evaluates the same fixtures without writing snapshots. Text output is path-sorted; JSON
has schema version 1 and includes every fixture's expectation state (`passed`, `blocked`, `stale`,
`failed`, or `not_applicable`) plus deterministic aggregate counts. Diagnostic-category aggregates
come from the canonical typed diagnostic query contract; the runner deliberately does not parse
rendered diagnostic text. It also audits every fixture rule ID against
`specifications/constraint_manifest.toml`, reporting manifest and fixture occurrence counts,
unique fixture IDs, missing evidence, duplicate fixture evidence, unknown IDs, and family/clause
mismatches. Report exits unsuccessfully while any of that coverage debt remains; ordinary
`check` and `update` continue to operate on their selected fixtures.

For a focused fixture that supplies its own library documents, keep the role explicit and
independent of parser behavior. Use `libraries=none` with a comma-separated list of SOURCE
document names in repeatable `standard_library_document` entries; those named documents are admitted as
`StandardLibrary` and every other SOURCE document as `Workspace`. This cannot be combined with
the repository fixed `libraries=standard` stratum:

```ini
libraries=none
standard_library_document=parts.sysml
```

Validation fixtures belong under `tests/snapshots/validation`, normally one normative rule per
file. Include both sides of the rule in `SOURCE`: at least one conforming example that must not be
diagnosed and one violating example that must be. Give every fixture traceable normative metadata:

```ini
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.3.3.8:validateRedefinitionEndConformance
```

For SysML rules, use the corresponding OMG SysML 2.0 Language specification identifier and URL.
Keep `description` as a concise statement of the rule; it is not a substitute for the normative
reference. When a rule is not yet satisfied, use `blocked_by` with a typed registry issue naming
the missing capability or parser gap rather than merely saying that the test fails.

The executable examples are:

- `validation/kerml_redefinition_end_mismatch.md`, an enforced KerML rule whose expected and actual
  diagnostics match;
- `validation/kerml_end_feature_direction.md`, a blocked KerML rule whose desired semantic
  diagnostic differs from parser-recovery diagnostics.

Run either example with the normal fixture command. A successful enforced case prints nothing;
a blocked case remains visible through `report`:

```sh
cargo snapshot report --fixture validation/kerml_end_feature_direction.md
```

Readable qualified-reference queries can be exercised directly without embedding opaque identity
encodings. Use a named source document or `*` for publication-wide lookup, followed by the KerML
qualified name and an expected `ElementKind` (or `*`):

```markdown
# QUALIFIED REFERENCE QUERIES
~~~text
resolve model.sysml Example::selected ViewUsage
resolve * StandardViewDefinitions::GeneralView ViewDefinition
~~~
```

The runner owns the corresponding `QUALIFIED REFERENCE RESULTS` section and checks it for
sequential/parallel parity.

## Generator snapshots

A fixture with `type=generate` selects a repository-owned WebAssembly plugin in `META`. Plugin
selection is closed: fixtures cannot provide filesystem paths. Conformance fixtures use the
canonical `conformance:<name>` form (the legacy bare name remains accepted), while diagram
fixtures use `repository:diagram`:

```markdown
# META
~~~ini
type=generate
libraries=standard
plugin=conformance:requirements_csv
~~~
```

Diagram fixtures select one authored view with its source-document name, KerML qualified name and
typed view kind. The semantic query owner resolves that readable reference to the canonical opaque
identity; the runner then finds the identical catalog entry and passes its opaque handle to the
guest. It never guesses from a display label or embeds the identity encoding in fixture metadata:

```ini
plugin=repository:diagram
viewKind=general-view
viewDocument=model.sysml
viewQualifiedName=Example::selected
```

All three selection keys must occur together and are invalid for conformance plugins. A document
name is normalized by the same `SourceDocument` constructor used to admit fixture source; the
qualified name is resolved only within that document. Unresolved, wrong-kind and ambiguous
references are errors rather than fallback selections.

The runner executes that plugin against both the sequential and parallel immutable publications.
Outcome, diagnostics, artifact paths, and exact artifact bytes must agree before the canonical
result is written. Generated files are captured in memory rather than applied to the filesystem:

```markdown
# GENERATED
## requirements.csv
~~~csv
qualified_name,name,documentation
Example::SafeStop,SafeStop,The vehicle shall stop safely.

~~~
```

Artifacts must be safe relative paths and valid UTF-8. Paths are sorted canonically; a changed or
removed artifact makes `check` fail and `update` replaces the complete section. Build test plugins
with `scripts/build-generator-plugins.sh` before running a generator fixture. Repository diagram
snapshots omit the top-level `modelDigest`: the real product retains that dependency-complete
identity, while dedicated generator/runtime tests cover it without making every diagram snapshot
change whenever the publication digest contract changes.
