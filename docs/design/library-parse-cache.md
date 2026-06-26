# Library Parse Cache

## Problem

Every time the LSP server starts it parses the full standard library and domain
libraries from scratch.  For a typical installation the closure contains several
hundred `.sysml`/`.kerml` files.  Parsing is the dominant cost of the startup
scan phase (`parseWorkersMs` in perf logs).

The libraries are read-only between `spec42` version upgrades — their content
never changes during normal use.  We should parse each file once, cache the
result to disk keyed by content hash, and restore it on the next start instead
of re-parsing.

---

## Approach: cache `RootNamespace` at the parser level

The natural cache boundary is the `RootNamespace` value that `sysml-v2-parser`
produces.  Caching at this level means the entire parsing step is skipped on a
cache hit — no tokenising, no grammar traversal.  The graph-build step that
follows (`build_graph_from_doc`) is comparatively fast and can remain as-is.

Caching at the semantic-graph level (`SemanticGraph` nodes) was also considered
but rejected: `SemanticGraph` is not serializable in its current form (contains
`Mutex` and `petgraph::StableGraph` without the `serde` feature), and
reconstructing it from serialized nodes would still need to re-run all the
cross-document linking phases anyway, so the saving would be smaller.

---

## Required changes

### 1. `sysml-v2-parser` — add a `serde` feature

The parser currently has no serde dependency.  All AST types derive only
`Debug`, `Clone`, `PartialEq`, `Eq`.

#### 1a. `Cargo.toml`

```toml
[features]
default = []
serde = ["dep:serde"]

[dependencies]
serde = { version = "1", features = ["derive"], optional = true }
```

#### 1b. Every AST type gets a conditional derive

Add the following attribute to **every `pub struct` and `pub enum`** in the AST
modules (`core.rs`, `common.rs`, `package.rs`, `structure.rs`, `behavior.rs`,
`requirement.rs`, `view.rs`, `kerml_fallback.rs`, `root.rs`):

```rust
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
```

The types that need this are:

| File | Types |
|------|-------|
| `core.rs` | `Span`, `Node<T>`, `Expression`, `BinaryOp`, `UnaryOp` |
| `common.rs` | `Identification`, `Import`, `ImportKind`, `Visibility`, `Doc`, `Comment`, `TextualRep`, `Filter`, `Annotation`, `MetadataAnnotation`, `MetadataKeywordUsage`, `Dependency`, `AliasDef`, `Multiplicity`, `Direction` |
| `root.rs` | `RootNamespace`, `RootElement`, `NamespaceDecl` |
| `package.rs` | `Package`, `LibraryPackage`, `PackageBody`, `PackageBodyElement` + all nested body element types |
| `structure.rs` | All `*Def`, `*Usage`, `*Body`, `*BodyElement` types for parts, ports, interfaces, connections, attributes, enums, occurrences, metadata |
| `behavior.rs` | `ActionDef`, `ActionUsage`, `ActionDefBody`, `ActionUsageBody`, all body element variants, `Perform`, `PerformBody`, `Assign`, `ForLoop`, `Bind` etc. |
| `requirement.rs` | `RequirementDef`, `RequirementUsage`, `SatisfyStmt` etc. |
| `view.rs` | `ViewDef`, `ViewpointDef`, `RenderingDef`, `ViewUsage` etc. |
| `kerml_fallback.rs` | `KermlSemanticDecl`, `KermlFeatureDecl`, `ExtendedLibraryDecl` etc. |

**`Node<T>`** needs special attention because it is generic.  The conditional
derive works fine for generic structs as long as the bound is propagated, which
serde does automatically:

```rust
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Node<T> {
    pub span: Span,
    pub value: T,
}
```

Serde will automatically add `T: Serialize` / `T: Deserialize` bounds on the
generated impls — no manual bound annotation needed.

#### 1c. Emit a `PARSE_AST_VERSION` constant

Add a single constant to `lib.rs` that must be bumped whenever any AST type
changes shape (field added/removed/renamed, variant added/removed):

```rust
/// Incremented on every breaking AST change.  The parse cache uses this to
/// invalidate entries built against an older schema.
pub const PARSE_AST_VERSION: u32 = 1;
```

The cache in `lsp_server` embeds this value in every cache entry and refuses to
load entries where it does not match the running binary's value.

---

### 2. `spec42` workspace — enable the feature and add a cache module

#### 2a. `Cargo.toml` (workspace root or `lsp_server/Cargo.toml`)

```toml
sysml-v2-parser = { version = "0.25.6", features = ["serde"] }
```

Add `bincode` as a new dependency of `lsp_server` (fast binary serialization,
already widely used in Rust tool chains):

```toml
bincode = "2"          # 2.x API: encode_to_vec / decode_from_slice
```

`serde_json` would work but produces files ~4–6× larger and is slower to
encode/decode for deeply nested tree data.  Bincode is the right choice here.

#### 2b. New module: `crates/lsp_server/src/workspace/parse_cache.rs`

Responsible for reading and writing `RootNamespace` values to a disk cache.

**Cache key** — SHA-256 of the raw file bytes.  This is collision-free,
file-path-independent (no issues with symlinks or case-normalisation on
Windows), and automatically invalidates when file content changes.

**Cache location** — OS-specific application data directory, not the workspace
or the extension directory, so it survives extension reinstalls:

- Windows: `%LOCALAPPDATA%\spec42\parse-cache\`
- macOS: `~/Library/Caches/spec42/parse-cache/`
- Linux: `~/.cache/spec42/parse-cache/`

Resolved at runtime via the `dirs` crate (`dirs::cache_dir()`).  If the
directory cannot be created or written, the cache is silently skipped — caching
is always a best-effort optimisation.

**On-disk format** — one file per source file, named `<hex-sha256>.bin`:

```
[magic: 4 bytes "KPC\0"]
[ast_version: u32 little-endian]
[bincode-encoded RootNamespace]
```

The magic + version header allows fast rejection of stale or corrupt entries
without deserialising the whole tree.

**Public API of the module:**

```rust
/// Try to load a cached parse result for a file whose content hashes to `sha256`.
/// Returns `None` on any miss or error (stale version, corrupt data, I/O failure).
pub fn load(cache_dir: &Path, sha256: &[u8; 32]) -> Option<RootNamespace>;

/// Write a freshly parsed result to the cache.  Silently ignores I/O errors.
pub fn store(cache_dir: &Path, sha256: &[u8; 32], root: &RootNamespace);

/// Resolve the platform cache directory.  Returns `None` if the OS gives no
/// suitable directory.
pub fn default_cache_dir() -> Option<PathBuf>;

/// Delete cache entries whose `ast_version` does not match the current binary.
/// Called once at startup.  Non-fatal.
pub fn evict_stale_entries(cache_dir: &Path);
```

#### 2c. Hook the cache into `parse_scanned_entry`

In `crates/lsp_server/src/workspace/services.rs`, `parse_scanned_entry`
currently always calls `util::parse_for_editor`:

```rust
fn parse_scanned_entry(ordinal: usize, uri: Url, content: String) -> ParsedScanEntry {
    let parse_start = Instant::now();
    let parsed_result = std::panic::catch_unwind(...parse_for_editor(&content)...);
    ...
}
```

Change it to accept an optional cache directory and check the cache first:

```rust
fn parse_scanned_entry(
    ordinal: usize,
    uri: Url,
    content: String,
    cache_dir: Option<&Path>,   // None disables caching
) -> ParsedScanEntry {
    if let Some(dir) = cache_dir {
        let hash = sha256_of(content.as_bytes());
        if let Some(root) = parse_cache::load(dir, &hash) {
            return ParsedScanEntry {
                ordinal,
                uri,
                content,
                parsed: Some(root),
                parse_errors: vec![],
                parse_metadata: ParseMetadata { parse_time_ms: 0, parse_cached: true },
            };
        }
        // cache miss: parse normally then store
        let entry = parse_scanned_entry_cold(ordinal, uri, content);
        if let Some(root) = &entry.parsed {
            parse_cache::store(dir, &hash, root);
        }
        return entry;
    }
    parse_scanned_entry_cold(ordinal, uri, content)
}
```

`sha256_of` can be implemented with the `sha2` crate (already a transitive
dependency via the cargo toolchain, or add it directly — it is `no_std`
compatible and has no significant compile-time cost).

#### 2d. Pass the cache directory only for library files

Library file paths are already separated from workspace files during the startup
scan.  Pass `Some(cache_dir)` only when parsing the library entries:

```rust
// In `initialized` handler (documents.rs):
let cache_dir = parse_cache::default_cache_dir();

// workspace files — no cache (they change)
let parsed_entries = spawn_blocking(move || {
    parse_scanned_entries(entries, should_parallel_parse, None)
}).await...;

// library files — use cache
let library_parsed = spawn_blocking(move || {
    parse_scanned_entries(library_entries, parallel, cache_dir.as_deref())
}).await...;
```

`parse_scanned_entries` threads the `cache_dir` down to each call to
`parse_scanned_entry`.

#### 2e. Stale-entry eviction at startup

Call `parse_cache::evict_stale_entries(cache_dir)` once during `initialized`,
before the scan begins, on a `spawn_blocking` task.  It iterates the cache
directory, reads the 8-byte header of each file, and deletes any whose
`ast_version` field differs from `sysml_v2_parser::PARSE_AST_VERSION`.  This
keeps the cache from accumulating indefinitely across spec42 upgrades.

---

## What is NOT cached

- **User workspace files** — change on every edit; caching them would add
  overhead for zero benefit in the common case.
- **Files opened via `did_open`** — already in the index from the startup scan;
  only re-parsed on change.
- **The semantic graph** — cross-document linking is fast relative to parsing
  and depends on the combined state of all files, making partial caching
  complex.  This can be revisited if profiling shows it is significant.

---

## Expected impact

On a cold start with a warm cache (every start after the first), the
`parseWorkersMs` contribution from library files drops to near zero.  Only the
content hash computation (`O(file size)`, dominated by memory bandwidth) and
bincode deserialisation remain.  For the standard SysML library (~200 files,
~300 kB of source), this should reduce library parse time from hundreds of
milliseconds to low single-digit milliseconds.

---

## Implementation order

1. `sysml-v2-parser`: add `serde` feature, add derives to all AST types, add
   `PARSE_AST_VERSION` constant, publish new patch version.
2. `spec42/lsp_server`: bump parser version, add `bincode` + `sha2` + `dirs`
   dependencies, implement `parse_cache` module, wire into
   `parse_scanned_entries`.
3. Verify with perf logging: `parseWorkersMs` in the `backend:startupScanPhases`
   event should be near-zero for library files after the first run.
