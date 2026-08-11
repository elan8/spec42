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
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "1549f253e83e7ef1793b8438c839915f3a33541c0e142914a5bf28b7325802fd") (contract-version "canonical-resolution-v1"))
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
