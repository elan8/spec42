# META
~~~ini
description=KerML 8.3.4.11.2 validateMultiplicityRangeBoundResultTypes also rejects a bound whose result is not an Integer, such as a real literal
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.11.2 validateMultiplicityRangeBoundResultTypes
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.4.11.2:validateMultiplicityRangeBoundResultTypes
coverage_role=secondary
blocked_by=semantic-multiplicity-bound-result-type
type=file
~~~
# SOURCE
~~~kerml
package Multiplicities {
    // Conforming: integer bounds.
    classifier Bounded[0..3];

    // Invalid: a real-valued bound.
    classifier Fractional[1.5];
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_multiplicity_range_bound_non_integer.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "multiplicity_bound_invalid")
        (source "semantic")
        (range (start 5 25) (end 5 30))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_multiplicity_range_bound_non_integer.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:b64b0173988802a2885113d2a6420dff3a5dfe734733cbd38487a400e8ff6886"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_bound_non_integer.md") (qualified-name "Multiplicities"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_bound_non_integer.md") (qualified-name "Multiplicities::Bounded"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (facts (multiplicity (lower 0) (upper 3))))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_bound_non_integer.md") (qualified-name "Multiplicities::Fractional"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (facts (multiplicity (lower expression) (upper expression))))
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
