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
  (document "diagnostic_canonical_order.md"
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
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "c3e481ddbf9d9d60a5b83c95dd71bced821e9b828cc334d4406d4fbe9079c88b") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "P"))) (kind "package") (name "P") (declared-name "P"))
    (element (id (node (document "d0") (qualified-name "P::bad_first"))) (kind "part") (name "bad_first") (declared-name "bad_first") (parent (node (document "d0") (qualified-name "P"))) (authored (membership (kind Feature)) (relationships (typing (reference "MissingFirst")))))
    (element (id (node (document "d0") (qualified-name "P::bad_second"))) (kind "part") (name "bad_second") (declared-name "bad_second") (parent (node (document "d0") (qualified-name "P"))) (authored (membership (kind Feature)) (relationships (typing (reference "MissingSecond")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "P::bad_first"))) (kind featureTyping) (ordinal 0)) (authored-target "MissingFirst") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "P::bad_second"))) (kind featureTyping) (ordinal 0)) (authored-target "MissingSecond") (outcome (status unresolved)))
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
  (document "d0"
    (query (range (start 1 21) (end 1 33)) (probe (position 1 21))
      (reference
        (source (document "d0") (qualified-name "P::bad_first"))
        (kind featureTyping) (ordinal 0) (authored-target "MissingFirst")
        (range (start 1 21) (end 1 33))
        (outcome (status unresolved))
      )
    )
    (query (range (start 2 22) (end 2 35)) (probe (position 2 22))
      (reference
        (source (document "d0") (qualified-name "P::bad_second"))
        (kind featureTyping) (ordinal 0) (authored-target "MissingSecond")
        (range (start 2 22) (end 2 35))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
