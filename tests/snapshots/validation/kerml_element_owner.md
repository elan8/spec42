# META
~~~ini
description=KerML deriveElementOwner projects the canonical declaration owner
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=kerml-1.0:8.3.2.1.2:deriveElementOwner
libraries=none
~~~
# SOURCE
~~~kerml
package Model { part def Vehicle { attribute mass; } }
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (element-owner
    (rule_id "kerml-1.0:8.3.2.1.2:deriveElementOwner")
    (source "Model::Vehicle::mass")
    (owner "Model::Vehicle")
    (outcome resolved))
  (element-owner
    (rule_id "kerml-1.0:8.3.2.1.2:deriveElementOwner")
    (source "Model")
    (outcome absent)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_element_owner.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:41bf27828561975f0a540d094ba86916d00b093f1be8cc9483447f2b71dd3e25") (contract-version "parser-owned-resolution-v2"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_element_owner.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_element_owner.md") (qualified-name "Model::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_element_owner.md") (qualified-name "Model::Vehicle::mass"))) (kind attribute) (membership (kind feature) (visibility default)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_element_owner.md") (qualified-name "Model::Vehicle::mass"))) (target (node (document "memory://snapshot/kerml_element_owner.md") (qualified-name "Model::Vehicle"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_element_owner.md") (qualified-name "Model::Vehicle::mass")))
      (featured-by (node (document "memory://snapshot/kerml_element_owner.md") (qualified-name "Model::Vehicle")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
