# META
~~~ini
description=KerML 8.3.4.11.2 checkMultiplicityRangeExpressionTypeFeaturing requires each multiplicity range bound expression to share the range featuringTypes
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.11.2:checkMultiplicityRangeExpressionTypeFeaturing
type=file
~~~
# SOURCE
~~~kerml
package Ranges {
    classifier Vehicle {
        // Both bounds of the MultiplicityRange of `mass` are featured by `Vehicle`.
        feature mass [1..2];
    }
    // The bound of a Feature with no featuringType has none.
    feature unfeatured [3];
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship
    (kind type_featuring)
    (source (anonymous (owner (anonymous (owner "Ranges::Vehicle::mass") (kind MultiplicityRange) (ordinal 0))) (kind Expression) (ordinal 0)))
    (target "Ranges::Vehicle")
    (provenance implied)
    (outcome resolved))
  (relationship
    (kind type_featuring)
    (source (anonymous (owner (anonymous (owner "Ranges::Vehicle::mass") (kind MultiplicityRange) (ordinal 0))) (kind Expression) (ordinal 1)))
    (target "Ranges::Vehicle")
    (provenance implied)
    (outcome resolved))
  (relationship
    (kind type_featuring)
    (source (anonymous (owner (anonymous (owner "Ranges::unfeatured") (kind MultiplicityRange) (ordinal 0))) (kind Expression) (ordinal 0)))
    (provenance implied)
    (outcome absent)))
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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:5a9756f0684285178f04b2e1612b659e94622eb14cd910c5d79c5d02b414a296"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (qualified-name "Ranges"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (qualified-name "Ranges::Vehicle"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (qualified-name "Ranges::Vehicle::mass"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 2))))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (kind kerml-multiplicity-range) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (qualified-name "Ranges::unfeatured"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 3) (upper 3))))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-feature) (name "unfeatured")) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (kind kerml-multiplicity-range) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-feature) (name "unfeatured")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-feature) (name "unfeatured")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-feature) (name "unfeatured")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (qualified-name "Ranges::Vehicle::mass"))) (target (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (qualified-name "Ranges::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (qualified-name "Ranges::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (qualified-name "Ranges::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1))))) (target (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (qualified-name "Ranges::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-feature) (name "unfeatured")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-feature) (name "unfeatured")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
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
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (qualified-name "Ranges::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (qualified-name "Ranges::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (qualified-name "Ranges::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1)))))
    )
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-feature) (name "unfeatured")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_multiplicity_range_expression_type_featuring.md") (path (named (kind package) (name "Ranges")) (named (kind kerml-feature) (name "unfeatured")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
