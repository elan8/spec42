# META
~~~ini
description=Colon family operators
type=file
semantic_graph=skip
semantic_graph_skip_reason=parser recovery for non-empty source produced no typed semantic graph facts
~~~
# SOURCE
~~~sysml
: :: :> ::> :>> :=
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "operators_colon_family.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "expected_keyword")
        (source "sysml")
        (range (start 0 0) (end 0 18))
      )
    )
  )
)
~~~
# TOKENS
~~~zig
Colon,ColonColon,ColonGt,ColonColonGt,ColonGtGt,ColonEq,EndOfFile,
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
: :: :> ::> :>> :=

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "68797422f77b0cb8a22f939e61771c3f8455dbc8fb7e3963f924b6ab3941f0f2") (contract-version "canonical-resolution-v1"))
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
