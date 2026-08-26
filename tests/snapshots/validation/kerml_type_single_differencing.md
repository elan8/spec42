# META
~~~ini
description=KerML 8.3.3.1.10 validateTypeOwnedDifferencingNotOne forbids a Type from owning exactly one Differencing
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.1.10 validateTypeOwnedDifferencingNotOne
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.3.1.10:validateTypeOwnedDifferencingNotOne
type=file
~~~
# SOURCE
~~~kerml
package Differences {
    classifier A;
    classifier B;

    // Conforming: two differencing operands.
    classifier Pair differences A, B;

    // Invalid: exactly one differencing operand.
    classifier Single differences A;
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_type_single_differencing.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "single_type_relationship_operand")
        (source "semantic")
        (range (start 8 34) (end 8 35))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_type_single_differencing.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "single_type_relationship_operand")
        (source "semantic")
        (range (start 8 34) (end 8 35))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:e0d74bb4ffbae1b8f9f97085951b9c1df7d72116b7c0b009f2cd3f72260450dd") (contract-version "constructor-expression-result-v8"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_type_single_differencing.md") (qualified-name "Differences"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_single_differencing.md") (qualified-name "Differences::A"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_single_differencing.md") (qualified-name "Differences::B"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_single_differencing.md") (qualified-name "Differences::Pair"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (differencing (reference "A")) (differencing (reference "B")))))
    (declaration (id (node (document "memory://snapshot/kerml_type_single_differencing.md") (qualified-name "Differences::Single"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (differencing (reference "A")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_type_single_differencing.md") (qualified-name "Differences::Pair"))) (kind differencing) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_single_differencing.md") (qualified-name "Differences::A")))))
    (reference (id (source (node (document "memory://snapshot/kerml_type_single_differencing.md") (qualified-name "Differences::Pair"))) (kind differencing) (ordinal 1))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_single_differencing.md") (qualified-name "Differences::B")))))
    (reference (id (source (node (document "memory://snapshot/kerml_type_single_differencing.md") (qualified-name "Differences::Single"))) (kind differencing) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_single_differencing.md") (qualified-name "Differences::A")))))
  )
  (relationships
    (relationship (kind differencing) (source (node (document "memory://snapshot/kerml_type_single_differencing.md") (qualified-name "Differences::Pair"))) (target (node (document "memory://snapshot/kerml_type_single_differencing.md") (qualified-name "Differences::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_type_single_differencing.md") (qualified-name "Differences::Pair"))) (kind differencing) (ordinal 0)))
    (relationship (kind differencing) (source (node (document "memory://snapshot/kerml_type_single_differencing.md") (qualified-name "Differences::Pair"))) (target (node (document "memory://snapshot/kerml_type_single_differencing.md") (qualified-name "Differences::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_type_single_differencing.md") (qualified-name "Differences::Pair"))) (kind differencing) (ordinal 1)))
    (relationship (kind differencing) (source (node (document "memory://snapshot/kerml_type_single_differencing.md") (qualified-name "Differences::Single"))) (target (node (document "memory://snapshot/kerml_type_single_differencing.md") (qualified-name "Differences::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_type_single_differencing.md") (qualified-name "Differences::Single"))) (kind differencing) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_type_single_differencing.md") (qualified-name "Differences::Pair")))
      (set-operand (operator difference) (ordinal 0) (node (document "memory://snapshot/kerml_type_single_differencing.md") (qualified-name "Differences::A")))
      (set-operand (operator difference) (ordinal 1) (node (document "memory://snapshot/kerml_type_single_differencing.md") (qualified-name "Differences::B")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_type_single_differencing.md") (qualified-name "Differences::Single")))
      (set-operand (operator difference) (ordinal 0) (node (document "memory://snapshot/kerml_type_single_differencing.md") (qualified-name "Differences::A")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_type_single_differencing.md") (range (start 5 32) (end 5 33)) (probe (position 5 32))
    (reference (id (source (node (document "memory://snapshot/kerml_type_single_differencing.md") (qualified-name "Differences::Pair"))) (kind differencing) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_single_differencing.md") (qualified-name "Differences::A")))))
    )
  )
  (query (document "memory://snapshot/kerml_type_single_differencing.md") (range (start 5 35) (end 5 36)) (probe (position 5 35))
    (reference (id (source (node (document "memory://snapshot/kerml_type_single_differencing.md") (qualified-name "Differences::Pair"))) (kind differencing) (ordinal 1) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_single_differencing.md") (qualified-name "Differences::B")))))
    )
  )
  (query (document "memory://snapshot/kerml_type_single_differencing.md") (range (start 8 34) (end 8 35)) (probe (position 8 34))
    (reference (id (source (node (document "memory://snapshot/kerml_type_single_differencing.md") (qualified-name "Differences::Single"))) (kind differencing) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_single_differencing.md") (qualified-name "Differences::A")))))
    )
  )
)
~~~
