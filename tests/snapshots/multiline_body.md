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
  (document "memory://snapshot/multiline_body.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:93b897f9dc17771b5ba4213d70ed39228d0a0e1a9ae8512914ee197093e6a250") (contract-version "semantic-metadata-projection-v6"))
  (declarations
    (declaration (id (node (document "memory://snapshot/multiline_body.md") (qualified-name "Foo"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/multiline_body.md") (qualified-name "Foo::x"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
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
