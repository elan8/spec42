# META
~~~ini
description=KerML 8.3.3.1.10 validateTypeOwnedIntersectingNotOne forbids a Type from owning exactly one Intersecting
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.1.10 validateTypeOwnedIntersectingNotOne
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.3.1.10:validateTypeOwnedIntersectingNotOne
type=file
~~~
# SOURCE
~~~kerml
package Intersections {
    classifier A;
    classifier B;

    // Conforming: two intersecting operands.
    classifier Pair intersects A, B;

    // Invalid: exactly one intersecting operand.
    classifier Single intersects A;
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_type_single_intersecting.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "single_type_relationship_operand")
        (source "semantic")
        (range (start 8 33) (end 8 34))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_type_single_intersecting.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "single_type_relationship_operand")
        (source "semantic")
        (range (start 8 33) (end 8 34))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:efba6b3e6f8330ee1de32b1b864fb26bbdf24e16c5c7fedc07ca0b023fa57cb3") (contract-version "feature-value-expression-results-v5"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_type_single_intersecting.md") (qualified-name "Intersections"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_single_intersecting.md") (qualified-name "Intersections::A"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_single_intersecting.md") (qualified-name "Intersections::B"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_single_intersecting.md") (qualified-name "Intersections::Pair"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (intersecting (reference "A")) (intersecting (reference "B")))))
    (declaration (id (node (document "memory://snapshot/kerml_type_single_intersecting.md") (qualified-name "Intersections::Single"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (intersecting (reference "A")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_type_single_intersecting.md") (qualified-name "Intersections::Pair"))) (kind intersecting) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_single_intersecting.md") (qualified-name "Intersections::A")))))
    (reference (id (source (node (document "memory://snapshot/kerml_type_single_intersecting.md") (qualified-name "Intersections::Pair"))) (kind intersecting) (ordinal 1))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_single_intersecting.md") (qualified-name "Intersections::B")))))
    (reference (id (source (node (document "memory://snapshot/kerml_type_single_intersecting.md") (qualified-name "Intersections::Single"))) (kind intersecting) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_single_intersecting.md") (qualified-name "Intersections::A")))))
  )
  (relationships
    (relationship (kind intersecting) (source (node (document "memory://snapshot/kerml_type_single_intersecting.md") (qualified-name "Intersections::Pair"))) (target (node (document "memory://snapshot/kerml_type_single_intersecting.md") (qualified-name "Intersections::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_type_single_intersecting.md") (qualified-name "Intersections::Pair"))) (kind intersecting) (ordinal 0)))
    (relationship (kind intersecting) (source (node (document "memory://snapshot/kerml_type_single_intersecting.md") (qualified-name "Intersections::Pair"))) (target (node (document "memory://snapshot/kerml_type_single_intersecting.md") (qualified-name "Intersections::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_type_single_intersecting.md") (qualified-name "Intersections::Pair"))) (kind intersecting) (ordinal 1)))
    (relationship (kind intersecting) (source (node (document "memory://snapshot/kerml_type_single_intersecting.md") (qualified-name "Intersections::Single"))) (target (node (document "memory://snapshot/kerml_type_single_intersecting.md") (qualified-name "Intersections::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_type_single_intersecting.md") (qualified-name "Intersections::Single"))) (kind intersecting) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_type_single_intersecting.md") (qualified-name "Intersections::Pair")))
      (set-operand (operator intersection) (ordinal 0) (node (document "memory://snapshot/kerml_type_single_intersecting.md") (qualified-name "Intersections::A")))
      (set-operand (operator intersection) (ordinal 1) (node (document "memory://snapshot/kerml_type_single_intersecting.md") (qualified-name "Intersections::B")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_type_single_intersecting.md") (qualified-name "Intersections::Single")))
      (set-operand (operator intersection) (ordinal 0) (node (document "memory://snapshot/kerml_type_single_intersecting.md") (qualified-name "Intersections::A")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_type_single_intersecting.md") (range (start 5 31) (end 5 32)) (probe (position 5 31))
    (reference (id (source (node (document "memory://snapshot/kerml_type_single_intersecting.md") (qualified-name "Intersections::Pair"))) (kind intersecting) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_single_intersecting.md") (qualified-name "Intersections::A")))))
    )
  )
  (query (document "memory://snapshot/kerml_type_single_intersecting.md") (range (start 5 34) (end 5 35)) (probe (position 5 34))
    (reference (id (source (node (document "memory://snapshot/kerml_type_single_intersecting.md") (qualified-name "Intersections::Pair"))) (kind intersecting) (ordinal 1) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_single_intersecting.md") (qualified-name "Intersections::B")))))
    )
  )
  (query (document "memory://snapshot/kerml_type_single_intersecting.md") (range (start 8 33) (end 8 34)) (probe (position 8 33))
    (reference (id (source (node (document "memory://snapshot/kerml_type_single_intersecting.md") (qualified-name "Intersections::Single"))) (kind intersecting) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_single_intersecting.md") (qualified-name "Intersections::A")))))
    )
  )
)
~~~
