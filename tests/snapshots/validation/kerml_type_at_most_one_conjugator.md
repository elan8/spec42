# META
~~~ini
description=KerML 8.3.3.1.10 validateTypeAtMostOneConjugator allows a Type at most one owned Conjugation
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.1.10 validateTypeAtMostOneConjugator
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.3.1.10:validateTypeAtMostOneConjugator
blocked_by=parser-gap-64-conjugation-declaration
type=file
~~~
# SOURCE
~~~kerml
package Conjugations {
    classifier A;
    classifier B;

    // Conforming: a single owned conjugation.
    classifier One conjugates A;

    // Invalid: two owned conjugations.
    classifier Two conjugates A conjugates B;
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_type_at_most_one_conjugator.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "type_multiple_conjugators")
        (source "semantic")
        (range (start 8 4) (end 8 45))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_type_at_most_one_conjugator.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 5 4) (end 5 32))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 5 4) (end 5 32))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 8 4) (end 8 45))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 8 4) (end 8 45))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:961dca0203ffbb38d247d964393f462836e1f48840958e9fee1e395798ea8a3f") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_type_at_most_one_conjugator.md") (qualified-name "Conjugations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_at_most_one_conjugator.md") (qualified-name "Conjugations::A"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_at_most_one_conjugator.md") (qualified-name "Conjugations::B"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
  )
  (relationships
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
)
~~~
