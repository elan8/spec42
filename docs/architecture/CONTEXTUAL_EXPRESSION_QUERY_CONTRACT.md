# Contextual expressions and connector paths

`structure().expression(symbol)` returns an authored tree. The explicit
`structure().expression_in_context(symbol, context)` query returns its effective body in an
inheriting type or typed usage. Both answers are indexed before publication; reads perform no
resolution. The contextual answer retains the context, effective expression element, authored
body declaration, original source locations, and authored spellings.

Redefined bodies override inherited bodies. A redefining feature with no authored expression
inherits the most specific body along canonical redefinition edges. Feature references follow
effective redefinition identities; subsetting alone does not replace a binding. Dotted references
use the canonical type-directed member-access resolver against the contextual scope. Local
parameters and references outside the inherited owner's scope keep their original targets.

Competing bodies or bindings produce `ContextualExpressionOutcome::Ambiguous` with candidate
identities and no asserted tree. Invalid contexts or missing prerequisites produce an unresolved
query. Publication completeness and unsupported expression shapes remain explicit.

Connector `FeatureChain` answers contain one `RelationshipTarget` per authored segment, captured
by the canonical endpoint resolver. The ordered path identifies instance traversal even when
different paths share a terminal declaration. Failed hops remain explicit; later hops are
unresolved. Authored text never substitutes for missing semantic identities.

Model-export JSON adds `path` to feature-chain endpoints. Projection schema version 1 permits
additive fields; the semantic contract version changes with the new publication behavior.
The WASM generator ABI is unchanged; extending it requires a concrete consumer contract.
