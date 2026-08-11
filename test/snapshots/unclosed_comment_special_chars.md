# META
~~~ini
description=Unclosed comment with special characters should be preserved
type=file
semantic_graph=skip
semantic_graph_skip_reason=parser recovery for non-empty source produced no typed semantic graph facts
~~~
# SOURCE
~~~sysml
/* isio . /% #ato
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "unclosed_comment_special_chars.md"
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
# TOKENS
~~~zig
MalformedRegularComment,EndOfFile,
~~~
# AST
~~~
(root
  (malformed))
~~~
# EXPECTED
~~~
tokenize.UnclosedRegularComment
~~~
# PROBLEMS
~~~
tokenize.UnclosedRegularComment
~~~
# FORMAT
~~~sysml
/* isio . /% #ato
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "449f67e2a99307bc657b30ac87378edf7a9a5d3522ff07a55b55342a12053b69") (contract-version "canonical-resolution-v1"))
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
