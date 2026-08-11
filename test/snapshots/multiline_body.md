# META
~~~ini
description=Multiline body with newline insertion
type=file
~~~
# SOURCE
~~~sysml
package Foo {
  feature x;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "multiline_body.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package Foo {
    feature x;
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "08231c07e2fac8d74bca78e76135339ba11439d9060a1ce612535aeceaf510e7") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Foo"))) (kind "package") (name "Foo") (declared-name "Foo"))
    (element (id (node (document "d0") (qualified-name "Foo::x"))) (kind "feature decl") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "Foo"))))
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
