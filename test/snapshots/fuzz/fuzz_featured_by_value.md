# META
~~~ini
description=Fuzz: featured by must precede value assignment for idempotent reparse
type=file
~~~
# SOURCE
~~~sysml
package P {
    feature g featured by c = 42;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "fuzz_featured_by_value.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "46014a724ef343bf296ba5b7ed087fc7cba2f6d73f77defead2b625ef68bf369") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "P"))) (kind "package") (name "P") (declared-name "P"))
    (element (id (node (document "d0") (qualified-name "P::g"))) (kind "feature decl") (name "g") (declared-name "g") (parent (node (document "d0") (qualified-name "P"))))
  )
  (references
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
)
~~~
