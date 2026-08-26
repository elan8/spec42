# META
~~~ini
description=KerML deriveTypeOwnedDifferencing projects authored differencing relationships
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=kerml-1.0:8.3.3.1.10:deriveTypeOwnedDifferencing
libraries=none
~~~
# SOURCE
~~~kerml
package Model { classifier Base; classifier Derived differences Base; }
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (type-derived-relationship-collection
    (rule_id "kerml-1.0:8.3.3.1.10:deriveTypeOwnedDifferencing")
    (source "Model::Derived")
    (kind differencing)
    (target "Model::Base")
    (provenance authored)
    (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_type_owned_differencing.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "single_type_relationship_operand")
        (source "semantic")
        (range (start 0 64) (end 0 68))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:31fe2489f64fcf22f768589aa3f12d887573104a92bd3bd045e2ac819c3cfd04") (contract-version "feature-value-expression-results-v5"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_differencing.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_differencing.md") (qualified-name "Model::Base"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_differencing.md") (qualified-name "Model::Derived"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (differencing (reference "Base")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_type_owned_differencing.md") (qualified-name "Model::Derived"))) (kind differencing) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_owned_differencing.md") (qualified-name "Model::Base")))))
  )
  (relationships
    (relationship (kind differencing) (source (node (document "memory://snapshot/kerml_type_owned_differencing.md") (qualified-name "Model::Derived"))) (target (node (document "memory://snapshot/kerml_type_owned_differencing.md") (qualified-name "Model::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_type_owned_differencing.md") (qualified-name "Model::Derived"))) (kind differencing) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_differencing.md") (qualified-name "Model::Derived")))
      (set-operand (operator difference) (ordinal 0) (node (document "memory://snapshot/kerml_type_owned_differencing.md") (qualified-name "Model::Base")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_type_owned_differencing.md") (range (start 0 64) (end 0 68)) (probe (position 0 64))
    (reference (id (source (node (document "memory://snapshot/kerml_type_owned_differencing.md") (qualified-name "Model::Derived"))) (kind differencing) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_owned_differencing.md") (qualified-name "Model::Base")))))
    )
  )
)
~~~
