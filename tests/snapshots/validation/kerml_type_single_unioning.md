# META
~~~ini
description=KerML 8.3.3.1.10 validateTypeOwnedUnioningNotOne forbids a Type from owning exactly one Unioning
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.1.10 validateTypeOwnedUnioningNotOne
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.3.1.10:validateTypeOwnedUnioningNotOne
type=file
~~~
# SOURCE
~~~kerml
package Unions {
    classifier A;
    classifier B;

    // Conforming: two unioning operands.
    classifier Pair unions A, B;

    // Invalid: exactly one unioning operand.
    classifier Single unions A;
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_type_single_unioning.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "single_type_relationship_operand")
        (source "semantic")
        (range (start 8 29) (end 8 30))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_type_single_unioning.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "single_type_relationship_operand")
        (source "semantic")
        (range (start 8 29) (end 8 30))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:e10e1401fab7be4d10dc1f22068b16dbe9f8c9c5bba36da02d7bbe5d10c10b7e") (contract-version "constructor-expression-specialization-v9"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_type_single_unioning.md") (qualified-name "Unions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_single_unioning.md") (qualified-name "Unions::A"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_single_unioning.md") (qualified-name "Unions::B"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_single_unioning.md") (qualified-name "Unions::Pair"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (unioning (reference "A")) (unioning (reference "B")))))
    (declaration (id (node (document "memory://snapshot/kerml_type_single_unioning.md") (qualified-name "Unions::Single"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (unioning (reference "A")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_type_single_unioning.md") (qualified-name "Unions::Pair"))) (kind unioning) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_single_unioning.md") (qualified-name "Unions::A")))))
    (reference (id (source (node (document "memory://snapshot/kerml_type_single_unioning.md") (qualified-name "Unions::Pair"))) (kind unioning) (ordinal 1))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_single_unioning.md") (qualified-name "Unions::B")))))
    (reference (id (source (node (document "memory://snapshot/kerml_type_single_unioning.md") (qualified-name "Unions::Single"))) (kind unioning) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_single_unioning.md") (qualified-name "Unions::A")))))
  )
  (relationships
    (relationship (kind unioning) (source (node (document "memory://snapshot/kerml_type_single_unioning.md") (qualified-name "Unions::Pair"))) (target (node (document "memory://snapshot/kerml_type_single_unioning.md") (qualified-name "Unions::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_type_single_unioning.md") (qualified-name "Unions::Pair"))) (kind unioning) (ordinal 0)))
    (relationship (kind unioning) (source (node (document "memory://snapshot/kerml_type_single_unioning.md") (qualified-name "Unions::Pair"))) (target (node (document "memory://snapshot/kerml_type_single_unioning.md") (qualified-name "Unions::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_type_single_unioning.md") (qualified-name "Unions::Pair"))) (kind unioning) (ordinal 1)))
    (relationship (kind unioning) (source (node (document "memory://snapshot/kerml_type_single_unioning.md") (qualified-name "Unions::Single"))) (target (node (document "memory://snapshot/kerml_type_single_unioning.md") (qualified-name "Unions::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_type_single_unioning.md") (qualified-name "Unions::Single"))) (kind unioning) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_type_single_unioning.md") (qualified-name "Unions::Pair")))
      (set-operand (operator union) (ordinal 0) (node (document "memory://snapshot/kerml_type_single_unioning.md") (qualified-name "Unions::A")))
      (set-operand (operator union) (ordinal 1) (node (document "memory://snapshot/kerml_type_single_unioning.md") (qualified-name "Unions::B")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_type_single_unioning.md") (qualified-name "Unions::Single")))
      (set-operand (operator union) (ordinal 0) (node (document "memory://snapshot/kerml_type_single_unioning.md") (qualified-name "Unions::A")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_type_single_unioning.md") (range (start 5 27) (end 5 28)) (probe (position 5 27))
    (reference (id (source (node (document "memory://snapshot/kerml_type_single_unioning.md") (qualified-name "Unions::Pair"))) (kind unioning) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_single_unioning.md") (qualified-name "Unions::A")))))
    )
  )
  (query (document "memory://snapshot/kerml_type_single_unioning.md") (range (start 5 30) (end 5 31)) (probe (position 5 30))
    (reference (id (source (node (document "memory://snapshot/kerml_type_single_unioning.md") (qualified-name "Unions::Pair"))) (kind unioning) (ordinal 1) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_single_unioning.md") (qualified-name "Unions::B")))))
    )
  )
  (query (document "memory://snapshot/kerml_type_single_unioning.md") (range (start 8 29) (end 8 30)) (probe (position 8 29))
    (reference (id (source (node (document "memory://snapshot/kerml_type_single_unioning.md") (qualified-name "Unions::Single"))) (kind unioning) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_single_unioning.md") (qualified-name "Unions::A")))))
    )
  )
)
~~~
