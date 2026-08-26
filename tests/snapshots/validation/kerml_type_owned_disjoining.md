# META
~~~ini
description=KerML deriveTypeOwnedDisjoining projects authored disjoining relationships
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=kerml-1.0:8.3.3.1.10:deriveTypeOwnedDisjoining
libraries=none
~~~
# SOURCE
~~~kerml
package Model { classifier Base; classifier Derived disjoint from Base; }
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (type-derived-relationship-collection
    (rule_id "kerml-1.0:8.3.3.1.10:deriveTypeOwnedDisjoining")
    (source "Model::Derived")
    (kind disjoining)
    (target "Model::Base")
    (provenance authored)
    (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_type_owned_disjoining.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:8f8b5319ccaa17262a0490988fd7bc50ccc03a420d2a822a45ae029485cde074") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_disjoining.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_disjoining.md") (qualified-name "Model::Base"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_disjoining.md") (qualified-name "Model::Derived"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (disjoining (reference "Base")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_type_owned_disjoining.md") (qualified-name "Model::Derived"))) (kind disjoining) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_owned_disjoining.md") (qualified-name "Model::Base")))))
  )
  (relationships
    (relationship (kind disjoining) (source (node (document "memory://snapshot/kerml_type_owned_disjoining.md") (qualified-name "Model::Derived"))) (target (node (document "memory://snapshot/kerml_type_owned_disjoining.md") (qualified-name "Model::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_type_owned_disjoining.md") (qualified-name "Model::Derived"))) (kind disjoining) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_disjoining.md") (qualified-name "Model::Derived")))
      (set-operand (operator disjoint) (ordinal 0) (node (document "memory://snapshot/kerml_type_owned_disjoining.md") (qualified-name "Model::Base")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_type_owned_disjoining.md") (range (start 0 66) (end 0 70)) (probe (position 0 66))
    (reference (id (source (node (document "memory://snapshot/kerml_type_owned_disjoining.md") (qualified-name "Model::Derived"))) (kind disjoining) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_owned_disjoining.md") (qualified-name "Model::Base")))))
    )
  )
)
~~~
