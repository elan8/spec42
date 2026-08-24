# META
~~~ini
description=KerML 8.3.2.4.5 validateNamespaceDistinguishibility requires all memberships of a Namespace to be distinguishable from each other
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.2.4.5 validateNamespaceDistinguishibility
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.2.4.5:validateNamespaceDistinguishibility
blocked_by=semantic-duplicate-namespace-member
type=file
~~~
# SOURCE
~~~kerml
// Conforming: every membership of Distinct carries a different member name.
package Distinct {
    classifier Thing;
    classifier Gadget;
}

// Invalid: two memberships of Colliding share the member name Thing.
package Colliding {
    classifier Thing;
    datatype Thing;
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_namespace_distinguishability.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "duplicate_namespace_member")
        (source "semantic")
        (range (start 9 4) (end 9 19))
        (related-information
          (related
            (uri "memory://snapshot/kerml_namespace_distinguishability.md")
            (range (start 8 4) (end 8 21))
          )
        )
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_namespace_distinguishability.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:b44d891fc3df360d2df938c0f54e92081844d1defe980240dd9dead5035b6eb7") (contract-version "lossless-publication-completeness-v3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_namespace_distinguishability.md") (qualified-name "Colliding"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_namespace_distinguishability.md") (path (named (kind package) (name "Colliding")) (named (kind kerml-classifier) (name "Thing"))))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_namespace_distinguishability.md") (path (named (kind package) (name "Colliding")) (named (kind kerml-datatype) (name "Thing"))))) (kind kerml-datatype) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_namespace_distinguishability.md") (qualified-name "Distinct"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_namespace_distinguishability.md") (qualified-name "Distinct::Gadget"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_namespace_distinguishability.md") (qualified-name "Distinct::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
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
