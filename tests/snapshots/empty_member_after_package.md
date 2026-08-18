# META
~~~ini
description=Empty member (bare semicolon) at file level after package
type=file
~~~
# SOURCE
~~~sysml
package MyPkg { }; in newX : Real;
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/empty_member_after_package.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "expected_keyword")
        (source "parser")
        (range (start 0 17) (end 0 34))
      )
      (diagnostic
        (severity error)
        (code "expected_keyword")
        (source "parser")
        (range (start 0 19) (end 0 34))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:a6a2ea2079b4d1fb5fc110bd88f052231a5b1c0115565e9aacdb373bf82083c1") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/empty_member_after_package.md") (qualified-name "MyPkg"))) (kind package) (membership (kind owning) (visibility default)))
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
