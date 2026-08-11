# META
~~~ini
description=Nested package definitions
type=file
~~~
# SOURCE
~~~sysml
package Outer {
    package Inner { }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "parse_nested_package.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package Outer {
    package Inner { }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "9701dd0051e52f095b6c9b0bb89f51de3c7395b617bc5907a5459dc917c29519") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Outer"))) (kind "package") (name "Outer") (declared-name "Outer"))
    (element (id (node (document "d0") (qualified-name "Outer::Inner"))) (kind "package") (name "Inner") (declared-name "Inner") (parent (node (document "d0") (qualified-name "Outer"))))
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
