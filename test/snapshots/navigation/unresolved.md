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
  (document "unresolved.md"
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
# FORMAT
~~~sysml
package P {
    part engine : MissingEngine;
}

~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "c699e7270eea0fccaf79e0e2e3f67bab02a00e3ad39699cb4c1a4668c7eb8261") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "P"))) (kind "package") (name "P") (declared-name "P"))
    (element (id (node (document "d0") (qualified-name "P::engine"))) (kind "part") (name "engine") (declared-name "engine") (parent (node (document "d0") (qualified-name "P"))) (authored (membership (kind Feature)) (relationships (typing (reference "MissingEngine")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "P::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "MissingEngine") (outcome (status unresolved)))
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
    (query (range (start 1 18) (end 1 31)) (probe (position 1 18))
      (reference
        (source (document "d0") (qualified-name "P::engine"))
        (kind featureTyping) (ordinal 0) (authored-target "MissingEngine")
        (range (start 1 18) (end 1 31))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
