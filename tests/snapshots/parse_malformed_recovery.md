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
  (document "memory://snapshot/parse_malformed_recovery.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_package_body_element")
        (source "parser")
        (range (start 1 4) (end 2 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:f491eea378f800426817629bbafa921ea3e956f72e6489a11a5fe34827bff600") (contract-version "feature-value-expression-results-v5"))
  (declarations
    (declaration (id (node (document "memory://snapshot/parse_malformed_recovery.md") (qualified-name "Foo"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parse_malformed_recovery.md") (qualified-name "Foo::Bar"))) (kind part-def) (membership (kind owning) (visibility default)))
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
