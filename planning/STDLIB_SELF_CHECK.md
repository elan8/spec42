# Standard-library self-check milestone

Spec42 does not yet validate its pinned standard library successfully. Reaching a clean
standard-library self-check is a major semantic-coverage milestone: the 94-file OMG library uses a
broad, interconnected portion of KerML and SysML, so publishing and checking it without errors is
strong evidence that the parser, lowering, name resolution, derived facts, type relationships, and
validation rules work together on a substantial real model.

This milestone means more than accepting the library's syntax. The canonical batch pipeline must
construct a complete semantic publication for the exact bundled library and report no errors. A
warning-free result is the target as well: unresolved references classified as warnings still mean
the semantic model is incomplete and must not be treated as successful compilation.

## Current blockers

Two distinct problems are visible with Spec42 0.50.0 and the pinned `2026-04` standard library.

1. A normal check of the materialized standard-library directory fails during request construction
   with exit code 2 and
   `internal_invariant_failure: RequestConstruction: DuplicateSourceIdentity`. The directory being
   checked and the automatically admitted bundled library contribute the same source identities.
   Checking an owning input must not admit a second copy of that input; fix this at library
   admission/request construction rather than suppressing the invariant.
2. Disabling automatic standard-library admission lets the canonical pipeline process all 94
   sources, but validation exits 1. The current baseline is 126 errors, 2,395 warnings, and 4
   informational diagnostics. The errors are 68 `ambiguous_reference`, 35
   `redefinition_type_incompatible`, 21 `subsetting_type_incompatible`, and 2
   `specialization_cycle` diagnostics. Of the warnings, 2,365 are `unresolved_reference`. There are
   no parser diagnostics in this run: the remaining failure is in semantic construction and
   checking, not basic syntax acceptance.

The counts are a reproducible baseline, not an acceptance contract. Fixes should add narrow tests
at the owning semantic layer; do not preserve a diagnostic or its count merely to match this
snapshot.

## Repeat the check

Build the CLI with the same embedded bundle used by normal distribution builds, then confirm which
library version and materialization it selected:

```sh
cargo build -p server --bin spec42 --features embed-stdlib
target/debug/spec42 --version
target/debug/spec42 stdlib status
stdlib_dir="$(target/debug/spec42 stdlib path)"
```

First exercise the user-facing command. This currently demonstrates the duplicate-source blocker:

```sh
target/debug/spec42 check "$stdlib_dir"
```

Then isolate semantic self-checking from automatic admission and retain a machine-readable report:

```sh
target/debug/spec42 --no-stdlib check "$stdlib_dir" --format json \
  > /tmp/spec42-stdlib-self-check.json
```

The expected current exit status is 1. Inspect the summary and diagnostic-code distribution with:

```sh
jq '.summary' /tmp/spec42-stdlib-self-check.json
jq '[.documents[].diagnostics[] | .code]
    | group_by(.)
    | map({code: .[0], count: length})
    | sort_by(-.count)' /tmp/spec42-stdlib-self-check.json
```

Do not pass `--strict-diagnostics` for the milestone check: it is a legacy reporting mode that can
skip semantic diagnostics after parse errors and suppress unresolved warnings. The milestone must
exercise the ordinary canonical check policy without hiding incomplete results.

## Completion criteria

The milestone is complete only when all of the following hold for the pinned bundled library:

- the normal `spec42 check "$(spec42 stdlib path)"` path has defined, non-duplicating admission
  semantics and reaches semantic validation rather than an internal invariant failure;
- all 94 sources publish as one coherent, complete model through the canonical batch pipeline;
- the self-check exits 0 with no unresolved, ambiguous, unsupported, partial, or failed semantic
  state represented as success;
- focused regression tests cover each repaired owning abstraction, including the duplicate-source
  admission case; and
- a standalone snapshot-tool fixture or other checked-in end-to-end contract exercises the same
  library publication so the milestone cannot regress silently.

When these criteria are satisfied, remove this planning document and its index entry. The commits,
tests, and any succinct user-visible changelog entry should retain the completed milestone history.
