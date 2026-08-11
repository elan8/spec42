# META
~~~ini
description=Empty input produces only EndOfFile
type=file
~~~
# SOURCE
~~~sysml
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "empty_input.md"
    (diagnostics
    )
  )
)
~~~
# TOKENS
~~~zig
EndOfFile,
~~~
# AST
~~~
(root)
~~~
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# FORMAT
~~~sysml


~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "d3a0016c7eb42f3bc7248f478468d13a3100c306984e7d2aa1f927676d352f6e") (contract-version "canonical-resolution-v1"))
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
