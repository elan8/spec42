# META
~~~ini
description=KerML 8.3.3.1.8 validateSpecificationSpecificNotConjugated forbids the specific Type of a Specialization from being a conjugated Type
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.1.8 validateSpecificationSpecificNotConjugated
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.3.1.8:validateSpecificationSpecificNotConjugated
blocked_by=lowering-gap-kerml-relationship-declarations
type=file
~~~
# SOURCE
~~~kerml
package Conjugations {
    classifier A;
    classifier B;

    // Conforming: a non-conjugated type specializes another type.
    classifier Plain specializes B;

    // Invalid: a conjugated type is the specific type of a specialization. KerML's
    // `TypeDeclaration` makes `SpecializationPart` and `ConjugationPart` exclusive alternatives,
    // so the violating specialization is authored as a standalone declaration.
    classifier Conjugated conjugates A;
    subclassifier Conjugated specializes B;
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_specialization_specific_not_conjugated.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "specialization_specific_conjugated")
        (source "semantic")
        (range (start 8 4) (end 8 53))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_specialization_specific_not_conjugated.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 11 4) (end 11 43))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:7b0c4057f268a63a60d7af7db2bc6e03f25d431378938a20b763142c3e6ebafa") (contract-version "parser-owned-resolution-v2"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_specialization_specific_not_conjugated.md") (qualified-name "Conjugations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_specialization_specific_not_conjugated.md") (qualified-name "Conjugations::A"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_specialization_specific_not_conjugated.md") (qualified-name "Conjugations::B"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_specialization_specific_not_conjugated.md") (qualified-name "Conjugations::Conjugated"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (conjugation (reference "A")))))
    (declaration (id (node (document "memory://snapshot/kerml_specialization_specific_not_conjugated.md") (qualified-name "Conjugations::Plain"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "B")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_specialization_specific_not_conjugated.md") (qualified-name "Conjugations::Conjugated"))) (kind conjugation) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_specialization_specific_not_conjugated.md") (qualified-name "Conjugations::A")))))
    (reference (id (source (node (document "memory://snapshot/kerml_specialization_specific_not_conjugated.md") (qualified-name "Conjugations::Plain"))) (kind specialization) (ordinal 0))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_specialization_specific_not_conjugated.md") (qualified-name "Conjugations::B")))))
  )
  (relationships
    (relationship (kind conjugation) (source (node (document "memory://snapshot/kerml_specialization_specific_not_conjugated.md") (qualified-name "Conjugations::Conjugated"))) (target (node (document "memory://snapshot/kerml_specialization_specific_not_conjugated.md") (qualified-name "Conjugations::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_specialization_specific_not_conjugated.md") (qualified-name "Conjugations::Conjugated"))) (kind conjugation) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_specialization_specific_not_conjugated.md") (qualified-name "Conjugations::Plain"))) (target (node (document "memory://snapshot/kerml_specialization_specific_not_conjugated.md") (qualified-name "Conjugations::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_specialization_specific_not_conjugated.md") (qualified-name "Conjugations::Plain"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_specialization_specific_not_conjugated.md") (qualified-name "Conjugations::B")))
      (subtype (node (document "memory://snapshot/kerml_specialization_specific_not_conjugated.md") (qualified-name "Conjugations::Plain")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_specialization_specific_not_conjugated.md") (qualified-name "Conjugations::Plain")))
      (supertype (node (document "memory://snapshot/kerml_specialization_specific_not_conjugated.md") (qualified-name "Conjugations::B")) (scopes any subclassification))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_specialization_specific_not_conjugated.md") (range (start 10 37) (end 10 38)) (probe (position 10 37))
    (reference (id (source (node (document "memory://snapshot/kerml_specialization_specific_not_conjugated.md") (qualified-name "Conjugations::Conjugated"))) (kind conjugation) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_specialization_specific_not_conjugated.md") (qualified-name "Conjugations::A")))))
    )
  )
  (query (document "memory://snapshot/kerml_specialization_specific_not_conjugated.md") (range (start 5 33) (end 5 34)) (probe (position 5 33))
    (reference (id (source (node (document "memory://snapshot/kerml_specialization_specific_not_conjugated.md") (qualified-name "Conjugations::Plain"))) (kind specialization) (ordinal 0) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_specialization_specific_not_conjugated.md") (qualified-name "Conjugations::B")))))
    )
  )
)
~~~
