# Generator API compatibility policy

The core WebAssembly protocol is **ABI version 4**, specified in [ABI.md](./ABI.md). The
semantic model API reported by `model.info` remains version `0.1.0`.

## How compatibility is enforced

Every guest exports `spec42_abi_version`, returning `COMPATIBILITY_TOKEN`. The host compares
it against its own before running the module and refuses a mismatch with exit 11.
Compatibility is therefore checked mechanically, at load time.

The token covers the whole behavioural contract, because a hash of the wire types alone
leaves three classes of breaking change invisible:

| Input | Catches |
| --- | --- |
| `SCHEMA_FINGERPRINT` | any field added, removed, reordered or retyped |
| `operation::ALL` | an operation renamed, renumbered, added or removed |
| `Level::ALL` | a diagnostic level renumbered — these cross as raw integers, not through Postcard |
| `SEMANTIC_API_VERSION` | ordering, defaulting, query meaning, effective-feature shadowing |
| `ABI_VERSION` | an intentional break where nothing else moved |

Only the first is derived automatically from the types. The rest exist because operation
codes and level codes are plain constants that no type-level hash can see, and because
semantics can change with every byte of every type identical. `Level` and its code table come
from one macro so the discriminants and the table cannot diverge; `SEMANTIC_API_VERSION` is
declared once in `generator_protocol` and re-exported by `generator_api`, so the value feeding
the token and the value reported through `model.info` are the same constant.

This replaces the previous policy of giving each incompatible ABI a distinct import
namespace or entrypoint name. That rule was never implemented: ABI 1 to 2 changed the
entrypoint's result schema while keeping both the `spec42` namespace and the
`spec42_generate` name, and the only reason stale guests failed was the incidental removal
of an import.

## What a change costs

Any change to a type in [ABI.md](./ABI.md) is breaking, and every guest must be rebuilt.
There is no partial compatibility and no negotiation: the host supports exactly one schema.
Supporting more than one simultaneously, and capability negotiation, remain prerequisites
for declaring 1.0 stable.

When changing the schema:

1. Make the change in `crates/generator_protocol`. A semantic change with no type change
   means bumping `SEMANTIC_API_VERSION`; nothing else will detect it.
2. Update the pinned values in `the_wire_schema_fingerprint_is_pinned` and
   `the_compatibility_token_is_pinned`. Both fail deliberately so the break is acknowledged
   in review rather than discovered downstream.
3. Update the token quoted in [ABI.md](./ABI.md).
4. Rebuild the example generator and the conformance plugin corpus — they will be refused
   until they are.
5. Tell downstream guests — `roc-spec42` pins a Spec42 revision and needs a matching bump.

## What is not breaking

Nothing in the wire schema. `metaclass` and relationship `kind` were previously strings, on
the argument that an older guest could then ignore unfamiliar values. That argument does not
survive contact with the failure mode: a guest cannot distinguish a value it has never heard
of from one it forgot to handle, because both are just strings, so it silently produces wrong
output — exactly what the fingerprint exists to prevent. Both are now closed enumerations
whose `Unrecognized` variant labels the unknown case explicitly, and adding a real variant is
a breaking change like any other.

## Ordering guarantees

Results are ordered by qualified name, then normative metaclass, then semantic ID, unless a
query documents semantic precedence. `effective-features` is the exception: direct features
precede inherited features, inheritance is nearest-first, each level is deterministically
sorted, and a same-named nearer feature shadows a farther one.

The API distinguishes declared detail fields from the explicitly effective query. New APIs
must retain that distinction rather than silently changing a declared query to include
implied facts.
