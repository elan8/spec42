# META
~~~ini
description=KerML 8.3.4.11.2 validateMultiplicityRangeBounds requires the lowerBound and upperBound Expressions to be the first ownedMembers of a MultiplicityRange
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.11.2 validateMultiplicityRangeBounds
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.4.11.2:validateMultiplicityRangeBounds
type=file
~~~
# SOURCE
~~~kerml
// Conforming: the bound expressions of each multiplicity range below are exactly the members the
// range declaration authors, in lower-then-upper order.
//
// The violating side has no textual counterpart: KerML multiplicity syntax admits only the bound
// expressions themselves inside the brackets and fixes their order, so a source document cannot
// author a MultiplicityRange whose bounds are not its first owned members.
package Multiplicities {
    classifier Exact[1];
    classifier Ranged[0..3];
    classifier Unbounded[0..*];
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_multiplicity_range_bounds.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_multiplicity_range_bounds.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:21bd4d564146a0e82879a897fe3e8b6c2348996e72bf3d64023a2769332fb834") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_bounds.md") (qualified-name "Multiplicities"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_bounds.md") (qualified-name "Multiplicities::Exact"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (facts (multiplicity (lower 1) (upper 1))))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_bounds.md") (qualified-name "Multiplicities::Ranged"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (facts (multiplicity (lower 0) (upper 3))))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_range_bounds.md") (qualified-name "Multiplicities::Unbounded"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (facts (multiplicity (lower 0) (upper unbounded))))
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
