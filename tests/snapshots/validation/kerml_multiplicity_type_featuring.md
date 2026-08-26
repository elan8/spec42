# META
~~~ini
description=KerML 8.3.3.1.9 checkMultiplicityTypeFeaturing requires a Multiplicity to inherit its owning Feature featuringTypes, or have none outside a Feature
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.3.1.9:checkMultiplicityTypeFeaturing
blocked_by=lowering-gap-type-featuring-multiplicity-owner
type=file
~~~
# SOURCE
~~~kerml
package Multiplicities {
    classifier Vehicle {
        feature mass [1];
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship
    (kind type_featuring)
    (source "Multiplicities::Vehicle::mass::multiplicity")
    (target "Multiplicities::Vehicle")
    (provenance implied)
    (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_multiplicity_type_featuring.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:34ad2311e7dcb165470149f704a89fb234bac361fad873410120a6d4b4c1cba0") (contract-version "constructor-expression-result-v8"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (qualified-name "Multiplicities"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (qualified-name "Multiplicities::Vehicle"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (qualified-name "Multiplicities::Vehicle::mass"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (qualified-name "Multiplicities::Vehicle::mass"))) (target (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (qualified-name "Multiplicities::Vehicle"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (qualified-name "Multiplicities::Vehicle::mass")))
      (featured-by (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (qualified-name "Multiplicities::Vehicle")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
