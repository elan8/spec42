# META
~~~ini
description=KerML 8.3.3.1.10 validateTypeIntersectingTypesNotSelf forbids a Type from being one of its own intersectingTypes
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.1.10 validateTypeIntersectingTypesNotSelf
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.3.1.10:validateTypeIntersectingTypesNotSelf
blocked_by=semantic-type-relationship-operand-is-self
type=file
~~~
# SOURCE
~~~kerml
package Intersections {
    classifier A;
    classifier B;

    // Conforming: neither intersecting operand is the intersecting type itself.
    classifier Good intersects A, B;

    // Invalid: the type is one of its own intersecting operands.
    classifier Bad intersects A, Bad;
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_type_intersecting_self.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "type_relationship_operand_is_self")
        (source "semantic")
        (range (start 8 33) (end 8 36))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_type_intersecting_self.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:7920f1b64ca3b9a539dbe11a91d5782c4bc8fba1a0bc6a535b9c00184a292d1c") (contract-version "feature-value-expression-results-v5"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_type_intersecting_self.md") (qualified-name "Intersections"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_intersecting_self.md") (qualified-name "Intersections::A"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_intersecting_self.md") (qualified-name "Intersections::B"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_intersecting_self.md") (qualified-name "Intersections::Bad"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (intersecting (reference "A")) (intersecting (reference "Bad")))))
    (declaration (id (node (document "memory://snapshot/kerml_type_intersecting_self.md") (qualified-name "Intersections::Good"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (intersecting (reference "A")) (intersecting (reference "B")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_type_intersecting_self.md") (qualified-name "Intersections::Bad"))) (kind intersecting) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_intersecting_self.md") (qualified-name "Intersections::A")))))
    (reference (id (source (node (document "memory://snapshot/kerml_type_intersecting_self.md") (qualified-name "Intersections::Bad"))) (kind intersecting) (ordinal 1))
      (authored-target "Bad")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_intersecting_self.md") (qualified-name "Intersections::Bad")))))
    (reference (id (source (node (document "memory://snapshot/kerml_type_intersecting_self.md") (qualified-name "Intersections::Good"))) (kind intersecting) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_intersecting_self.md") (qualified-name "Intersections::A")))))
    (reference (id (source (node (document "memory://snapshot/kerml_type_intersecting_self.md") (qualified-name "Intersections::Good"))) (kind intersecting) (ordinal 1))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_intersecting_self.md") (qualified-name "Intersections::B")))))
  )
  (relationships
    (relationship (kind intersecting) (source (node (document "memory://snapshot/kerml_type_intersecting_self.md") (qualified-name "Intersections::Bad"))) (target (node (document "memory://snapshot/kerml_type_intersecting_self.md") (qualified-name "Intersections::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_type_intersecting_self.md") (qualified-name "Intersections::Bad"))) (kind intersecting) (ordinal 0)))
    (relationship (kind intersecting) (source (node (document "memory://snapshot/kerml_type_intersecting_self.md") (qualified-name "Intersections::Bad"))) (target (node (document "memory://snapshot/kerml_type_intersecting_self.md") (qualified-name "Intersections::Bad"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_type_intersecting_self.md") (qualified-name "Intersections::Bad"))) (kind intersecting) (ordinal 1)))
    (relationship (kind intersecting) (source (node (document "memory://snapshot/kerml_type_intersecting_self.md") (qualified-name "Intersections::Good"))) (target (node (document "memory://snapshot/kerml_type_intersecting_self.md") (qualified-name "Intersections::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_type_intersecting_self.md") (qualified-name "Intersections::Good"))) (kind intersecting) (ordinal 0)))
    (relationship (kind intersecting) (source (node (document "memory://snapshot/kerml_type_intersecting_self.md") (qualified-name "Intersections::Good"))) (target (node (document "memory://snapshot/kerml_type_intersecting_self.md") (qualified-name "Intersections::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_type_intersecting_self.md") (qualified-name "Intersections::Good"))) (kind intersecting) (ordinal 1)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_type_intersecting_self.md") (qualified-name "Intersections::Bad")))
      (set-operand (operator intersection) (ordinal 0) (node (document "memory://snapshot/kerml_type_intersecting_self.md") (qualified-name "Intersections::A")))
      (set-operand (operator intersection) (ordinal 1) (node (document "memory://snapshot/kerml_type_intersecting_self.md") (qualified-name "Intersections::Bad")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_type_intersecting_self.md") (qualified-name "Intersections::Good")))
      (set-operand (operator intersection) (ordinal 0) (node (document "memory://snapshot/kerml_type_intersecting_self.md") (qualified-name "Intersections::A")))
      (set-operand (operator intersection) (ordinal 1) (node (document "memory://snapshot/kerml_type_intersecting_self.md") (qualified-name "Intersections::B")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_type_intersecting_self.md") (range (start 8 30) (end 8 31)) (probe (position 8 30))
    (reference (id (source (node (document "memory://snapshot/kerml_type_intersecting_self.md") (qualified-name "Intersections::Bad"))) (kind intersecting) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_intersecting_self.md") (qualified-name "Intersections::A")))))
    )
  )
  (query (document "memory://snapshot/kerml_type_intersecting_self.md") (range (start 8 33) (end 8 36)) (probe (position 8 33))
    (reference (id (source (node (document "memory://snapshot/kerml_type_intersecting_self.md") (qualified-name "Intersections::Bad"))) (kind intersecting) (ordinal 1) (authored-target "Bad")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_intersecting_self.md") (qualified-name "Intersections::Bad")))))
    )
  )
  (query (document "memory://snapshot/kerml_type_intersecting_self.md") (range (start 5 31) (end 5 32)) (probe (position 5 31))
    (reference (id (source (node (document "memory://snapshot/kerml_type_intersecting_self.md") (qualified-name "Intersections::Good"))) (kind intersecting) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_intersecting_self.md") (qualified-name "Intersections::A")))))
    )
  )
  (query (document "memory://snapshot/kerml_type_intersecting_self.md") (range (start 5 34) (end 5 35)) (probe (position 5 34))
    (reference (id (source (node (document "memory://snapshot/kerml_type_intersecting_self.md") (qualified-name "Intersections::Good"))) (kind intersecting) (ordinal 1) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_intersecting_self.md") (qualified-name "Intersections::B")))))
    )
  )
)
~~~
