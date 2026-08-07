# Generator API compatibility policy

The core WebAssembly protocol is **ABI version 3**, specified in [ABI.md](./ABI.md). The
semantic model API reported by `model.info` remains version `0.1.0`.

## How compatibility is enforced

Every guest exports `spec42_abi_version`, returning a structural fingerprint of the wire
schema. The host compares it against its own before running the module and refuses a
mismatch with exit 11. Compatibility is therefore checked mechanically, at load time, and
never depends on anyone remembering to bump a number.

The fingerprint is derived from the types themselves — `SCHEMA_FINGERPRINT` in
`crates/generator_protocol`. Adding, removing, reordering or retyping any field of any wire
type changes it, as does bumping `ABI_VERSION`, which is mixed into the hash so an
intentional break is observable even when no type changed.

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

1. Make the change in `crates/generator_protocol`.
2. Update the pinned value in `the_wire_schema_fingerprint_is_pinned`, which fails
   deliberately so the break is acknowledged in review rather than discovered downstream.
3. Update the fingerprint quoted in [ABI.md](./ABI.md).
4. Rebuild the example generator and the conformance plugin corpus.
5. Tell downstream guests — `roc-spec42` pins a Spec42 revision and needs a matching bump.

## What is not breaking

String-valued `metaclass` and relationship `kind` values may grow without changing the
binary schema, because they are strings precisely so an older guest can ignore concepts it
does not recognise. Guests must skip unknown values rather than failing on them.

## Ordering guarantees

Results are ordered by qualified name, then normative metaclass, then semantic ID, unless a
query documents semantic precedence. `effective-features` is the exception: direct features
precede inherited features, inheritance is nearest-first, each level is deterministically
sorted, and a same-named nearer feature shadows a farther one.

The API distinguishes declared detail fields from the explicitly effective query. New APIs
must retain that distinction rather than silently changing a declared query to include
implied facts.
