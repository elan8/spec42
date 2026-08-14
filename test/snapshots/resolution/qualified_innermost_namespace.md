# META
~~~ini
description=Qualified segments resolve from the innermost namespace
type=file
~~~
# SOURCE
~~~sysml
package A {
    part def T;
}
package C {
    package A {
        part def T;
    }
    part p : A::T;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/qualified_innermost_namespace.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:2d824647a809458b422e075f80c32dc855b8f39cfc8a6345bf1d1f7a437168d3") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/qualified_innermost_namespace.md") (qualified-name "A"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/qualified_innermost_namespace.md") (qualified-name "A::T"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/qualified_innermost_namespace.md") (qualified-name "C"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/qualified_innermost_namespace.md") (qualified-name "C::A"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/qualified_innermost_namespace.md") (qualified-name "C::A::T"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/qualified_innermost_namespace.md") (qualified-name "C::p"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A::T"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/qualified_innermost_namespace.md") (qualified-name "C::p"))) (kind featureTyping) (ordinal 0))
      (authored-target "A::T")
      (outcome (status resolved) (target (node (document "memory://snapshot/qualified_innermost_namespace.md") (qualified-name "C::A::T")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/qualified_innermost_namespace.md") (qualified-name "C::p"))) (target (node (document "memory://snapshot/qualified_innermost_namespace.md") (qualified-name "C::A::T"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/qualified_innermost_namespace.md") (qualified-name "C::p"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/qualified_innermost_namespace.md") (qualified-name "C::p")))
      (supertype (node (document "memory://snapshot/qualified_innermost_namespace.md") (qualified-name "C::A::T")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/qualified_innermost_namespace.md") (range (start 7 13) (end 7 17)) (probe (position 7 13))
    (reference (id (source (node (document "memory://snapshot/qualified_innermost_namespace.md") (qualified-name "C::p"))) (kind featureTyping) (ordinal 0) (authored-target "A::T")
      (outcome (status resolved) (target (node (document "memory://snapshot/qualified_innermost_namespace.md") (qualified-name "C::A::T")))))
  )
)
~~~
