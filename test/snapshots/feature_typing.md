# META
~~~ini
description=Feature with type annotation
type=file
~~~
# SOURCE
~~~sysml
feature x : Integer;
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/feature_typing.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 0 12) (end 0 19))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:273caa5b768cf58a9a4a83d66fd3189f9d6739a9c6a1ca78d34372ac87a213b9") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/feature_typing.md") (qualified-name "x"))) (kind default-reference) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Integer"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/feature_typing.md") (qualified-name "x"))) (kind featureTyping) (ordinal 0))
      (authored-target "Integer")
      (outcome (status unresolved)))
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
  (query (document "memory://snapshot/feature_typing.md") (range (start 0 12) (end 0 19)) (probe (position 0 12))
    (reference (id (source (node (document "memory://snapshot/feature_typing.md") (qualified-name "x"))) (kind featureTyping) (ordinal 0) (authored-target "Integer")
      (outcome (status unresolved)))
  )
)
~~~
