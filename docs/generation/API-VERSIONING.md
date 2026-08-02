# Generator API compatibility policy

The experimental package is `elan8:spec42-generator@0.1.0`. Handles are opaque and valid only for
one invocation; `semantic-id` is the deterministic provenance identity.

Before 1.0, a minor package version may add functions, record fields, or string-valued semantic
concepts. Removing or changing a function/type requires a new incompatible minor package version.
Patch versions do not change WIT shape. The host initially supports exactly 0.1.x-compatible shape;
support for multiple package versions requires separate generated bindings and linkers.

Metaclasses and relationship kinds are strings so an older guest can ignore unknown semantic
concepts. Result ordering is qualified name, normative metaclass, then semantic ID unless a query
documents semantic precedence. `effective-features` is the exception: direct features precede
inherited features, inheritance is nearest-first, each level is deterministically sorted, and a
same-named nearer feature shadows a farther one.

The API distinguishes declared detail fields from the explicitly effective query. New APIs must
retain that distinction rather than silently changing a declared query to include implied facts.
Capability negotiation and more than one simultaneously supported API version remain required
before declaring 1.0 stable.
