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
- recovery-mode semantic graph construction, asserting it never panics;
- formatter idempotence for every source document;
- exact formatter-golden comparison when the current formatter already agrees.

The fixture `TOKENS`, `AST`, `SMG`, `EXPECTED`, and `PROBLEMS` sections are
retained as source evidence. They are not direct assertions because their
internal representations and diagnostic wording are implementation-specific. A
non-matching parser acceptance result or formatter golden produces an explicit
`SKIP` message (with its reason) under `--nocapture`; it is never silently
dropped. A non-UTF-8 fuzz fixture is likewise reported as skipped because the
pinned parser API accepts UTF-8 text. These skips are future compatibility work,
not passing conformance claims.

The runner's count guard prevents accidental loss of fixtures. When deliberately
refreshing the corpus, update `IN_SCOPE_SNAPSHOT_COUNT` in the test and record
the new count here or in the commit message.
