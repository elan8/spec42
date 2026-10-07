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
        (code "type_multiple_multiplicities")
        (source "semantic")
        (range (start 7 8) (end 7 31))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:57e963da3df9ddc3b85962173b6ecc2be124762b0fdd6d60efb2314bdff544e6"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (qualified-name "Multiplicities"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (qualified-name "Multiplicities::One"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (facts (multiplicity (lower 1) (upper 1))))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "One")) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (kind kerml-multiplicity-range) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "One")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "One")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "One")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (qualified-name "Multiplicities::Two"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (facts (multiplicity (lower 1) (upper 1))))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Two")) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (kind kerml-multiplicity-range) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Two")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Two")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Two")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (qualified-name "Multiplicities::Two::extra"))) (kind kerml-multiplicity) (membership (kind owning) (visibility default)) (facts (multiplicity (lower 2) (upper 2))))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Two")) (named (kind kerml-multiplicity) (name "extra")) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (kind kerml-multiplicity-range) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Two")) (named (kind kerml-multiplicity) (name "extra")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Two")) (named (kind kerml-multiplicity) (name "extra")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Two")) (named (kind kerml-multiplicity) (name "extra")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "One")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "One")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Two")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Two")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Two")) (named (kind kerml-multiplicity) (name "extra")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Two")) (named (kind kerml-multiplicity) (name "extra")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "One")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "One")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Two")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Two")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Two")) (named (kind kerml-multiplicity) (name "extra")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Two")) (named (kind kerml-multiplicity) (name "extra")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
