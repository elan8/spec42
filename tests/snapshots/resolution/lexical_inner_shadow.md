# META
~~~ini
description=Lexical inner binding shadows an incompatible imported binding
type=file
~~~
# SOURCE
~~~sysml
package A {
    part def T;
}
package C {
    import A::*;
    part T;
    part p : T;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/lexical_inner_shadow.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 5 4) (end 5 11))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:f4f78350c7cde18e37c5cc680608d1513846fa38ba55c1115fdbcb368c96230f"))
  (declarations
    (declaration (id (node (document "memory://snapshot/lexical_inner_shadow.md") (qualified-name "A"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/lexical_inner_shadow.md") (qualified-name "A::T"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/lexical_inner_shadow.md") (qualified-name "C"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/lexical_inner_shadow.md") (path (named (kind package) (name "C")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility default)) (authored (membership (kind import) (visibility default)) (relationships (namespaceImport (reference "A") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/lexical_inner_shadow.md") (qualified-name "C::T"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/lexical_inner_shadow.md") (qualified-name "C::p"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "T")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/lexical_inner_shadow.md") (path (named (kind package) (name "C")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/lexical_inner_shadow.md") (qualified-name "A")))))
    (reference (id (source (node (document "memory://snapshot/lexical_inner_shadow.md") (qualified-name "C::p"))) (kind featureTyping) (ordinal 0))
      (authored-target "T")
      (outcome (status resolved) (target (node (document "memory://snapshot/lexical_inner_shadow.md") (qualified-name "C::T")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/lexical_inner_shadow.md") (qualified-name "C::p"))) (target (node (document "memory://snapshot/lexical_inner_shadow.md") (qualified-name "C::T"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/lexical_inner_shadow.md") (qualified-name "C::p"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/lexical_inner_shadow.md") (qualified-name "C::T")))
      (subtype (node (document "memory://snapshot/lexical_inner_shadow.md") (qualified-name "C::p")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/lexical_inner_shadow.md") (qualified-name "C::p")))
      (type (node (document "memory://snapshot/lexical_inner_shadow.md") (qualified-name "C::T")) (provenance authored))
      (effective-type (node (document "memory://snapshot/lexical_inner_shadow.md") (qualified-name "C::T")) (source direct))
      (supertype (node (document "memory://snapshot/lexical_inner_shadow.md") (qualified-name "C::T")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/lexical_inner_shadow.md") (range (start 4 11) (end 4 15)) (probe (position 4 11))
    (reference (id (source (node (document "memory://snapshot/lexical_inner_shadow.md") (path (named (kind package) (name "C")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/lexical_inner_shadow.md") (qualified-name "A")))))
    )
  )
  (query (document "memory://snapshot/lexical_inner_shadow.md") (range (start 6 13) (end 6 14)) (probe (position 6 13))
    (reference (id (source (node (document "memory://snapshot/lexical_inner_shadow.md") (qualified-name "C::p"))) (kind featureTyping) (ordinal 0) (authored-target "T")
      (outcome (status resolved) (target (node (document "memory://snapshot/lexical_inner_shadow.md") (qualified-name "C::T")))))
    )
  )
)
~~~
