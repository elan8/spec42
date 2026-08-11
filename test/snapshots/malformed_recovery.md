# META
~~~ini
description=Malformed tokens with recovery
type=file
semantic_graph=skip
semantic_graph_skip_reason=parser recovery for non-empty source produced no typed semantic graph facts
~~~
# SOURCE
~~~sysml
x ` y
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "malformed_recovery.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "expected_keyword")
        (source "sysml")
        (range (start 0 0) (end 0 5))
      )
    )
  )
)
~~~
# TOKENS
~~~zig
Ident,MalformedUnknownToken,Ident,EndOfFile,
~~~
# AST
~~~
(root
  (malformed))
~~~
# EXPECTED
~~~
parse.unexpected_token
~~~
# PROBLEMS
~~~
parse.unexpected_token
~~~
# FORMAT
~~~sysml
x ` y

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "364234af9ad23f3cd148380e18bd0c42c70d65c0798524239ee28f767da0c526") (contract-version "canonical-resolution-v1"))
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
