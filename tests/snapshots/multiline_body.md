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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:8cc0f15b79d896b160639f0f9b880ba7dcd9ffc5d359396ca73b2ae0a8557213"))
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
