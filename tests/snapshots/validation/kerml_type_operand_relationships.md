# META
~~~ini
description=KerML Type operand derivations retain the canonical authored relationship facts
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=kerml-1.0:8.3.3.1.10:deriveTypeUnioningType
rule_id=kerml-1.0:8.3.3.1.10:deriveTypeIntersectingType
rule_id=kerml-1.0:8.3.3.1.10:deriveTypeDifferencingType
libraries=none
~~~
# SOURCE
~~~kerml
package Model { classifier Base; classifier Derived unions Base intersects Base differences Base; }
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (type-derived-relationship-collection
    (rule_id "kerml-1.0:8.3.3.1.10:deriveTypeUnioningType")
    (source "Model::Derived")
    (kind unioning)
    (target "Model::Base")
    (provenance authored)
    (outcome resolved))
  (type-derived-relationship-collection
    (rule_id "kerml-1.0:8.3.3.1.10:deriveTypeIntersectingType")
    (source "Model::Derived")
    (kind intersecting)
    (target "Model::Base")
    (provenance authored)
    (outcome resolved))
  (type-derived-relationship-collection
    (rule_id "kerml-1.0:8.3.3.1.10:deriveTypeDifferencingType")
    (source "Model::Derived")
    (kind differencing)
    (target "Model::Base")
    (provenance authored)
    (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_type_operand_relationships.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "single_type_relationship_operand")
        (source "semantic")
        (range (start 0 59) (end 0 63))
      )
      (diagnostic
        (severity error)
        (code "single_type_relationship_operand")
        (source "semantic")
        (range (start 0 75) (end 0 79))
      )
      (diagnostic
        (severity error)
        (code "single_type_relationship_operand")
        (source "semantic")
        (range (start 0 92) (end 0 96))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:64041063b934b53a138177680427338406d5831f8f3e8b7589057a7d5e314bc5") (contract-version "owned-cross-feature-typing-v4"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_type_operand_relationships.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_operand_relationships.md") (qualified-name "Model::Base"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_operand_relationships.md") (qualified-name "Model::Derived"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (unioning (reference "Base")) (intersecting (reference "Base")) (differencing (reference "Base")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_type_operand_relationships.md") (qualified-name "Model::Derived"))) (kind unioning) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_operand_relationships.md") (qualified-name "Model::Base")))))
    (reference (id (source (node (document "memory://snapshot/kerml_type_operand_relationships.md") (qualified-name "Model::Derived"))) (kind intersecting) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_operand_relationships.md") (qualified-name "Model::Base")))))
    (reference (id (source (node (document "memory://snapshot/kerml_type_operand_relationships.md") (qualified-name "Model::Derived"))) (kind differencing) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_operand_relationships.md") (qualified-name "Model::Base")))))
  )
  (relationships
    (relationship (kind unioning) (source (node (document "memory://snapshot/kerml_type_operand_relationships.md") (qualified-name "Model::Derived"))) (target (node (document "memory://snapshot/kerml_type_operand_relationships.md") (qualified-name "Model::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_type_operand_relationships.md") (qualified-name "Model::Derived"))) (kind unioning) (ordinal 0)))
    (relationship (kind intersecting) (source (node (document "memory://snapshot/kerml_type_operand_relationships.md") (qualified-name "Model::Derived"))) (target (node (document "memory://snapshot/kerml_type_operand_relationships.md") (qualified-name "Model::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_type_operand_relationships.md") (qualified-name "Model::Derived"))) (kind intersecting) (ordinal 0)))
    (relationship (kind differencing) (source (node (document "memory://snapshot/kerml_type_operand_relationships.md") (qualified-name "Model::Derived"))) (target (node (document "memory://snapshot/kerml_type_operand_relationships.md") (qualified-name "Model::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_type_operand_relationships.md") (qualified-name "Model::Derived"))) (kind differencing) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_type_operand_relationships.md") (qualified-name "Model::Derived")))
      (set-operand (operator union) (ordinal 0) (node (document "memory://snapshot/kerml_type_operand_relationships.md") (qualified-name "Model::Base")))
      (set-operand (operator intersection) (ordinal 0) (node (document "memory://snapshot/kerml_type_operand_relationships.md") (qualified-name "Model::Base")))
      (set-operand (operator difference) (ordinal 0) (node (document "memory://snapshot/kerml_type_operand_relationships.md") (qualified-name "Model::Base")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_type_operand_relationships.md") (range (start 0 59) (end 0 63)) (probe (position 0 59))
    (reference (id (source (node (document "memory://snapshot/kerml_type_operand_relationships.md") (qualified-name "Model::Derived"))) (kind unioning) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_operand_relationships.md") (qualified-name "Model::Base")))))
    )
  )
  (query (document "memory://snapshot/kerml_type_operand_relationships.md") (range (start 0 75) (end 0 79)) (probe (position 0 75))
    (reference (id (source (node (document "memory://snapshot/kerml_type_operand_relationships.md") (qualified-name "Model::Derived"))) (kind intersecting) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_operand_relationships.md") (qualified-name "Model::Base")))))
    )
  )
  (query (document "memory://snapshot/kerml_type_operand_relationships.md") (range (start 0 92) (end 0 96)) (probe (position 0 92))
    (reference (id (source (node (document "memory://snapshot/kerml_type_operand_relationships.md") (qualified-name "Model::Derived"))) (kind differencing) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_operand_relationships.md") (qualified-name "Model::Base")))))
    )
  )
)
~~~
