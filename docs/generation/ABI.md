# Spec42 generator ABI

The wire contract between Spec42 and a generator module. This document is the
specification; `crates/generator_sdk` is one implementation of it, not its definition. A
guest written in any language that can emit the imports and exports below is equally valid.

Current version: **ABI 4**. Wire schema fingerprint: `0x05508bec442da0db`.

A generator is plain **core WebAssembly** — not a component. No metadata, no
post-processing, no `wasm-tools` step. Pass the `.wasm` straight to `spec42 generate`.

## Exports the guest must provide

| Export | Signature | Purpose |
| --- | --- | --- |
| `memory` | linear memory | All data crosses through this |
| `spec42_abi_version` | `() -> i64` | Wire schema fingerprint (see below) |
| `spec42_alloc` | `(i32) -> i32` | Allocate `n` bytes, return a pointer |
| `spec42_generate` | `(i32, i32) -> i64` | Entrypoint |

The host validates all four at load time — presence *and* signature — and refuses the
module otherwise. Memory must be 32-bit.

### `spec42_abi_version`

Returns the fingerprint of the wire schema the guest was built against, as a bit pattern in
an `i64`. The host compares it against its own and refuses the module if they differ. This
is checked before the entrypoint runs, so a mismatch costs nothing but a clear error.

The value is a structural hash of every type in this document. It changes automatically when
any field is added, removed, reordered or retyped — including changes nobody thought were
breaking. Do not compute it yourself: take it from the protocol crate, or copy the constant
above and update it when you rebuild against a newer Spec42.

The reason this exists is that Postcard is positional. There is no field name, type tag or
length prefix to disagree about at runtime: an extra field in a struct shifts every later
field by one, and because most of them are `bool` and `Option`, the shifted bytes still
decode. Without this check the failure mode is not an error, it is wrong output.

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

| Op | Name | Request | Response `T` |
| --: | --- | --- | --- |
| 0 | `info` | `()` | `ModelInfo` |
| 1 | `roots` | `()` | `Vec<ElementSummary>` |
| 2 | `find` | `Option<String>` (metaclass filter) | `Vec<ElementSummary>` |
| 3 | `children` | `String` (owner handle) | `Vec<ElementSummary>` |
| 4 | `element` | `String` (handle) | `ElementDetail` |
| 5 | `typed_by` | `String` (feature handle) | `Option<ElementSummary>` |
| 6 | `relationships` | `String` (handle) | `Vec<Relationship>` |
| 7 | `effective_features` | `String` (handle) | `Vec<ElementSummary>` |

Note that `find` takes `Option<String>`: `None` means "every element". An empty string is
*not* the same thing, and the two are one byte apart on the wire — `None` encodes as `0x00`,
`Some("")` as `0x01 0x00`.

Any other operation code is a contract violation and fails the run.

### `diagnostic(level, message_ptr, message_len, element_ptr, element_len)`

Records a message. `level` is `0` debug, `1` info, `2` warning, `3` error. Pass
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

`handle` and `semantic_id` are both opaque strings and are **not** interchangeable. A handle
addresses an element for the duration of one run; `semantic_id` is the stable provenance
identity and is what to embed in generated output. Do not persist handles between runs.

`metaclass` and `kind` are closed enumerations, so a guest can match them exhaustively and
the compiler will point out variants it has not handled. Each carries an `Unrecognized(String)`
variant for a value this Spec42 produced but the enumeration does not name; Spec42's own
conformance suite asserts it is never produced, so encountering one means the upstream model
gained a concept the ABI has not yet mapped. Adding a real variant changes the fingerprint,
so older guests are refused at load rather than silently mishandling it.

## Artifact paths

Relative, `/`-separated, and rejected if they are empty, absolute, contain a drive prefix, a
backslash, a NUL, an empty segment, `.` or `..`, or exceed 4 KiB. Returning the same path
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
set. See `generator-examples/rust`. For other languages, the table above is the whole
contract — `crates/generator_conformance` can run its corpus against any module, so a new
implementation can be checked against the same suite the Rust SDK is.
