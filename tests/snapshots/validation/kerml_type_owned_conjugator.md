# META
~~~ini
description=KerML Type ownedConjugator retains the canonical Conjugation relationship and its original Type endpoint
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=kerml-1.0:8.3.3.1.10:deriveTypeOwnedConjugator
libraries=none
~~~
# SOURCE
~~~kerml
package Model {
  classifier Original;
  classifier Conjugated conjugates Original;
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (type-derived-fact
    (rule_id "kerml-1.0:8.3.3.1.10:deriveTypeOwnedConjugator")
    (source "Model::Conjugated")
    (target "Model::Original")
    (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_type_owned_conjugator.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:25de93c734923c384c41f5aafb4a24edfd6283590486bd925ed20652c397dbb8") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_conjugator.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_conjugator.md") (qualified-name "Model::Conjugated"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (conjugation (reference "Original")))))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_conjugator.md") (qualified-name "Model::Original"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_type_owned_conjugator.md") (qualified-name "Model::Conjugated"))) (kind conjugation) (ordinal 0))
      (authored-target "Original")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_owned_conjugator.md") (qualified-name "Model::Original")))))
  )
  (relationships
    (relationship (kind conjugation) (source (node (document "memory://snapshot/kerml_type_owned_conjugator.md") (qualified-name "Model::Conjugated"))) (target (node (document "memory://snapshot/kerml_type_owned_conjugator.md") (qualified-name "Model::Original"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_type_owned_conjugator.md") (qualified-name "Model::Conjugated"))) (kind conjugation) (ordinal 0)))
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
  (query (document "memory://snapshot/kerml_type_owned_conjugator.md") (range (start 2 35) (end 2 43)) (probe (position 2 35))
    (reference (id (source (node (document "memory://snapshot/kerml_type_owned_conjugator.md") (qualified-name "Model::Conjugated"))) (kind conjugation) (ordinal 0) (authored-target "Original")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_owned_conjugator.md") (qualified-name "Model::Original")))))
    )
  )
)
~~~
