# META
~~~ini
description=Empty member (bare semicolon) inside package body
type=file
~~~
# SOURCE
~~~sysml
package MyPkg {;}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/empty_member_in_package.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:f1b53c1b427914a7063a3ccb23a1e86a7b9b362dc23ce358d41760b04bc52807"))
  (declarations
    (declaration (id (node (document "memory://snapshot/empty_member_in_package.md") (qualified-name "MyPkg"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/empty_member_in_package.md") (path (named (kind package) (name "MyPkg")) (anonymous (kind default-reference) (ordinal 0))))) (kind default-reference) (membership (kind feature) (visibility default)))
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
