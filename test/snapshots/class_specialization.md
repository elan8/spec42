# META
~~~ini
description=Class with specialization
type=file
semantic_graph=skip
semantic_graph_skip_reason=standalone KerML class declarations are opaque parser fallback nodes; no structured specialization is available to semantic construction
~~~
# SOURCE
~~~sysml
class B :> A { }
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "class_specialization.md"
    (diagnostics
    )
  )
)
~~~
# TOKENS
~~~zig
KwClass,Ident,ColonGt,Ident,OpenCurly,CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (class_def 'B' :> 'A'))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'A'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'A'
~~~
# FORMAT
~~~sysml
class B :> A { }

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "7cb10869a753c15dde0b32b4924eee486d3dad8e80177d650ebfdbff6b187fdc") (contract-version "canonical-resolution-v1"))
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
