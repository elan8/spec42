# Generator API compatibility policy

The experimental core WebAssembly protocol is ABI version 1. Handles are opaque and valid only for
one invocation; `semantic-id` is the deterministic provenance identity. The semantic model API
reported by `model.info` remains version `0.1.0`.

The Postcard schema is positional. Adding, removing, or changing a field or operation requires a
new ABI version and matching SDK/host support. Each incompatible ABI uses a distinct import
namespace or entrypoint name so compatibility remains observable from ordinary module imports and
exports; version 1 uses the `spec42` import namespace. Supporting multiple ABI versions requires
separate linkers and codecs. String-valued metaclasses and relationship kinds may grow without
changing the binary schema.

Metaclasses and relationship kinds are strings so an older guest can ignore unknown semantic
concepts. Result ordering is qualified name, normative metaclass, then semantic ID unless a query
documents semantic precedence. `effective-features` is the exception: direct features precede
inherited features, inheritance is nearest-first, each level is deterministically sorted, and a
same-named nearer feature shadows a farther one.

The API distinguishes declared detail fields from the explicitly effective query. New APIs must
retain that distinction rather than silently changing a declared query to include implied facts.
Capability negotiation and more than one simultaneously supported API version remain required
before declaring 1.0 stable.
