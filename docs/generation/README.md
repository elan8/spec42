# Sandboxed model generation

Spec42 can run a language-neutral WebAssembly Component against the same immutable semantic
snapshot used by its normal workspace engine:

```sh
spec42 generate generator.component.wasm model.sysml --output generated -- target=rust
```

`--output` is mandatory. This avoids an implicit write location. A model path may be one SysML or
KerML file or a workspace directory, and all global standard-library, KPAR, configuration, and
library-path options apply before `generate` just as they do for `check`.

Generation stops before guest instantiation when validation contains an error. Warnings are printed
and are also present in JSON reports. The host does not expose source mutation or an alternative
parser/linker: `GeneratorModelView` reads one `HostWorkspaceSnapshot` produced by the normal engine.

## Guest contract

The public contract is [`generator.wit`](../../crates/generator_host/wit/generator.wit), package
`elan8:spec42-generator@0.1.0`. It imports only:

- `model`: deterministic, read-only semantic queries;
- `artifacts`: bounded, staged byte emission;
- `diagnostics`: bounded generator logs and element-associated diagnostics.

It exports `generate(args: list<string>) -> result<_, string>`. Arguments after CLI `--` are passed
verbatim. Components requiring WASI, filesystem, sockets, environment variables, clocks, random,
or subprocess imports fail compatibility checking because those imports are never linked.

The first-party Rust SDK is in [`crates/generator_sdk`](../../crates/generator_sdk), and a standalone
two-file example is in [`generator-examples/rust`](../../generator-examples/rust). Build guests for
`wasm32-unknown-unknown` and wrap their WIT metadata with `wasm-tools component new`; using
`wasm32-wasip2` would intentionally introduce WASI imports that this host refuses.

## Output ownership and transactions

Generated paths use `/`, are relative, and reject empty segments, `.`, `..`, absolute paths, drive
prefixes, backslashes, and NUL. The initial policy is:

- binary contents are preserved byte-for-byte;
- duplicate emission is an invocation failure;
- unowned or locally modified files conflict unless `--force` is explicit;
- files absent from the current generation are never deleted;
- `.spec42-generator-manifest.json` records generator, model, API, Spec42, and artifact hashes;
- output containing symlinks is refused rather than traversed.

Artifacts remain in memory until the guest succeeds. Commit copies the existing tree to a private
sibling staging directory, applies every artifact and the manifest there, then swaps the output
directory with same-filesystem renames. If installation fails after moving the old tree, the host
renames the backup into place. A trap, cancellation, validation error, limit violation, rejected
path, or guest error never reaches commit.

`--dry-run` reports the proposed create/change operations. `--check` compares bytes without writing
and exits 15 when output would differ. Stale deletion is intentionally deferred; a future `--clean`
must still never delete a path absent from the prior manifest.

## Limits and exit codes

Defaults are 256 MiB guest memory, 100 million fuel units, 30 seconds wall time, 1,000 files,
16 MiB per file, 128 MiB total output, and 50,000 elements per query result. CLI limit flags can
reduce or increase those values explicitly.

| Code | Meaning |
| ---: | --- |
| 0 | generated, unchanged check, or successful dry run |
| 10 | model loading or validation failure |
| 11 | malformed component or generator API incompatibility |
| 12 | guest error, trap, or cancellation |
| 13 | fuel, wall-time, memory, or runtime resource exhaustion |
| 14 | artifact path, output limit, conflict, symlink, or commit-policy failure |
| 15 | `--check` found byte differences |

JSON reports include snapshot and component digests, API and Spec42 versions, query count, output
totals, runtime, fuel consumed, diagnostics, and per-path operation classes. The manifest and digest
inputs contain no current time, random value, environment value, or machine-specific output path.
