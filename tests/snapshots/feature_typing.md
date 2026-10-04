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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:457cdb59b8eb091603c091cedc9c4f5e6a5ac1c5503dbd01ad2e04551235347f"))
  (declarations
    (declaration (id (node (document "memory://snapshot/feature_typing.md") (qualified-name "x"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Integer")))))
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
)
~~~
