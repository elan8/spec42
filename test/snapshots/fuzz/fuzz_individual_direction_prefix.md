# META
~~~ini
description=Fuzz: individual usage with direction prefix preserves 'individual' keyword
type=file
semantic_graph=skip
semantic_graph_skip_reason=parser recovery for non-empty source produced no typed semantic graph facts
~~~
# SOURCE
~~~sysml
in individual it;
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "fuzz_individual_direction_prefix.md"
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
# FORMAT
~~~sysml
in individual it;

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "ef7d5cacf4ffda28ef856a92dd9dd19d8c40322bbf707bb42987c977680dea7b") (contract-version "canonical-resolution-v1"))
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
