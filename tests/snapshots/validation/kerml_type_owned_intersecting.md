# META
~~~ini
description=KerML deriveTypeOwnedIntersecting projects authored intersecting relationships
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=kerml-1.0:8.3.3.1.10:deriveTypeOwnedIntersecting
libraries=none
~~~
# SOURCE
~~~kerml
package Model { classifier Base; classifier Derived intersects Base; }
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (type-derived-relationship-collection
    (rule_id "kerml-1.0:8.3.3.1.10:deriveTypeOwnedIntersecting")
    (source "Model::Derived")
    (kind intersecting)
    (target "Model::Base")
    (provenance authored)
    (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_type_owned_intersecting.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "single_type_relationship_operand")
        (source "semantic")
        (range (start 0 63) (end 0 67))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:3ee9eca61a09e3635767c77adf61fa1eb8fc486d555fcf9d76da471c1be9421b") (contract-version "owned-cross-feature-typing-v4"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_intersecting.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_intersecting.md") (qualified-name "Model::Base"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_intersecting.md") (qualified-name "Model::Derived"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (intersecting (reference "Base")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_type_owned_intersecting.md") (qualified-name "Model::Derived"))) (kind intersecting) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_owned_intersecting.md") (qualified-name "Model::Base")))))
  )
  (relationships
    (relationship (kind intersecting) (source (node (document "memory://snapshot/kerml_type_owned_intersecting.md") (qualified-name "Model::Derived"))) (target (node (document "memory://snapshot/kerml_type_owned_intersecting.md") (qualified-name "Model::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_type_owned_intersecting.md") (qualified-name "Model::Derived"))) (kind intersecting) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_intersecting.md") (qualified-name "Model::Derived")))
      (set-operand (operator intersection) (ordinal 0) (node (document "memory://snapshot/kerml_type_owned_intersecting.md") (qualified-name "Model::Base")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_type_owned_intersecting.md") (range (start 0 63) (end 0 67)) (probe (position 0 63))
    (reference (id (source (node (document "memory://snapshot/kerml_type_owned_intersecting.md") (qualified-name "Model::Derived"))) (kind intersecting) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_owned_intersecting.md") (qualified-name "Model::Base")))))
    )
  )
)
~~~
