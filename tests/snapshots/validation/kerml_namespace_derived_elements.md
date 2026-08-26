# META
~~~ini
description=KerML Namespace owned member and import derivations project canonical structural facts
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=kerml-1.0:8.3.2.4.5:deriveNamespaceOwnedMember
rule_id=kerml-1.0:8.3.2.4.5:deriveNamespaceOwnedImport
libraries=none
~~~
# SOURCE
~~~kerml
package Model { part def Owned; }
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (namespace-derived-element
    (rule_id "kerml-1.0:8.3.2.4.5:deriveNamespaceOwnedMember")
    (source "Model")
    (target "Model::Owned")
    (outcome resolved))
  (namespace-derived-element
    (rule_id "kerml-1.0:8.3.2.4.5:deriveNamespaceOwnedImport")
    (source "Model")
    (outcome absent)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_namespace_derived_elements.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:d85f157a6608e4d66fdc6d059354330aafd4b769dd50ce7aa8360546b40a1712"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_namespace_derived_elements.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_namespace_derived_elements.md") (qualified-name "Model::Owned"))) (kind part-def) (membership (kind owning) (visibility default)))
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
