# META
~~~ini
description=Unclosed comment (missing */) at file level should be preserved
type=file
semantic_graph=skip
semantic_graph_skip_reason=parser recovery for non-empty source produced no typed semantic graph facts
~~~
# SOURCE
~~~sysml
/* unclosed comment without closing marker
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "unclosed_comment_at_file_level.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "expected_keyword")
        (source "sysml")
        (range (start 0 0) (end 0 40))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
/* unclosed comment without closing marker
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "240228c74fcd5e4e87262c76b2c76ac5f891de2b97f989064670f2de5851b7ba") (contract-version "canonical-resolution-v1"))
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
