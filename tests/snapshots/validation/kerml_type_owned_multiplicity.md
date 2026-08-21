# META
~~~ini
description=KerML 8.3.3.1.10 validateTypeOwnedMultiplicity allows a Type at most one ownedMember that is a Multiplicity
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.1.10 validateTypeOwnedMultiplicity
type=file
skip_validation=the pinned parser has no production for a second multiplicity clause, so `classifier Two[1][2];` never reaches semantics and is reported as unsupported_grammar_form
~~~
# SOURCE
~~~kerml
package Multiplicities {
    // Conforming: a single owned multiplicity.
    classifier One[1];

    // Invalid: two owned multiplicities.
    classifier Two[1][2];
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_type_owned_multiplicity.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "type_multiple_owned_multiplicities")
        (source "semantic")
        (range (start 5 4) (end 5 25))
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
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 5 4) (end 5 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 5 4) (end 5 25))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:fc8012492846d9738fb23d8830b095a71bc07ab4c7ee3e678ea22015a1091e2e") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (qualified-name "Multiplicities"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_multiplicity.md") (qualified-name "Multiplicities::One"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (facts (multiplicity (lower 1) (upper 1))))
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
