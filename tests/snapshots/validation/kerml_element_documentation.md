# META
~~~ini
description=KerML Element documentation derivations project canonical typed documentation facts
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=kerml-1.0:8.3.2.1.2:deriveElementDocumentation
rule_id=kerml-1.0:8.3.2.1.2:deriveElementTextualRepresentation
libraries=none
~~~
# SOURCE
~~~kerml
package Model { action def Vehicle { doc /* vehicle documentation */ language "Alf" /* vehicle implementation */ } }
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (element-documentation
    (rule_id "kerml-1.0:8.3.2.1.2:deriveElementDocumentation")
    (source "Model::Vehicle")
    (form documentation)
    (locale none)
    (language none)
    (text " vehicle documentation ")
    (outcome resolved))
  (element-documentation
    (rule_id "kerml-1.0:8.3.2.1.2:deriveElementTextualRepresentation")
    (source "Model::Vehicle")
    (form textual_representation)
    (locale none)
    (language "Alf")
    (text " vehicle implementation ")
    (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_element_documentation.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:8308ef4dbd1f4b2efa1b9163f5d950fec9338e6ed8fc278361cf021ea52aca4c") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_element_documentation.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_element_documentation.md") (qualified-name "Model::Vehicle"))) (kind action-def) (membership (kind owning) (visibility default)) (documentation (doc (text " vehicle documentation ")) (rep (language "Alf") (text " vehicle implementation "))))
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
