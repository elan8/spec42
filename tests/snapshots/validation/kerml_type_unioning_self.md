# META
~~~ini
description=KerML 8.3.3.1.10 validateTypeUnioningTypesNotSelf forbids a Type from being one of its own unioningTypes
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.1.10 validateTypeUnioningTypesNotSelf
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.3.1.10:validateTypeUnioningTypesNotSelf
blocked_by=semantic-type-relationship-operand-is-self
type=file
~~~
# SOURCE
~~~kerml
package Unions {
    classifier A;
    classifier B;

    // Conforming: neither unioning operand is the unioned type itself.
    classifier Good unions A, B;

    // Invalid: the unioned type is one of its own unioning operands.
    classifier Bad unions A, Bad;
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_type_unioning_self.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "type_relationship_operand_is_self")
        (source "semantic")
        (range (start 8 29) (end 8 32))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_type_unioning_self.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:63bea654229513d713456c8a004d73443b530e544dc70bc5f0413707e5f6d606") (contract-version "operator-expression-arguments-v7"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_type_unioning_self.md") (qualified-name "Unions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_unioning_self.md") (qualified-name "Unions::A"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_unioning_self.md") (qualified-name "Unions::B"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_unioning_self.md") (qualified-name "Unions::Bad"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (unioning (reference "A")) (unioning (reference "Bad")))))
    (declaration (id (node (document "memory://snapshot/kerml_type_unioning_self.md") (qualified-name "Unions::Good"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (unioning (reference "A")) (unioning (reference "B")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_type_unioning_self.md") (qualified-name "Unions::Bad"))) (kind unioning) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_unioning_self.md") (qualified-name "Unions::A")))))
    (reference (id (source (node (document "memory://snapshot/kerml_type_unioning_self.md") (qualified-name "Unions::Bad"))) (kind unioning) (ordinal 1))
      (authored-target "Bad")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_unioning_self.md") (qualified-name "Unions::Bad")))))
    (reference (id (source (node (document "memory://snapshot/kerml_type_unioning_self.md") (qualified-name "Unions::Good"))) (kind unioning) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_unioning_self.md") (qualified-name "Unions::A")))))
    (reference (id (source (node (document "memory://snapshot/kerml_type_unioning_self.md") (qualified-name "Unions::Good"))) (kind unioning) (ordinal 1))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_unioning_self.md") (qualified-name "Unions::B")))))
  )
  (relationships
    (relationship (kind unioning) (source (node (document "memory://snapshot/kerml_type_unioning_self.md") (qualified-name "Unions::Bad"))) (target (node (document "memory://snapshot/kerml_type_unioning_self.md") (qualified-name "Unions::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_type_unioning_self.md") (qualified-name "Unions::Bad"))) (kind unioning) (ordinal 0)))
    (relationship (kind unioning) (source (node (document "memory://snapshot/kerml_type_unioning_self.md") (qualified-name "Unions::Bad"))) (target (node (document "memory://snapshot/kerml_type_unioning_self.md") (qualified-name "Unions::Bad"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_type_unioning_self.md") (qualified-name "Unions::Bad"))) (kind unioning) (ordinal 1)))
    (relationship (kind unioning) (source (node (document "memory://snapshot/kerml_type_unioning_self.md") (qualified-name "Unions::Good"))) (target (node (document "memory://snapshot/kerml_type_unioning_self.md") (qualified-name "Unions::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_type_unioning_self.md") (qualified-name "Unions::Good"))) (kind unioning) (ordinal 0)))
    (relationship (kind unioning) (source (node (document "memory://snapshot/kerml_type_unioning_self.md") (qualified-name "Unions::Good"))) (target (node (document "memory://snapshot/kerml_type_unioning_self.md") (qualified-name "Unions::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_type_unioning_self.md") (qualified-name "Unions::Good"))) (kind unioning) (ordinal 1)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_type_unioning_self.md") (qualified-name "Unions::Bad")))
      (set-operand (operator union) (ordinal 0) (node (document "memory://snapshot/kerml_type_unioning_self.md") (qualified-name "Unions::A")))
      (set-operand (operator union) (ordinal 1) (node (document "memory://snapshot/kerml_type_unioning_self.md") (qualified-name "Unions::Bad")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_type_unioning_self.md") (qualified-name "Unions::Good")))
      (set-operand (operator union) (ordinal 0) (node (document "memory://snapshot/kerml_type_unioning_self.md") (qualified-name "Unions::A")))
      (set-operand (operator union) (ordinal 1) (node (document "memory://snapshot/kerml_type_unioning_self.md") (qualified-name "Unions::B")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_type_unioning_self.md") (range (start 8 26) (end 8 27)) (probe (position 8 26))
    (reference (id (source (node (document "memory://snapshot/kerml_type_unioning_self.md") (qualified-name "Unions::Bad"))) (kind unioning) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_unioning_self.md") (qualified-name "Unions::A")))))
    )
  )
  (query (document "memory://snapshot/kerml_type_unioning_self.md") (range (start 8 29) (end 8 32)) (probe (position 8 29))
    (reference (id (source (node (document "memory://snapshot/kerml_type_unioning_self.md") (qualified-name "Unions::Bad"))) (kind unioning) (ordinal 1) (authored-target "Bad")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_unioning_self.md") (qualified-name "Unions::Bad")))))
    )
  )
  (query (document "memory://snapshot/kerml_type_unioning_self.md") (range (start 5 27) (end 5 28)) (probe (position 5 27))
    (reference (id (source (node (document "memory://snapshot/kerml_type_unioning_self.md") (qualified-name "Unions::Good"))) (kind unioning) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_unioning_self.md") (qualified-name "Unions::A")))))
    )
  )
  (query (document "memory://snapshot/kerml_type_unioning_self.md") (range (start 5 30) (end 5 31)) (probe (position 5 30))
    (reference (id (source (node (document "memory://snapshot/kerml_type_unioning_self.md") (qualified-name "Unions::Good"))) (kind unioning) (ordinal 1) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_unioning_self.md") (qualified-name "Unions::B")))))
    )
  )
)
~~~
