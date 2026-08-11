# META
~~~ini
description=Feature with type annotation
type=file
semantic_graph=skip
semantic_graph_skip_reason=standalone KerML feature declarations are opaque parser fallback nodes; no structured feature target is available to semantic construction
~~~
# SOURCE
~~~sysml
feature x : Integer;
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "feature_typing.md"
    (diagnostics
    )
  )
)
~~~
# TOKENS
~~~zig
KwFeature,Ident,Colon,Ident,Semicolon,EndOfFile,
~~~
# AST
~~~
(root
  (feature_def 'x' : 'Integer'))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Integer'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Integer'
~~~
# FORMAT
~~~sysml
feature x : Integer;

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "4455adac864df3e9d0ab6e9e8af50fa0492053aeac395cf985f3be80bddc7ced") (contract-version "canonical-resolution-v1"))
  (structure
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
