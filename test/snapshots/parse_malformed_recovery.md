# META
~~~ini
description=Parser recovers from unexpected tokens
type=file
~~~
# SOURCE
~~~sysml
package Foo {
    + bad stuff;
    part def Bar;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "parse_malformed_recovery.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_package_body_element")
        (source "sysml")
        (range (start 1 4) (end 1 21))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package Foo {
    + bad stuff;
    part def Bar;
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "cda5a6f40388b019f8774c8e73b793d835738fe30bc045f5c3363ff67b9e21dd") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Foo"))) (kind "package") (name "Foo") (declared-name "Foo"))
    (element (id (node (document "d0") (qualified-name "Foo::Bar"))) (kind "part def") (name "Bar") (declared-name "Bar") (parent (node (document "d0") (qualified-name "Foo"))))
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
