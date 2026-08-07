# Sandboxed model generation

Spec42 can run a language-neutral core WebAssembly module against the same immutable semantic
snapshot used by its normal workspace engine:

```sh
spec42 generate generator.wasm model.sysml
```

Generated paths are written beneath `./generated` by default. Use `--output <DIR>` to select a
different destination root. A model path may be one SysML or KerML file or a workspace directory,
and all global standard-library, KPAR, configuration, and library-path options apply before
`generate` just as they do for `check`.

Generation stops before guest instantiation when validation contains an error. Warnings are printed
and are also present in JSON reports. The host does not expose source mutation or an alternative
parser/linker: its read-only model view comes from one immutable snapshot produced by the normal
workspace engine.

## Guest contract

The public contract is the core WebAssembly ABI specified in [ABI.md](./ABI.md), which is
complete enough to implement a guest in any language without reading the Rust SDK. A module
imports only two functions from the `spec42` module:

- `query`: deterministic, read-only semantic queries encoded with Postcard;
- `diagnostic`: bounded logs and element-associated diagnostics.

It exports `memory`, `spec42_abi_version`, `spec42_alloc`, and `spec42_generate`. Arguments
after CLI `--` are passed verbatim. The entrypoint returns a Postcard-encoded result containing
a list of records with a UTF-8 `file_path` and binary `contents`. Modules requiring WASI,
filesystem, sockets, environment variables, clocks, random, or subprocess imports fail
compatibility checking because those imports are never linked.

`spec42_abi_version` reports a structural fingerprint of the wire schema. Because Postcard is
positional and carries no field names or version marker, a guest built against a different
revision would otherwise misread every payload rather than fail; the host compares the
fingerprint and refuses a mismatched module before running it.

Any language or toolchain that can produce the required core WebAssembly imports and exports can
implement this contract. Pass the resulting core module directly to Spec42; no metadata or
post-processing step is required. Toolchains must not introduce WASI imports because this host does
not link them.

## Output ownership and transactions

Generated paths use `/`, are relative, and reject empty segments, `.`, `..`, absolute paths, drive
prefixes, backslashes, and NUL. The initial policy is:

- binary contents are preserved byte-for-byte;
- duplicate returned paths are an invocation failure;
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

A generator is build tooling: the user selected the module and invoked it, the same way they
invoke a compiler plugin. Execution is therefore **unmetered by default** — a slow generator
runs to completion, and `--max-fuel` and `--timeout-seconds` are opt-in rather than defaults
to be raised. The limits that always apply exist to contain accidents and to bound output:
256 MiB guest memory, 1,000 files, 4 KiB per returned path, 16 MiB per file, 128 MiB total
output, and 50,000 elements per query result.

Supplying `--max-fuel` also switches on fuel accounting, which is what makes `fuel_consumed`
appear in the report. Fuel is an exact, reproducible instruction count, so it is the useful
signal for comparing the cost of two runs; it is instrumentation first and a limit second.

The engine is configured for reproducibility: deterministic relaxed SIMD and NaN
canonicalization are both enabled, so a generator that does floating-point work produces the
same bytes on every host architecture. Reproducibility is guaranteed per Wasmtime version.

| Code | Meaning |
| ---: | --- |
| 0 | generated, unchanged check, or successful dry run |
| 10 | model loading or validation failure |
| 11 | malformed module or generator ABI incompatibility |
| 12 | guest error, trap, or cancellation |
| 13 | fuel, wall-time, memory, or runtime resource exhaustion |
| 14 | artifact path, output limit, conflict, symlink, or commit-policy failure |
| 15 | `--check` found byte differences |

JSON reports include snapshot and module digests, API and Spec42 versions, query count, output
totals, runtime, fuel consumed, diagnostics, and per-path operation classes. The manifest and digest
inputs contain no current time, random value, environment value, or machine-specific output path.
