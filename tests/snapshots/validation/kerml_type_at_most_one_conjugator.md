# META
~~~ini
description=KerML 8.3.3.1.10 validateTypeAtMostOneConjugator allows a Type at most one owned Conjugation
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.1.10 validateTypeAtMostOneConjugator
source_expectation=accepted
rule_family=validate
expectation=by_construction
rule_id=kerml-1.0:8.3.3.1.10:validateTypeAtMostOneConjugator
blocked_by=abstract-syntax-nonrepresentable-second-conjugation
type=file
~~~
# SOURCE
~~~kerml
package Conjugations {
    classifier A;

    // Conforming: a single owned conjugation. `ConjugationPart = ( 'conjugates' | '~' )
    // OwnedConjugation` (KerML BNF 462) admits one clause per type declaration, so the
    // violating second owned Conjugation has no concrete-syntax spelling.
    classifier One conjugates A;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_type_at_most_one_conjugator.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:cad8d5d9e688108014f1146152415f348c54c29e02f4bed8c578ead6a0c2bcd8") (contract-version "constructor-expression-result-v8"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_type_at_most_one_conjugator.md") (qualified-name "Conjugations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_at_most_one_conjugator.md") (qualified-name "Conjugations::A"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_at_most_one_conjugator.md") (qualified-name "Conjugations::One"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (conjugation (reference "A")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_type_at_most_one_conjugator.md") (qualified-name "Conjugations::One"))) (kind conjugation) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_at_most_one_conjugator.md") (qualified-name "Conjugations::A")))))
  )
  (relationships
    (relationship (kind conjugation) (source (node (document "memory://snapshot/kerml_type_at_most_one_conjugator.md") (qualified-name "Conjugations::One"))) (target (node (document "memory://snapshot/kerml_type_at_most_one_conjugator.md") (qualified-name "Conjugations::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_type_at_most_one_conjugator.md") (qualified-name "Conjugations::One"))) (kind conjugation) (ordinal 0)))
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
  (query (document "memory://snapshot/kerml_type_at_most_one_conjugator.md") (range (start 6 30) (end 6 31)) (probe (position 6 30))
    (reference (id (source (node (document "memory://snapshot/kerml_type_at_most_one_conjugator.md") (qualified-name "Conjugations::One"))) (kind conjugation) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_at_most_one_conjugator.md") (qualified-name "Conjugations::A")))))
    )
  )
)
~~~
