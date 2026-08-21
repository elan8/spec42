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

The canonical top-level section order is `META`, `SOURCE`, `DIAGNOSTICS`, `SMG`, `TYPES`,
`NAVIGATION`, with optional editor-query sections and `GENERATED` last.
`SOURCE` is authored; generated sections are rewritten to this order with one final newline.
Every generated section uses a canonical `sexpr` fence.
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

```sh
cargo run -p spec42-snapshot -- update
git diff -- tests/snapshots
cargo run -p spec42-snapshot -- check
```

To inspect one fixture:

```sh
cargo run -p spec42-snapshot -- update --fixture resolution/imports.md
cargo run -p spec42-snapshot -- check --fixture resolution/imports.md
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

For a researched rule that the compiler does not enforce yet, add a non-empty reason to `META`:

```ini
skip_validation=KerML 8.3.3.3.8 is not implemented
```

Only a diagnostic-expectation mismatch is skipped. The runner prints `SKIPPED <path>: <reason>`;
malformed fixtures, semantic build errors, parity failures, and stale generated sections still
fail normally. A skip whose expectation starts passing is an error, requiring the obsolete
`skip_validation` entry to be removed.

Validation fixtures belong under `tests/snapshots/validation`, normally one normative rule per
file. Include both sides of the rule in `SOURCE`: at least one conforming example that must not be
diagnosed and one violating example that must be. Give every fixture traceable normative metadata:

```ini
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.3.8
```

For SysML rules, use the corresponding OMG SysML 2.0 Language specification identifier and URL.
Keep `description` as a concise statement of the rule; it is not a substitute for the normative
reference. When a rule is skipped, make `skip_validation` name the missing capability or tracked
parser gap rather than merely saying that the test fails.

The executable examples are:

- `validation/kerml_redefinition_end_mismatch.md`, an enforced KerML rule whose expected and actual
  diagnostics match;
- `validation/kerml_end_feature_direction.md`, an unsupported part of a KerML rule whose desired
  semantic diagnostic differs from the parser-recovery diagnostics and is therefore reported as
  `SKIPPED`.

Run either example with the normal fixture command. A successful enforced case prints nothing;
the unsupported example prints its stable `SKIPPED` line:

```sh
cargo run -p spec42-snapshot -- check --fixture validation/kerml_end_feature_direction.md
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
with `scripts/build-generator-plugins.sh` before running a generator fixture.
