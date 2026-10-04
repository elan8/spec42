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
  (document "memory://snapshot/package_declaration.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:bd2bf77a4d11071910da299af2ab8ab986bdeabb1cde946384793f8094d24f40"))
  (declarations
    (declaration (id (node (document "memory://snapshot/package_declaration.md") (qualified-name "MyPkg"))) (kind package) (membership (kind owning) (visibility default)))
  )
  (references
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
)
~~~
