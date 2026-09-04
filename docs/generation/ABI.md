# Spec42 generator ABI

The wire contract between Spec42 and a generator module. This document is the
specification; `crates/generator_sdk` is one implementation of it, not its definition. A
guest written in any language that can emit the imports and exports below is equally valid.

<!-- generated:abi-header -->
Current version: **ABI 5**. Compatibility token: `0xfe7b3c6064f18e26`.
<!-- /generated:abi-header -->

The tables below and `generator-abi.json` are generated from the contract declaration in
`crates/generator_protocol/src/contract.rs`; run `node scripts/sync-generator-abi.mjs` after
changing it.

A generator is plain **core WebAssembly** — not a component. No metadata, no
post-processing, no `wasm-tools` step. Pass the `.wasm` straight to `spec42 generate`.

## Exports the guest must provide

| Export | Signature | Purpose |
| --- | --- | --- |
| `memory` | linear memory | All data crosses through this |
| `spec42_abi_version` | `() -> i64` | Compatibility token (see below) |
| `spec42_alloc` | `(i32) -> i32` | Allocate `n` bytes, return a pointer |
| `spec42_generate` | `(i32, i32) -> i64` | Entrypoint |

The host validates all four at load time — presence *and* signature — and refuses the
module otherwise. Memory must be 32-bit.

### `spec42_abi_version`

Returns the compatibility token of the ABI the guest was built against, as a bit pattern in
an `i64`. The host compares it against its own and refuses the module if they differ, before
the entrypoint runs, so a mismatch costs nothing but a clear error.

The token covers the **whole** contract, not just the types:

- a structural hash of every type in this document, so adding, removing, reordering or
  retyping any field moves it, including changes nobody thought were breaking;
- the operation numbering, because operation codes are plain constants that no type-level
  hash can see — renumbering one would otherwise route an old guest's `children` request to
  `element` with every type identical;
- the diagnostic level codes, which cross as raw integers rather than through Postcard and
  are equally invisible to a type-level hash;
- a semantic API version, bumped when observable behaviour changes with no type change at
  all: result ordering, defaulting, what a query means, effective-feature shadowing.

Do not compute it yourself: take `COMPATIBILITY_TOKEN` from the protocol crate, or copy the
constant above and update it when you rebuild against a newer Spec42.

The reason this exists is that Postcard is positional. There is no field name, type tag or
length prefix to disagree about at runtime: an extra field in a struct shifts every later
field by one, and because most of them are `bool` and `Option`, the shifted bytes still
decode. Without this check the failure mode is not an error, it is wrong output.

Before 1.0 the rule is deliberately blunt: any breaking change moves the token and an
incompatible guest is refused. Negotiation, and supporting more than one version at once,
can come later.

### `spec42_alloc`

Must return a pointer to `n` writable bytes, and must **not** return `0` — the host treats a
null result as a contract violation rather than writing over address 0. Signal failure by
trapping. The host calls this exactly once per run, to pass arguments.

### `spec42_generate`

Called once. Receives `(args_ptr, args_len)` and returns a packed pointer and length:

```
result = (length << 32) | pointer
```

with `pointer` in the low 32 bits and `length` in the high 32. Both refer to guest memory,
and the range must lie inside it.

The bytes at that range are a Postcard-encoded `Result<Vec<Artifact>, String>`. `Ok` yields
the generated files; `Err` yields a message the user sees, and the run fails without writing
anything.

## Imports the host provides

Both live in the `spec42` module. **Nothing else is linked** — no WASI, filesystem, sockets,
clock, random or environment access. A module importing anything else fails to load, which
is the mechanism by which generation stays a pure function of the model.

```
(import "spec42" "query"      (func (param i32 i32 i32 i32 i32) (result i64)))
(import "spec42" "diagnostic" (func (param i32 i32 i32 i32 i32)))
```

### `query(operation, request_ptr, request_len, response_ptr, response_capacity) -> i64`

Reads a Postcard request, writes a Postcard response into the guest buffer, and returns:

- **`n >= 0`** — success; `n` bytes were written at `response_ptr`.
- **`n < 0`** — the buffer was too small; `-n` is the size required. Grow and call again
  with identical arguments. The host recomputes nothing between the two calls, so this costs
  one extra round trip and no extra query work.

The response is always a Postcard `Result<T, String>`, so a zero-length response is
impossible and `0` unambiguously means "success, nothing written".

<!-- generated:abi-operations -->
| Op | Name | Request | Response `T` |
| --: | --- | --- | --- |
| 0 | `info` | `()` | `ModelInfo` |
| 1 | `roots` | `()` | `Vec<ElementSummary>` |
| 2 | `find` | `Option<String>` | `Vec<ElementSummary>` |
| 3 | `children` | `String` | `Vec<ElementSummary>` |
| 4 | `element` | `String` | `ElementDetail` |
| 5 | `typed_by` | `String` | `Option<ElementSummary>` |
| 6 | `relationships` | `String` | `Vec<Relationship>` |
| 7 | `effective_features` | `String` | `Vec<ElementSummary>` |
| 8 | `requirement_typing` | `String` | `RequirementUsageTyping` |
| 9 | `satisfy_relationships` | `()` | `Vec<SatisfyRelationship>` |
| 10 | `requirement_verifications` | `()` | `Vec<RequirementVerification>` |
| 11 | `diagram_views` | `()` | `Vec<DiagramViewSummary>` |
| 12 | `diagram_view` | `String` | `DiagramViewProjection` |
<!-- /generated:abi-operations -->

Note that `find` takes `Option<String>`: `None` means "every element". An empty string is
*not* the same thing, and the two are one byte apart on the wire — `None` encodes as `0x00`,
`Some("")` as `0x01 0x00`.

Any other operation code is a contract violation and fails the run.

### `diagnostic(level, message_ptr, message_len, element_ptr, element_len)`

Records a message, at one of these levels:

<!-- generated:abi-levels -->
| Level | Code |
| --- | --: |
| `debug` | 0 |
| `info` | 1 |
| `warning` | 2 |
| `error` | 3 |
<!-- /generated:abi-levels -->

Pass
`element_ptr = 0, element_len = 0` for a message not tied to an element; otherwise pass a
handle obtained from a query.

Fire and forget, with bounds the guest cannot observe: messages over 64 KiB are truncated,
at most 10,000 are kept, the total is capped at 4 MiB, and a handle that does not resolve is
dropped so the message becomes unscoped rather than being rejected.

## Memory ownership

Three buffers cross the boundary and they do **not** follow the same rule. This is the part
most likely to be got wrong.

| Buffer | Allocated by | Freed by |
| --- | --- | --- |
| Arguments to `spec42_generate` | Host, via `spec42_alloc` | **Guest** — ownership transfers with the call |
| Query response | Guest, before calling `query` | Guest |
| Result of `spec42_generate` | Guest | **Nobody** |

The host never frees guest memory: it reads the result and discards the entire store
immediately afterwards. Leaking the result buffer is correct and intended — there is no
`spec42_dealloc`, and a guest that tries to free the result during its own entrypoint has a
use-after-free.

`args_len` is exactly the length passed to `spec42_alloc`, so a guest whose allocator needs
the original size to free can rely on it.

## Encoding

[Postcard](https://postcard.jamesmunns.com/) v1, which is compact and non-self-describing:

- integers are LEB128 varints; `u32` occupies 1–5 bytes
- `String` and `Vec<T>` are a varint length followed by elements
- `Option` is `0x00` for `None`, or `0x01` followed by the value
- `enum` is a varint variant index, then the payload
- `struct` is its fields in declaration order, with nothing between them

There is no framing, no field name and no version marker anywhere in the payload. Field
order below is normative.

## Types

```rust
struct ModelInfo { model_digest: String, spec42_version: String, semantic_api_version: String }

struct ElementSummary {
    handle: String, semantic_id: String, metaclass: Metaclass,
    name: Option<String>, qualified_name: String, library_element: bool,
}

/// Closed enumeration; see the protocol crate for the full variant list. Encoded as a
/// Postcard enum: a varint variant index, and for `Unrecognized` a trailing string.
enum Metaclass { Package, PartDefinition, PartUsage, /* ... */ Unrecognized(String) }

enum RelationshipKind { Typing, Specializes, Subsetting, /* ... */ Unrecognized(String) }

struct SourceRange { start_line: u32, start_character: u32, end_line: u32, end_character: u32 }

struct Multiplicity {
    lower: Option<String>, upper: Option<String>,
    ordered: bool, unique: Option<bool>, implied: bool,
}

struct ElementDetail {
    summary: ElementSummary, owner: Option<ElementSummary>,
    declared_name: Option<String>, effective_name: Option<String>,
    source_uri: String, source_range: SourceRange,
    definition: bool, documentation: Option<String>, short_name: Option<String>,
    direction: Option<String>, derived: bool, constant: bool, abstract_flag: bool,
    variation: bool, individual: bool, conjugated: bool,
    composite: Option<bool>, reference: Option<bool>, end: bool,
    ordered: Option<bool>, unique: Option<bool>,
    multiplicity: Option<Multiplicity>, evaluated_value: Option<String>,
}

struct Relationship { kind: RelationshipKind, source: ElementSummary, target: ElementSummary, implied: bool }

struct Artifact { file_path: String, contents: Vec<u8> }
```

`handle` and `semantic_id` are both opaque strings and are **not** interchangeable. They address
elements within an immutable publication and must not be persisted or embedded in externally
consumed diagram output. Diagram queries translate them to `DiagramSemanticReference`: a
document-scoped qualified name, an authoritative tooling/library element ID when one exists, or an
explicit source anchor for an unnamed element. The diagram product interns these values and source
documents/ranges into normalized tables. Its numeric indexes are local foreign keys for that one
artifact and are not another identity domain. Do not persist handles or product-local indexes
between runs.

`metaclass` and `kind` are closed enumerations, so a guest can match them exhaustively and
the compiler will point out variants it has not handled. The mapping from Spec42's internal
element kinds is itself an exhaustive match with no fallback, so a new internal kind is a
compile error rather than something that reaches guests unannounced.

Each still carries an `Unrecognized(String)` variant, reachable in exactly one case: Spec42's
parser did not classify the construct either, so there is nothing better to publish than the
raw spelling. Encountering one means a parser gap, not an ABI gap. Adding a real variant
moves the compatibility token, so older guests are refused at load rather than silently
mishandling a value they cannot name.

## Artifact paths

Relative, `/`-separated, and rejected if they are empty, absolute, contain a drive prefix, a
backslash, an empty segment, `.` or `..`, or exceed 4 KiB.

Segments are also rejected for anything that aliases another name on Windows, on every
platform, so an output set does not depend on where the generator runs: the reserved
characters `< > : " | ? *`, control characters, trailing dots or spaces, and device names such
as `NUL.txt` — including `COM¹`–`COM³` and `LPT¹`–`LPT³`, since Windows reads those
superscripts as digits. `:` matters most — on NTFS it opens an alternate data stream, so
`manifest.json::$DATA` addresses an existing file's default stream. Returning the same path
twice fails the run, as does colliding with `.spec42-generator-manifest.json`. Contents are
written byte for byte.

## Determinism

Generated bytes are compared with `--check`, so a generator must be a pure function of the
model and its arguments. The host removes the obvious sources of variation — there is no
clock, no randomness, no environment, query results are deterministically ordered, and the
engine pins NaN canonicalization and deterministic relaxed SIMD so floating-point work does
not vary by host architecture — but it cannot prove a guest is pure. Do not embed
timestamps, iteration order of a hash map, or `model_digest` (which includes the engine
version, so it changes on every Spec42 release) in generated output.

## Writing a guest

1. Export the four functions above.
2. Return the fingerprint from `spec42_abi_version`.
3. Decode arguments as a Postcard `Vec<String>`.
4. Query the model as needed, handling the negative-return resize protocol.
5. Return a Postcard `Result<Vec<Artifact>, String>` at a packed pointer and length.

For Rust, `spec42-generator-sdk` does all of this; `export!(YourGenerator)` emits the export
set. See `generator-plugins/example`. For other languages, the table above is the whole
contract — `crates/generator_conformance` can run its corpus against any module, so a new
implementation can be checked against the same suite the Rust SDK is.
