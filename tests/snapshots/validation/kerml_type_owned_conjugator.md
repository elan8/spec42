# META
~~~ini
description=KerML Type ownedConjugator retains the canonical Conjugation relationship and its original Type endpoint
source_expectation=unsupported
rule_family=derive
expectation=semantics
rule_id=kerml-1.0:8.3.3.1.10:deriveTypeOwnedConjugator
blocked_by=lowering-gap-type-conjugation-relationship-identity
libraries=none
~~~
# SOURCE
~~~kerml
package Model {
  classifier Original;
  classifier Conjugated conjugates Original;
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (type-derived-fact
    (rule_id "kerml-1.0:8.3.3.1.10:deriveTypeOwnedConjugator")
    (source "Model::Conjugated")
    (target "Model::Original")
    (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_type_owned_conjugator.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 2 2) (end 2 44))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 2 2) (end 2 44))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:25de93c734923c384c41f5aafb4a24edfd6283590486bd925ed20652c397dbb8") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_conjugator.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_conjugator.md") (qualified-name "Model::Original"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
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
