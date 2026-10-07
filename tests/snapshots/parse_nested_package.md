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
  (document "memory://snapshot/parse_nested_package.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:37f20984c2f3dff72a480a48f29691e84df00b71eeaf2dd7376fa6653b1fbb21"))
  (declarations
    (declaration (id (node (document "memory://snapshot/parse_nested_package.md") (qualified-name "Outer"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parse_nested_package.md") (qualified-name "Outer::Inner"))) (kind package) (membership (kind owning) (visibility default)))
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
