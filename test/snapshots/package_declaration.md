# META
~~~ini
description=Simple package declaration
type=file
~~~
# SOURCE
~~~sysml
package MyPkg { }
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "package_declaration.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package MyPkg { }

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "f84a071683f8cf24c30ffc95505415422281b7775c79fbffc7de4602752052b0") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "MyPkg"))) (kind "package") (name "MyPkg") (declared-name "MyPkg"))
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
