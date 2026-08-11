# META
~~~ini
description=Expression relationship endpoints are resolved at publication
type=file
~~~
# SOURCE
~~~sysml
package M { part def System { part a; part b; connect a to b; } }
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "expression_relationship_publication.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package M { part def System { part a; part b; connect a to b; } }

~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "902842558f9a56e1a89b2cbf0b29dc936a624fd55f3d499914a5a0c64292fc9f") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "M"))) (kind "package") (name "M") (declared-name "M"))
    (element (id (node (document "d0") (qualified-name "M::System"))) (kind "part def") (name "System") (declared-name "System") (parent (node (document "d0") (qualified-name "M"))))
    (element (id (node (document "d0") (qualified-name "M::System::a"))) (kind "part") (name "a") (declared-name "a") (parent (node (document "d0") (qualified-name "M::System"))))
    (element (id (node (document "d0") (qualified-name "M::System::b"))) (kind "part") (name "b") (declared-name "b") (parent (node (document "d0") (qualified-name "M::System"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "M::System"))) (kind connectionSource) (ordinal 0)) (authored-target "a") (outcome (status resolved) (target (node (document "d0") (qualified-name "M::System::a")))))
    (reference (id (source (node (document "d0") (qualified-name "M::System"))) (kind connectionTarget) (ordinal 0)) (authored-target "b") (outcome (status resolved) (target (node (document "d0") (qualified-name "M::System::b")))))
  )
  (relationships
    (relationship (kind connection) (source (node (document "d0") (qualified-name "M::System::a"))) (target (node (document "d0") (qualified-name "M::System::b"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "M::System"))) (kind connectionSource) (ordinal 0)) (expression (kind connection) (source "a") (target "b")))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 0 54) (end 0 55)) (probe (position 0 54))
      (reference
        (source (document "d0") (qualified-name "M::System"))
        (kind connectionSource) (ordinal 0) (authored-target "a")
        (range (start 0 54) (end 0 55))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "M::System::a") (range (start 0 30) (end 0 37)))
        )
      )
    )
    (query (range (start 0 59) (end 0 60)) (probe (position 0 59))
      (reference
        (source (document "d0") (qualified-name "M::System"))
        (kind connectionTarget) (ordinal 0) (authored-target "b")
        (range (start 0 59) (end 0 60))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "M::System::b") (range (start 0 38) (end 0 45)))
        )
      )
    )
  )
)
~~~
