# META
~~~ini
description=KerML 8.3.4.11.2 checkMultiplicityRangeExpressionTypeFeaturing requires each multiplicity range bound expression to share the range featuringTypes
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.11.2:checkMultiplicityRangeExpressionTypeFeaturing
blocked_by=lowering-gap-type-featuring-multiplicity-range-bounds
type=file
~~~
# SOURCE
~~~kerml
package Ranges {
    classifier Vehicle {
        feature mass [1..2];
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship
    (kind type_featuring)
    (source "Ranges::Vehicle::mass::multiplicityRange::lowerBound")
    (target "Ranges::Vehicle")
    (provenance implied)
    (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:3f4c13a00b116c10d893dbf60f108174e594704fcd98615bae160a9532a7f692"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (qualified-name "Ranges"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (qualified-name "Ranges::Vehicle"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (qualified-name "Ranges::Vehicle::mass"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 2))))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (qualified-name "Ranges::Vehicle::mass"))) (target (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (qualified-name "Ranges::Vehicle"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (qualified-name "Ranges::Vehicle::mass")))
      (featured-by (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (qualified-name "Ranges::Vehicle")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
