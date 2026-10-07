# META
~~~ini
description=Case subject derived relationship retains explicit provenance
type=file
~~~
# SOURCE
~~~sysml
package M {
    part def P;
    analysis def A {
        subject s : P;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/case_subject_provenance.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:c640803ffe3405c64aef5ff27403278e568216d746d107bc54fc03260caaa724"))
  (declarations
    (declaration (id (node (document "memory://snapshot/case_subject_provenance.md") (qualified-name "M"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/case_subject_provenance.md") (qualified-name "M::A"))) (kind analysis-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/case_subject_provenance.md") (qualified-name "M::A::s"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "P")))))
    (declaration (id (node (document "memory://snapshot/case_subject_provenance.md") (qualified-name "M::P"))) (kind part-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/case_subject_provenance.md") (qualified-name "M::A::s"))) (kind featureTyping) (ordinal 0))
      (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/case_subject_provenance.md") (qualified-name "M::P")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/case_subject_provenance.md") (qualified-name "M::A::s"))) (target (node (document "memory://snapshot/case_subject_provenance.md") (qualified-name "M::P"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/case_subject_provenance.md") (qualified-name "M::A::s"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/case_subject_provenance.md") (qualified-name "M::A::s"))) (target (node (document "memory://snapshot/case_subject_provenance.md") (qualified-name "M::A"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/case_subject_provenance.md") (qualified-name "M::A::s")))
      (featured-by (node (document "memory://snapshot/case_subject_provenance.md") (qualified-name "M::A")))
      (type (node (document "memory://snapshot/case_subject_provenance.md") (qualified-name "M::P")) (provenance authored))
      (effective-type (node (document "memory://snapshot/case_subject_provenance.md") (qualified-name "M::P")) (source direct))
      (supertype (node (document "memory://snapshot/case_subject_provenance.md") (qualified-name "M::P")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/case_subject_provenance.md") (qualified-name "M::P")))
      (subtype (node (document "memory://snapshot/case_subject_provenance.md") (qualified-name "M::A::s")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/case_subject_provenance.md") (range (start 3 20) (end 3 21)) (probe (position 3 20))
    (reference (id (source (node (document "memory://snapshot/case_subject_provenance.md") (qualified-name "M::A::s"))) (kind featureTyping) (ordinal 0) (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/case_subject_provenance.md") (qualified-name "M::P")))))
    )
  )
)
~~~
