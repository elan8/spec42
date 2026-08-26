# META
~~~ini
description=Unclosed comment (missing */) at file level should be preserved
type=file
~~~
# SOURCE
~~~sysml
/* unclosed comment without closing marker
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/unclosed_comment_at_file_level.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "expected_keyword")
        (source "parser")
        (range (start 0 0) (end 0 40))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:ab79cf05669b2343fd46a4c67beb48b010f5b0edc172985ff1ae589b3bf5148b") (contract-version "feature-chain-expression-result-v10"))
  (declarations
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
