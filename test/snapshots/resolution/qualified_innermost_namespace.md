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
  (document "qualified_innermost_namespace.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "5c7b706d304eaadd49f72a9a58289c60db6ee9b307666cfc827e564c544905b2") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "A"))) (kind "package") (name "A") (declared-name "A"))
    (element (id (node (document "d0") (qualified-name "A::T"))) (kind "part def") (name "T") (declared-name "T") (parent (node (document "d0") (qualified-name "A"))))
    (element (id (node (document "d0") (qualified-name "C"))) (kind "package") (name "C") (declared-name "C"))
    (element (id (node (document "d0") (qualified-name "C::A"))) (kind "package") (name "A") (declared-name "A") (parent (node (document "d0") (qualified-name "C"))))
    (element (id (node (document "d0") (qualified-name "C::A::T"))) (kind "part def") (name "T") (declared-name "T") (parent (node (document "d0") (qualified-name "C::A"))))
    (element (id (node (document "d0") (qualified-name "C::p"))) (kind "part") (name "p") (declared-name "p") (parent (node (document "d0") (qualified-name "C"))) (authored (membership (kind Feature)) (relationships (typing (reference "A::T")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "C::p"))) (kind featureTyping) (ordinal 0)) (authored-target "A::T") (outcome (status resolved) (target (node (document "d0") (qualified-name "C::A::T")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "C::p"))) (target (node (document "d0") (qualified-name "C::A::T"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "C::p"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 7 13) (end 7 17)) (probe (position 7 13))
      (reference
        (source (document "d0") (qualified-name "C::p"))
        (kind featureTyping) (ordinal 0) (authored-target "A::T")
        (range (start 7 13) (end 7 17))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "C::A::T") (range (start 5 8) (end 5 19)))
        )
      )
    )
  )
)
~~~
