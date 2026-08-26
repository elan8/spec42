# META
~~~ini
description=Unclosed comment with special characters should be preserved
type=file
~~~
# SOURCE
~~~sysml
/* isio . /% #ato
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/unclosed_comment_special_chars.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "expected_keyword")
        (source "parser")
        (range (start 0 0) (end 0 17))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:b48eaed3ffef5fa6c2ef90d301a3c3e8aa541a18fc378f7888b82447da3ce8f0") (contract-version "feature-chain-expression-result-v10"))
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
