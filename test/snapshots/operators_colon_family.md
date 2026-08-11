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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "19f0ee67d2fc34fdbbe40ce2ba0c6d02b2cdd7e3deb1087781728b803bdb04d5") (contract-version "canonical-resolution-v1"))
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
