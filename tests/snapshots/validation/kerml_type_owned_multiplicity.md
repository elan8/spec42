# META
~~~ini
description=KerML 8.3.3.1.10 validateTypeOwnedMultiplicity allows a Type at most one ownedMember that is a Multiplicity
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.1.10 validateTypeOwnedMultiplicity
source_expectation=accepted
rule_family=validate
expectation=by_construction
rule_id=kerml-1.0:8.3.3.1.10:validateTypeOwnedMultiplicity
blocked_by=abstract-syntax-nonrepresentable-multiplicity
type=file
~~~
# SOURCE
~~~kerml
package Multiplicities {
    // Conforming: a single owned multiplicity.
    classifier One[1];
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_type_owned_multiplicity.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:71bf4d9e5c3b913e351b59f565c29c56f34076103db3c3b09fa883ec35f5a381") (contract-version "feature-value-expression-results-v5"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (qualified-name "Multiplicities"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (qualified-name "Multiplicities::One"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (facts (multiplicity (lower 1) (upper 1))))
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
