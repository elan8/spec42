# META
~~~ini
description=KerML 8.3.3.1.10 validateTypeOwnedMultiplicity allows a Type at most one ownedMember that is a Multiplicity
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.1.10 validateTypeOwnedMultiplicity
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.3.1.10:validateTypeOwnedMultiplicity
blocked_by=parser-gap-kerml-multiplicity-member
type=file
~~~
# SOURCE
~~~kerml
package Multiplicities {
    // Conforming: a single owned multiplicity.
    classifier One[1];

    // Invalid: a TypeBody NonFeatureMember may be a Multiplicity, giving the type a second
    // ownedMember that is a Multiplicity.
    classifier Two[1] {
        multiplicity extra [2];
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_type_owned_multiplicity.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "type_multiple_multiplicities")
        (source "semantic")
        (range (start 7 8) (end 7 31))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_type_owned_multiplicity.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 7 8) (end 7 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 7 21) (end 7 26))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:8eb8588969cf05e56a15787cdf698326a6cc06a49074ffe4d01c6335fa635f5c"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (qualified-name "Multiplicities"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (qualified-name "Multiplicities::One"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (facts (multiplicity (lower 1) (upper 1))))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "One")) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (kind kerml-multiplicity-range) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "One")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "One")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "One")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (qualified-name "Multiplicities::Two"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "multiplicity")) (expressionOperand (reference "extra")))))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Two")) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (kind kerml-multiplicity-range) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Two")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Two")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Two")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (qualified-name "Multiplicities::Two"))) (kind expressionOperand) (ordinal 0))
      (authored-target "multiplicity")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (qualified-name "Multiplicities::Two"))) (kind expressionOperand) (ordinal 1))
      (authored-target "extra")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "One")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "One")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Two")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Two")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (qualified-name "Multiplicities::Two"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (qualified-name "Multiplicities::Two"))) (state unsupported))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "One")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "One")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Two")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Two")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (qualified-name "Multiplicities::Two"))) (outcome unsupported))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_type_owned_multiplicity.md") (range (start 7 8) (end 7 20)) (probe (position 7 8))
    (reference (id (source (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (qualified-name "Multiplicities::Two"))) (kind expressionOperand) (ordinal 0) (authored-target "multiplicity")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/kerml_type_owned_multiplicity.md") (range (start 7 21) (end 7 26)) (probe (position 7 21))
    (reference (id (source (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (qualified-name "Multiplicities::Two"))) (kind expressionOperand) (ordinal 1) (authored-target "extra")
      (outcome (status unresolved)))
    )
  )
)
~~~
