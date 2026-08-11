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
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "847da806f6d5257ce2de2585df9841b6fbac00c6f774383a1f78d6dafdcf7eae") (contract-version "canonical-resolution-v1"))
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
