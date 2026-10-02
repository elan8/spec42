# META
~~~ini
description=KerML 8.3.4.11.2 validateMultiplicityRangeBoundResultTypes requires the bound Expressions of a MultiplicityRange to be typed by ScalarValues::Integer and, when model-level evaluable, to evaluate to a non-negative value
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.11.2 validateMultiplicityRangeBoundResultTypes
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.4.11.2:validateMultiplicityRangeBoundResultTypes
type=file
~~~
# SOURCE
~~~kerml
package Multiplicities {
    // Conforming: non-negative integer bounds.
    classifier Bounded[0..3];

    // Invalid: a negative bound.
    classifier Negative[-1];
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_multiplicity_range_bound_result_types.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "multiplicity_bound_invalid")
        (source "semantic")
        (range (start 5 23) (end 5 27))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_multiplicity_range_bound_result_types.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "multiplicity_bound_invalid")
        (source "semantic")
        (range (start 5 23) (end 5 27))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:fde8adbe3e5a4f25a749c3913122bbb1fd62b800f522b522e75f64c56ce1d190"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_bound_result_types.md") (qualified-name "Multiplicities"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_bound_result_types.md") (qualified-name "Multiplicities::Bounded"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (facts (multiplicity (lower 0) (upper 3))))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_bound_result_types.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Bounded")) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (kind kerml-multiplicity-range) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_bound_result_types.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Bounded")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_multiplicity_range_bound_result_types.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Bounded")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_bound_result_types.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Bounded")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_multiplicity_range_bound_result_types.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Bounded")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_bound_result_types.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Bounded")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_bound_result_types.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Bounded")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_bound_result_types.md") (qualified-name "Multiplicities::Negative"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (facts (multiplicity (lower -1) (upper -1))))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_bound_result_types.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Negative")) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (kind kerml-multiplicity-range) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_bound_result_types.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Negative")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_multiplicity_range_bound_result_types.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Negative")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_bound_result_types.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Negative")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_multiplicity_range_bound_result_types.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Bounded")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_multiplicity_range_bound_result_types.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Bounded")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_multiplicity_range_bound_result_types.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Bounded")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_multiplicity_range_bound_result_types.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Bounded")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_multiplicity_range_bound_result_types.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Negative")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_multiplicity_range_bound_result_types.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Negative")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_bound_result_types.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Bounded")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_multiplicity_range_bound_result_types.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Bounded")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_bound_result_types.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Bounded")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_multiplicity_range_bound_result_types.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Bounded")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1)))))
    )
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_bound_result_types.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Negative")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_multiplicity_range_bound_result_types.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Negative")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
