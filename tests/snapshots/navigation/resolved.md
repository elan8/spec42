# META
~~~ini
description=Resolved navigation preserves target identity and range
type=file
~~~
# SOURCE
~~~sysml
package P {
    part def Engine;
    part engine : Engine;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/resolved.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:bb87776e2a9c30329de4b07bbf8a3bb99767a60e78eea7f45b53b29fb9bb97ad") (contract-version "feature-value-expression-results-v5"))
  (declarations
    (declaration (id (node (document "memory://snapshot/resolved.md") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/resolved.md") (qualified-name "P::Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/resolved.md") (qualified-name "P::engine"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/resolved.md") (qualified-name "P::engine"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/resolved.md") (qualified-name "P::Engine")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/resolved.md") (qualified-name "P::engine"))) (target (node (document "memory://snapshot/resolved.md") (qualified-name "P::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/resolved.md") (qualified-name "P::engine"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/resolved.md") (qualified-name "P::Engine")))
      (subtype (node (document "memory://snapshot/resolved.md") (qualified-name "P::engine")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/resolved.md") (qualified-name "P::engine")))
      (type (node (document "memory://snapshot/resolved.md") (qualified-name "P::Engine")) (provenance authored))
      (effective-type (node (document "memory://snapshot/resolved.md") (qualified-name "P::Engine")) (source direct))
      (supertype (node (document "memory://snapshot/resolved.md") (qualified-name "P::Engine")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/resolved.md") (range (start 2 18) (end 2 24)) (probe (position 2 18))
    (reference (id (source (node (document "memory://snapshot/resolved.md") (qualified-name "P::engine"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/resolved.md") (qualified-name "P::Engine")))))
    )
  )
)
~~~
