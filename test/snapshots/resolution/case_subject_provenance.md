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
  (document "case_subject_provenance.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package M {
    part def P;
    analysis def A {
        subject s : P;
    }
}

~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "93da094d06d9352c461dc6bd2768302ab81c25a7876613214d1c74fda083ed50") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "M"))) (kind "package") (name "M") (declared-name "M"))
    (element (id (node (document "d0") (qualified-name "M::A"))) (kind "analysis def") (name "A") (declared-name "A") (parent (node (document "d0") (qualified-name "M"))))
    (element (id (node (document "d0") (qualified-name "M::A::s"))) (kind "subject") (name "s") (declared-name "s") (parent (node (document "d0") (qualified-name "M::A"))) (authored (relationships (typing (reference "P")))))
    (element (id (node (document "d0") (qualified-name "M::P"))) (kind "part def") (name "P") (declared-name "P") (parent (node (document "d0") (qualified-name "M"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "M::A::s"))) (kind featureTyping) (ordinal 0)) (authored-target "P") (outcome (status resolved) (target (node (document "d0") (qualified-name "M::P")))))
  )
  (relationships
    (relationship (kind subject) (source (node (document "d0") (qualified-name "M::A"))) (target (node (document "d0") (qualified-name "M::P"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "M::A::s"))) (target (node (document "d0") (qualified-name "M::P"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "M::A::s"))) (kind featureTyping) (ordinal 0)))
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
