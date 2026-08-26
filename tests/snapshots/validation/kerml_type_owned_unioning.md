# META
~~~ini
description=KerML deriveTypeOwnedUnioning projects authored unioning relationships
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=kerml-1.0:8.3.3.1.10:deriveTypeOwnedUnioning
libraries=none
~~~
# SOURCE
~~~kerml
package Model { classifier Base; classifier Derived unions Base; classifier Partial unions Missing; }
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (type-derived-relationship-collection
    (rule_id "kerml-1.0:8.3.3.1.10:deriveTypeOwnedUnioning")
    (source "Model::Derived")
    (kind unioning)
    (target "Model::Base")
    (provenance authored)
    (outcome resolved))
  (type-derived-relationship-collection
    (rule_id "kerml-1.0:8.3.3.1.10:deriveTypeOwnedUnioning")
    (source "Model::Partial")
    (kind unioning)
    (provenance authored)
    (outcome unresolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_type_owned_unioning.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "single_type_relationship_operand")
        (source "semantic")
        (range (start 0 59) (end 0 63))
      )
      (diagnostic
        (severity error)
        (code "single_type_relationship_operand")
        (source "semantic")
        (range (start 0 91) (end 0 98))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 0 91) (end 0 98))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:3ee56d4a4aa004f950fe48d24c75ee97f844766af4a6882cd045752aa1ef68ea") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_unioning.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_unioning.md") (qualified-name "Model::Base"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_unioning.md") (qualified-name "Model::Derived"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (unioning (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_unioning.md") (qualified-name "Model::Partial"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (unioning (reference "Missing")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_type_owned_unioning.md") (qualified-name "Model::Derived"))) (kind unioning) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_owned_unioning.md") (qualified-name "Model::Base")))))
    (reference (id (source (node (document "memory://snapshot/kerml_type_owned_unioning.md") (qualified-name "Model::Partial"))) (kind unioning) (ordinal 0))
      (authored-target "Missing")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind unioning) (source (node (document "memory://snapshot/kerml_type_owned_unioning.md") (qualified-name "Model::Derived"))) (target (node (document "memory://snapshot/kerml_type_owned_unioning.md") (qualified-name "Model::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_type_owned_unioning.md") (qualified-name "Model::Derived"))) (kind unioning) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_unioning.md") (qualified-name "Model::Derived")))
      (set-operand (operator union) (ordinal 0) (node (document "memory://snapshot/kerml_type_owned_unioning.md") (qualified-name "Model::Base")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_type_owned_unioning.md") (range (start 0 59) (end 0 63)) (probe (position 0 59))
    (reference (id (source (node (document "memory://snapshot/kerml_type_owned_unioning.md") (qualified-name "Model::Derived"))) (kind unioning) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_owned_unioning.md") (qualified-name "Model::Base")))))
    )
  )
  (query (document "memory://snapshot/kerml_type_owned_unioning.md") (range (start 0 91) (end 0 98)) (probe (position 0 91))
    (reference (id (source (node (document "memory://snapshot/kerml_type_owned_unioning.md") (qualified-name "Model::Partial"))) (kind unioning) (ordinal 0) (authored-target "Missing")
      (outcome (status unresolved)))
    )
  )
)
~~~
