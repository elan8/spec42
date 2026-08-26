# META
~~~ini
description=KerML deriveTypeOwnedSpecialization projects authored specialization relationships
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=kerml-1.0:8.3.3.1.10:deriveTypeOwnedSpecialization
libraries=none
~~~
# SOURCE
~~~kerml
package Model { classifier Base; classifier Derived specializes Base; }
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (type-derived-relationship-collection
    (rule_id "kerml-1.0:8.3.3.1.10:deriveTypeOwnedSpecialization")
    (source "Model::Derived")
    (kind specialization)
    (target "Model::Base")
    (provenance authored)
    (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_type_owned_specialization.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:892e744c523895ed785b0743ca67dfcb4b78c98ea2f79f12f739ee396200bbfa") (contract-version "constructor-expression-result-v8"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_specialization.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_specialization.md") (qualified-name "Model::Base"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_specialization.md") (qualified-name "Model::Derived"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_type_owned_specialization.md") (qualified-name "Model::Derived"))) (kind specialization) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_owned_specialization.md") (qualified-name "Model::Base")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_type_owned_specialization.md") (qualified-name "Model::Derived"))) (target (node (document "memory://snapshot/kerml_type_owned_specialization.md") (qualified-name "Model::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_type_owned_specialization.md") (qualified-name "Model::Derived"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_specialization.md") (qualified-name "Model::Base")))
      (subtype (node (document "memory://snapshot/kerml_type_owned_specialization.md") (qualified-name "Model::Derived")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_specialization.md") (qualified-name "Model::Derived")))
      (supertype (node (document "memory://snapshot/kerml_type_owned_specialization.md") (qualified-name "Model::Base")) (scopes any subclassification))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_type_owned_specialization.md") (range (start 0 64) (end 0 68)) (probe (position 0 64))
    (reference (id (source (node (document "memory://snapshot/kerml_type_owned_specialization.md") (qualified-name "Model::Derived"))) (kind specialization) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_owned_specialization.md") (qualified-name "Model::Base")))))
    )
  )
)
~~~
