# META
~~~ini
description=Diagnostics preserve canonical ordering for multiple unresolved type references
type=file
~~~
# SOURCE
~~~sysml
package P {
    part bad_first : MissingFirst;
    part bad_second : MissingSecond;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/diagnostic_canonical_order.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1 21) (end 1 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2 22) (end 2 35))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:d557c1a87a521aa276580c656df0928dfdde08d93aeca279d28ec8374aea8603") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/diagnostic_canonical_order.md") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagnostic_canonical_order.md") (qualified-name "P::bad_first"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MissingFirst")))))
    (declaration (id (node (document "memory://snapshot/diagnostic_canonical_order.md") (qualified-name "P::bad_second"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MissingSecond")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/diagnostic_canonical_order.md") (qualified-name "P::bad_first"))) (kind featureTyping) (ordinal 0))
      (authored-target "MissingFirst")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/diagnostic_canonical_order.md") (qualified-name "P::bad_second"))) (kind featureTyping) (ordinal 0))
      (authored-target "MissingSecond")
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
  (query (document "memory://snapshot/diagnostic_canonical_order.md") (range (start 1 21) (end 1 33)) (probe (position 1 21))
    (reference (id (source (node (document "memory://snapshot/diagnostic_canonical_order.md") (qualified-name "P::bad_first"))) (kind featureTyping) (ordinal 0) (authored-target "MissingFirst")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/diagnostic_canonical_order.md") (range (start 2 22) (end 2 35)) (probe (position 2 22))
    (reference (id (source (node (document "memory://snapshot/diagnostic_canonical_order.md") (qualified-name "P::bad_second"))) (kind featureTyping) (ordinal 0) (authored-target "MissingSecond")
      (outcome (status unresolved)))
    )
  )
)
~~~
