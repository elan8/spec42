# META
~~~ini
description=KerML 8.3.3.1.10 validateTypeDifferencingTypesNotSelf forbids a Type from being one of its own differencingTypes
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.1.10 validateTypeDifferencingTypesNotSelf
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.3.1.10:validateTypeDifferencingTypesNotSelf
blocked_by=semantic-type-relationship-operand-is-self
type=file
~~~
# SOURCE
~~~kerml
package Differences {
    classifier A;
    classifier B;

    // Conforming: neither differencing operand is the differenced type itself.
    classifier Good differences A, B;

    // Invalid: the type is one of its own differencing operands.
    classifier Bad differences A, Bad;
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_type_differencing_self.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "type_relationship_operand_is_self")
        (source "semantic")
        (range (start 8 34) (end 8 37))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_type_differencing_self.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:f7a18ebd8d80182345b3892edb2cf051022d0515e8462674b3d8db6b9240bf6a") (contract-version "constructor-expression-result-v8"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_type_differencing_self.md") (qualified-name "Differences"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_differencing_self.md") (qualified-name "Differences::A"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_differencing_self.md") (qualified-name "Differences::B"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_differencing_self.md") (qualified-name "Differences::Bad"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (differencing (reference "A")) (differencing (reference "Bad")))))
    (declaration (id (node (document "memory://snapshot/kerml_type_differencing_self.md") (qualified-name "Differences::Good"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (differencing (reference "A")) (differencing (reference "B")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_type_differencing_self.md") (qualified-name "Differences::Bad"))) (kind differencing) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_differencing_self.md") (qualified-name "Differences::A")))))
    (reference (id (source (node (document "memory://snapshot/kerml_type_differencing_self.md") (qualified-name "Differences::Bad"))) (kind differencing) (ordinal 1))
      (authored-target "Bad")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_differencing_self.md") (qualified-name "Differences::Bad")))))
    (reference (id (source (node (document "memory://snapshot/kerml_type_differencing_self.md") (qualified-name "Differences::Good"))) (kind differencing) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_differencing_self.md") (qualified-name "Differences::A")))))
    (reference (id (source (node (document "memory://snapshot/kerml_type_differencing_self.md") (qualified-name "Differences::Good"))) (kind differencing) (ordinal 1))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_differencing_self.md") (qualified-name "Differences::B")))))
  )
  (relationships
    (relationship (kind differencing) (source (node (document "memory://snapshot/kerml_type_differencing_self.md") (qualified-name "Differences::Bad"))) (target (node (document "memory://snapshot/kerml_type_differencing_self.md") (qualified-name "Differences::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_type_differencing_self.md") (qualified-name "Differences::Bad"))) (kind differencing) (ordinal 0)))
    (relationship (kind differencing) (source (node (document "memory://snapshot/kerml_type_differencing_self.md") (qualified-name "Differences::Bad"))) (target (node (document "memory://snapshot/kerml_type_differencing_self.md") (qualified-name "Differences::Bad"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_type_differencing_self.md") (qualified-name "Differences::Bad"))) (kind differencing) (ordinal 1)))
    (relationship (kind differencing) (source (node (document "memory://snapshot/kerml_type_differencing_self.md") (qualified-name "Differences::Good"))) (target (node (document "memory://snapshot/kerml_type_differencing_self.md") (qualified-name "Differences::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_type_differencing_self.md") (qualified-name "Differences::Good"))) (kind differencing) (ordinal 0)))
    (relationship (kind differencing) (source (node (document "memory://snapshot/kerml_type_differencing_self.md") (qualified-name "Differences::Good"))) (target (node (document "memory://snapshot/kerml_type_differencing_self.md") (qualified-name "Differences::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_type_differencing_self.md") (qualified-name "Differences::Good"))) (kind differencing) (ordinal 1)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_type_differencing_self.md") (qualified-name "Differences::Bad")))
      (set-operand (operator difference) (ordinal 0) (node (document "memory://snapshot/kerml_type_differencing_self.md") (qualified-name "Differences::A")))
      (set-operand (operator difference) (ordinal 1) (node (document "memory://snapshot/kerml_type_differencing_self.md") (qualified-name "Differences::Bad")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_type_differencing_self.md") (qualified-name "Differences::Good")))
      (set-operand (operator difference) (ordinal 0) (node (document "memory://snapshot/kerml_type_differencing_self.md") (qualified-name "Differences::A")))
      (set-operand (operator difference) (ordinal 1) (node (document "memory://snapshot/kerml_type_differencing_self.md") (qualified-name "Differences::B")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_type_differencing_self.md") (range (start 8 31) (end 8 32)) (probe (position 8 31))
    (reference (id (source (node (document "memory://snapshot/kerml_type_differencing_self.md") (qualified-name "Differences::Bad"))) (kind differencing) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_differencing_self.md") (qualified-name "Differences::A")))))
    )
  )
  (query (document "memory://snapshot/kerml_type_differencing_self.md") (range (start 8 34) (end 8 37)) (probe (position 8 34))
    (reference (id (source (node (document "memory://snapshot/kerml_type_differencing_self.md") (qualified-name "Differences::Bad"))) (kind differencing) (ordinal 1) (authored-target "Bad")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_differencing_self.md") (qualified-name "Differences::Bad")))))
    )
  )
  (query (document "memory://snapshot/kerml_type_differencing_self.md") (range (start 5 32) (end 5 33)) (probe (position 5 32))
    (reference (id (source (node (document "memory://snapshot/kerml_type_differencing_self.md") (qualified-name "Differences::Good"))) (kind differencing) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_differencing_self.md") (qualified-name "Differences::A")))))
    )
  )
  (query (document "memory://snapshot/kerml_type_differencing_self.md") (range (start 5 35) (end 5 36)) (probe (position 5 35))
    (reference (id (source (node (document "memory://snapshot/kerml_type_differencing_self.md") (qualified-name "Differences::Good"))) (kind differencing) (ordinal 1) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_differencing_self.md") (qualified-name "Differences::B")))))
    )
  )
)
~~~
