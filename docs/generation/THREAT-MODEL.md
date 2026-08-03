# Generator threat model

The generator module, its bytes, arguments, emitted paths, diagnostic strings, handles, and all
returned guest values are untrusted. Workspace model sources are trusted only to the same extent as
normal `spec42 check`; generation introduces no parser or linker.

## Protected assets

- files outside the explicit output root and unowned files within it;
- workspace sources, libraries, semantic graph, host process, credentials, and environment;
- availability of local and CI hosts;
- deterministic artifacts and trustworthy provenance.

## Controls

- The linker supplies only `spec42.query`, `spec42.emit`, and `spec42.diagnostic`. No WASI,
  network, filesystem, environment, clock, random, secret, or subprocess interfaces are linked.
- The core ABI validates memory ranges, transfer sizes, UTF-8, Postcard payloads, operation codes,
  and diagnostic levels. The facade validates opaque handles; artifact collection validates paths,
  duplicates, counts, and byte totals; diagnostics and query results are bounded.
- A fresh store and host state are used per call. Wasmtime traps are returned as categorized
  errors. Store memory limits, fuel, epoch wall-time interruption, and a host cancellation flag
  limit denial of service.
- Paths are normalized without guest-controlled canonicalization. The transaction rejects symlinks
  at the output root, at emitted path components, and anywhere copied from an existing output tree.
  Staging is a private sibling and final replacement uses same-filesystem renames with rollback.
- The manifest permits replacement only when the existing bytes still match the prior owned hash;
  otherwise explicit `--force` is required. Stale or unrecorded files are never deleted.
- Reports do not include ambient environment values or unrelated host paths. Model source URIs are
  included because they are an explicit input and are needed for diagnostics.

## Residual risks

- Cross-platform directory replacement is atomic only to the guarantees of the host filesystem;
  Windows locks can make commit fail, in which case rollback is attempted and its backup location
  is reported if rollback also fails.
- TOCTOU races remain possible when another host process mutates output between planning and the
  directory swap. Symlink refusal and whole-directory staging prevent escaping the selected root;
  descriptor-relative directory APIs may further narrow races on supported platforms.
- A generator can compute nondeterministic bytes internally. The host supplies deterministic order
  and no nondeterministic capability, but cannot prove guest purity.
- Module compilation consumes host resources before store limits apply. Wasmtime validates malformed
  binaries; separate input-size and compilation concurrency limits should be measured before
  accepting generators from multi-tenant remote users.
