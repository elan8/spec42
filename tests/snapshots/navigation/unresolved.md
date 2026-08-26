# META
~~~ini
description=Unresolved navigation remains an explicit outcome
type=file
~~~
# SOURCE
~~~sysml
package P {
    part engine : MissingEngine;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/unresolved.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1 18) (end 1 31))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:166a9a80f38c9979e36ac0c6be417fab282afa58f08bbc8c5ec20b9330399b64") (contract-version "constructor-expression-result-v8"))
  (declarations
    (declaration (id (node (document "memory://snapshot/unresolved.md") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/unresolved.md") (qualified-name "P::engine"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MissingEngine")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/unresolved.md") (qualified-name "P::engine"))) (kind featureTyping) (ordinal 0))
      (authored-target "MissingEngine")
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
  (query (document "memory://snapshot/unresolved.md") (range (start 1 18) (end 1 31)) (probe (position 1 18))
    (reference (id (source (node (document "memory://snapshot/unresolved.md") (qualified-name "P::engine"))) (kind featureTyping) (ordinal 0) (authored-target "MissingEngine")
      (outcome (status unresolved)))
    )
  )
)
~~~
