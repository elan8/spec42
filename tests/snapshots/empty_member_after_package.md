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
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 0 29) (end 0 33))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:a6a2ea2079b4d1fb5fc110bd88f052231a5b1c0115565e9aacdb373bf82083c1") (contract-version "constructor-expression-specialization-v9"))
  (declarations
    (declaration (id (node (document "memory://snapshot/empty_member_after_package.md") (path (anonymous (kind default-reference) (ordinal 0))))) (kind default-reference) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/empty_member_after_package.md") (qualified-name "MyPkg"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/empty_member_after_package.md") (qualified-name "newX"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real") (direction in)))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/empty_member_after_package.md") (qualified-name "newX"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
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
  (query (document "memory://snapshot/empty_member_after_package.md") (range (start 0 29) (end 0 33)) (probe (position 0 29))
    (reference (id (source (node (document "memory://snapshot/empty_member_after_package.md") (qualified-name "newX"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
    )
  )
)
~~~
