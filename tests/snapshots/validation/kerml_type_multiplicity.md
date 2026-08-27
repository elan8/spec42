# META
~~~ini
description=KerML Type multiplicity exposes the authored scalar multiplicity through a canonical Multiplicity identity
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=kerml-1.0:8.3.3.1.10:deriveTypeMultiplicity
blocked_by=lowering-gap-type-multiplicity-identity
libraries=none
~~~
# SOURCE
~~~kerml
package Model {
  type Sized [1];
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (type-derived-fact
    (rule_id "kerml-1.0:8.3.3.1.10:deriveTypeMultiplicity")
    (source "Model::Sized")
    (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_type_multiplicity.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:0ef9261d84f31a9b8ed14864b9a1b0ce9e76651aeafe0f45e68aab311363ce9c"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_type_multiplicity.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_multiplicity.md") (qualified-name "Model::Sized"))) (kind kerml-type) (membership (kind owning) (visibility default)) (facts (multiplicity (lower 1) (upper 1))))
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
