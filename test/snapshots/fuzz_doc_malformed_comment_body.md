# META
~~~ini
description=Documentation node with malformed comment body should close the comment when formatting
type=file
semantic_graph=skip
semantic_graph_skip_reason=parser recovery for non-empty source produced no typed semantic graph facts
~~~
# SOURCE
~~~sysml
alias Foo for Bar {
    doc /* unclosed comment
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "fuzz_doc_malformed_comment_body.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "expected_keyword")
        (source "sysml")
        (range (start 0 0) (end 0 19))
      )
      (diagnostic
        (severity error)
        (code "expected_keyword")
        (source "sysml")
        (range (start 1 4) (end 1 27))
      )
      (diagnostic
        (severity error)
        (code "unexpected_closing_brace")
        (source "sysml")
        (range (start 2 0) (end 2 1))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "a616379595f2ffb66a7aeaeb9c7b7dff19a0fb726be4922a105dcc3896fb7238") (contract-version "canonical-resolution-v1"))
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
