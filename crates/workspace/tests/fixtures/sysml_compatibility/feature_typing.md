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
# TOKENS
~~~zig
KwFeature,Ident,Colon,Ident,Semicolon,EndOfFile,
~~~
# AST
~~~
(root
  (feature_def 'x' : 'Integer'))
~~~
# FORMAT
~~~sysml
feature x : Integer;

~~~
# EXPECTED
~~~
semantic.unresolved_name 'Integer'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Integer'
~~~
# SMG
~~~
(semantic-graph
  (containment
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
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
