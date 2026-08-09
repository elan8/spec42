# SysML snapshot compatibility corpus

`crates/workspace/tests/fixtures/sysml_compatibility` contains 479 Markdown
fixtures covering SysML and KerML source, parser recovery, semantic construction,
and formatter behavior. OMG JSON interchange fixtures are deliberately excluded
from this corpus.

Run the portable compatibility runner with:

```sh
cargo test -p workspace --test sysml_compatibility_corpus -- --nocapture
```

The runner accounts for every imported fixture and performs the checks that are
meaningful across implementations:

- strict parsing with the pinned `sysml-v2-parser`;
- recovery-mode semantic graph construction and canonical S-expression rendering,
  asserting both complete for every UTF-8 source;
- formatter idempotence for every source document;
- exact formatter-golden comparison when the current formatter already agrees.

The fixture `SMG` section is the exact, canonical `(semantic-graph ...)` rendering
of the semantic graph constructed from its `SOURCE`. Every UTF-8 fixture is
byte-compared to this owned golden, including recovery-mode sources that
materialize semantic facts. A non-empty source that currently has no typed
semantic facts uses an inline `(status (skip ...))` graph with a stable strict
or recovery code and an explicit zero-facts state; that state is also counted
and printed by the runner. Refresh these sections deliberately with:

```sh
cargo test -p workspace --no-default-features --test sysml_compatibility_corpus \
  regenerate_semantic_graph_sections -- --ignored
```

Use `SPEC42_SEMANTIC_GRAPH_FIXTURE=<relative fixture path>` with the ignored
`print_semantic_graph_fixture` test to inspect one complete rendering before
accepting a refresh. `TOKENS`, `AST`, `EXPECTED`, and `PROBLEMS` are retained
as evidence but are not direct assertions because their internal
representations and diagnostic wording are implementation-specific. A
non-matching parser acceptance result or formatter golden produces an explicit
`SKIP` message (with its reason) under `--nocapture`; it is never silently
dropped. A non-UTF-8 fuzz fixture is likewise reported as skipped because the
pinned parser API accepts UTF-8 text. These skips are future compatibility work,
not passing conformance claims.

The runner's count guard prevents accidental loss of fixtures. When deliberately
refreshing the corpus, update `IN_SCOPE_SNAPSHOT_COUNT` in the test and record
the new count here or in the commit message.
