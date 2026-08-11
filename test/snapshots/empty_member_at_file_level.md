# META
~~~ini
description=Empty member (bare semicolon) at file level
type=file
semantic_graph=skip
semantic_graph_skip_reason=parser recovery for non-empty source produced no typed semantic graph facts
~~~
# SOURCE
~~~sysml
; in v : SpeedVal
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "empty_member_at_file_level.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "expected_keyword")
        (source "sysml")
        (range (start 0 0) (end 0 17))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "fefede073c8c7ce43930b22b1d1c30639364c8e8b3185e2ef91212bd14218998") (contract-version "canonical-resolution-v1"))
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
