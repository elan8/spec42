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
# FORMAT
~~~sysml
feature x : Integer;

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "c4d81fc5b52c6b692cf163f606fc32e73ac142224d82ad8d7ed2d3cb3b1a3a9a") (contract-version "canonical-resolution-v1"))
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
# NAVIGATION
~~~sexpr
(navigation
)
~~~
