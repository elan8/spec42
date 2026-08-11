# SysML snapshot compatibility corpus

`test/snapshots` contains 479 Markdown
fixtures covering SysML and KerML source, parser recovery, semantic construction,
and formatter behavior. OMG JSON interchange fixtures are deliberately excluded
from this corpus.

Run the standalone snapshot runner with:

```sh
cargo run -p spec42-snapshot -- check
```

The runner accounts for every fixture and performs these checks:

- strict parsing with the pinned `sysml-v2-parser`;
- recovery-mode semantic graph construction and canonical S-expression rendering,
  asserting both complete for every UTF-8 source;
- formatter idempotence for every source document;
- exact formatter-golden comparison when the current formatter already agrees.

The fixture `SMG` section is the exact, canonical `(semantic-graph ...)` rendering
of the semantic graph constructed from its `SOURCE`. Every UTF-8 fixture is
byte-compared to this owned golden, including recovery-mode sources that
materialize semantic facts. A non-empty source that currently has no typed
semantic facts retains the pure empty graph rendering. Its `META` section must
instead declare `semantic_graph=skip` and a concrete non-empty
`semantic_graph_skip_reason`; this state is counted and printed by the runner,
and becomes a failure when graph facts materialize. Refresh these sections
deliberately with:

```sh
cargo run -p spec42-snapshot -- update
```

Use `--fixture=<relative path>` to inspect one complete rendering before
accepting a refresh. `TOKENS`, `AST`, `EXPECTED`, and `PROBLEMS` are retained
as evidence but are not owned by this runner. A non-UTF-8 fuzz fixture is
reported as skipped because the pinned parser API accepts UTF-8 text. These
skips are future compatibility work, not passing conformance claims.

The runner's path and count guard prevent accidental loss of fixtures. When
deliberately refreshing the corpus, record the new count here or in the commit
message.
