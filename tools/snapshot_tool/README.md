# Spec42 standalone snapshots

`spec42-snapshot` is the source-to-golden harness for semantic-model snapshots. It is a separate
command-line runner, not a Rust integration test and not an `insta` assertion layer.

Each Markdown file is a test case. The runner reads its `# SOURCE` section, builds the immutable
`SemanticModel`, and rewrites the owned `SMG`, `DIAGNOSTICS`, and `FORMAT` sections. It never
exposes graph nodes, resolution indexes, or fact collections to the caller; the semantic owner
streams its canonical debug S-expression through the caller-provided writer. Diagnostics are
collected by `sysml_diagnostics` from category-owned projections of the same published model; the
runner never rebuilds a mutable graph for validation.

The canonical top-level section order is `META`, `SOURCE`, `DIAGNOSTICS`, `FORMAT`, `SMG`.
`SOURCE` is authored; generated sections are rewritten to this order with one final newline.
Only sections in this contract are retained during normalization. Unknown or future sections
should be added to the explicit ordering table before they become part of the corpus contract.

Each readable fixture is built with both sequential and parallel construction. The runner compares
the complete owned `SMG` and `DIAGNOSTICS` renderings, including the publication state embedded in
`SMG`, before checking or writing goldens. Sequential output is the canonical rendering only after
this parity check succeeds; there is no strategy override that can bypass it.

Fixtures are evaluated concurrently with Rayon’s bounded global worker pool. Results are sorted by
path, errors and stale paths are reported in that order, and update writes occur only after the
complete worker batch succeeds.

Run it from the repository root:

```sh
cargo run -p spec42-snapshot -- update
git diff -- test/snapshots
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
