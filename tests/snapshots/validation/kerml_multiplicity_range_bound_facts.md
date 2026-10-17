# META
~~~ini
description=A multiplicity range projects its lower bound, upper bound, and bound sequence from the expressions it owns
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=kerml-1.0:8.3.4.11.2:deriveMultiplicityRangeLowerBound
rule_id=kerml-1.0:8.3.4.11.2:deriveMultiplicityRangeUpperBound
rule_id=kerml-1.0:8.3.4.11.2:deriveMultiplicityRangeBound
libraries=none
~~~
# SOURCE
~~~kerml
package Ranges {
    classifier Vehicle {
        feature mass [1..2];
        feature one [1];
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (action-derived-fact
    (rule_id "kerml-1.0:8.3.4.11.2:deriveMultiplicityRangeLowerBound")
    (source (anonymous (owner "Ranges::Vehicle::mass") (kind MultiplicityRange) (ordinal 0)))
    (target (anonymous (owner (anonymous (owner "Ranges::Vehicle::mass") (kind MultiplicityRange) (ordinal 0))) (kind LiteralInteger) (ordinal 0)))
    (outcome resolved))
  (action-derived-fact
    (rule_id "kerml-1.0:8.3.4.11.2:deriveMultiplicityRangeUpperBound")
    (source (anonymous (owner "Ranges::Vehicle::mass") (kind MultiplicityRange) (ordinal 0)))
    (target (anonymous (owner (anonymous (owner "Ranges::Vehicle::mass") (kind MultiplicityRange) (ordinal 0))) (kind LiteralInteger) (ordinal 1)))
    (outcome resolved))
  (action-derived-fact
    (rule_id "kerml-1.0:8.3.4.11.2:deriveMultiplicityRangeBound")
    (source (anonymous (owner "Ranges::Vehicle::mass") (kind MultiplicityRange) (ordinal 0)))
    (target (anonymous (owner (anonymous (owner "Ranges::Vehicle::mass") (kind MultiplicityRange) (ordinal 0))) (kind LiteralInteger) (ordinal 0)))
    (outcome resolved))
  (action-derived-fact
    (rule_id "kerml-1.0:8.3.4.11.2:deriveMultiplicityRangeBound")
    (source (anonymous (owner "Ranges::Vehicle::mass") (kind MultiplicityRange) (ordinal 0)))
    (target (anonymous (owner (anonymous (owner "Ranges::Vehicle::mass") (kind MultiplicityRange) (ordinal 0))) (kind LiteralInteger) (ordinal 1)))
    (outcome resolved))
  (action-derived-fact
    (rule_id "kerml-1.0:8.3.4.11.2:deriveMultiplicityRangeLowerBound")
    (source (anonymous (owner "Ranges::Vehicle::one") (kind MultiplicityRange) (ordinal 0)))
    (outcome absent))
  (action-derived-fact
    (rule_id "kerml-1.0:8.3.4.11.2:deriveMultiplicityRangeUpperBound")
    (source (anonymous (owner "Ranges::Vehicle::one") (kind MultiplicityRange) (ordinal 0)))
    (target (anonymous (owner (anonymous (owner "Ranges::Vehicle::one") (kind MultiplicityRange) (ordinal 0))) (kind LiteralInteger) (ordinal 0)))
    (outcome resolved))
  (action-derived-fact
    (rule_id "kerml-1.0:8.3.4.11.2:deriveMultiplicityRangeBound")
    (source (anonymous (owner "Ranges::Vehicle::one") (kind MultiplicityRange) (ordinal 0)))
    (target (anonymous (owner (anonymous (owner "Ranges::Vehicle::one") (kind MultiplicityRange) (ordinal 0))) (kind LiteralInteger) (ordinal 0)))
    (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:78649aea496e4d469ee061a7c3afe8ec1b69d5cbc964813e2c3cc1086e95a5e9"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (qualified-name "Ranges"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (qualified-name "Ranges::Vehicle"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (qualified-name "Ranges::Vehicle::mass"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 2))))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (kind kerml-multiplicity-range) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (qualified-name "Ranges::Vehicle::one"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "one")) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (kind kerml-multiplicity-range) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "one")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "one")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "one")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (qualified-name "Ranges::Vehicle::mass"))) (target (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (qualified-name "Ranges::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (qualified-name "Ranges::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (qualified-name "Ranges::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (target (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (qualified-name "Ranges::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (qualified-name "Ranges::Vehicle::one"))) (target (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (qualified-name "Ranges::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "one")) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (qualified-name "Ranges::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "one")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (qualified-name "Ranges::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "one")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "one")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (qualified-name "Ranges::Vehicle::mass")))
      (featured-by (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (qualified-name "Ranges::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (qualified-name "Ranges::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (qualified-name "Ranges::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (qualified-name "Ranges::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)))))
    )
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (qualified-name "Ranges::Vehicle::one")))
      (featured-by (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (qualified-name "Ranges::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "one")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (qualified-name "Ranges::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "one")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (qualified-name "Ranges::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "one")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_multiplicity_range_bound_facts.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "one")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
