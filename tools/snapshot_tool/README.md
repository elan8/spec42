# Spec42 standalone snapshots

`spec42-snapshot` is the source-to-golden harness for semantic-model snapshots. It is a separate
command-line runner, not a Rust integration test and not an `insta` assertion layer.

Each Markdown file is a test case. The runner reads its `# SOURCE` section, builds the immutable
`SemanticModel`, and rewrites the owned `SMG`, `DIAGNOSTICS`, and `FORMAT` sections. It never
exposes graph nodes, resolution indexes, or fact collections to the caller; the semantic owner
streams its canonical debug S-expression into an internal buffer owned by the harness.

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
