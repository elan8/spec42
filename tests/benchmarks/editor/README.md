# Live editor benchmark inputs

`small` is a minimal authoring workspace. `inheritance` combines a 40-level specialization
chain with 300 typed usages. These are the exact source bytes used in the controlled namespace
memo comparison recorded in issue #192. They exercise the normal host with its pinned admitted
libraries; they are performance workloads, not a normative semantic conformance inventory.

Build the normal release host (`cargo build --locked --release -p server --bin spec42`) using
this checkout's own target directory. Prepare pinned library bundles as described in
`DEVELOPMENT.md`. The benchmark rejects unavailable or mismatched standard-library admission.

From the repository root, for example:

```powershell
python scripts/benchmark-editor.py --server target/release/spec42.exe --server-arg=lsp --profile release --workspace tests/benchmarks/editor/inheritance --document tests/benchmarks/editor/inheritance/model.sysml --iterations 5 --burst-size 5 --change-kind full --output target/editor-baseline/inheritance-full.json
```

Repeat with `--change-kind incremental` and the `small` workspace. Robot-vacuum and elevator
runs use the corresponding project root and `model/Root.sysml` from the user's model repositories.
The report records all discovered source/manifest hashes and the resolved library input hashes.

For before/after collection, use the same absolute workspace/document paths for both binaries.
Keep each checkout at the source revision of the binary being measured; record and verify the
binary, runner, configuration and library hashes. Each build uses its own Cargo target; share
download caches only. Finish compilation and other test work before collecting timings.
Moving these fixtures to a different absolute path changes document identities and therefore
model digests, so compare model digests only for runs using the same document identities.

Five single edits and five bursts of five yield 30 samples per run. Preserve raw reports and
report all workload combinations, including regressions and variability. These are transport and
handler-completion timings: they do not measure version-correlated diagnostics or view refresh,
and these small sample counts do not establish release regression thresholds.
