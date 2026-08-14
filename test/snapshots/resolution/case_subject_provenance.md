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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:5e7ca5133ba92cef73efb066c60f959921920855d122a4765f27bdcfa636968b") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/case_subject_provenance.md") (qualified-name "M"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/case_subject_provenance.md") (qualified-name "M::A"))) (kind analysis-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/case_subject_provenance.md") (qualified-name "M::A::s"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "P"))))
    (declaration (id (node (document "memory://snapshot/case_subject_provenance.md") (qualified-name "M::P"))) (kind part-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/case_subject_provenance.md") (qualified-name "M::A::s"))) (kind featureTyping) (ordinal 0))
      (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/case_subject_provenance.md") (qualified-name "M::P")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/case_subject_provenance.md") (qualified-name "M::A::s"))) (target (node (document "memory://snapshot/case_subject_provenance.md") (qualified-name "M::P"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/case_subject_provenance.md") (qualified-name "M::A::s"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/case_subject_provenance.md") (qualified-name "M::A::s")))
      (supertype (node (document "memory://snapshot/case_subject_provenance.md") (qualified-name "M::P")) (scopes any))
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
~~~
