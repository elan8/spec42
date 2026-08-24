# META
~~~ini
description=Validation consumes the implied checkPartDefinitionSpecialization fact and detects a specialization cycle it completes
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.11.2:checkPartDefinitionSpecialization
coverage_role=secondary
type=file
libraries=none
standard_library_document=parts.sysml
~~~
# SOURCE
## parts.sysml
~~~sysml
standard library package Parts {
    part def Part specializes Model::Component;
}
~~~
## model.sysml
~~~sysml
package Model {
    part def Component;
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/model.sysml"
    (diagnostics
      (diagnostic
        (severity error)
        (code "specialization_cycle")
        (source "semantic")
        (range (start 1 4) (end 1 23))
      )
    )
  )
)
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship
    (kind specialization)
    (source "Model::Component")
    (target "Parts::Part")
    (provenance implied)
    (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/model.sysml"
    (diagnostics
      (diagnostic
        (severity error)
        (code "specialization_cycle")
        (source "semantic")
        (range (start 1 4) (end 1 23))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:fd62707aa40a255dfa44d1d3c8227413c95715db74b5d328eec47c1d1c6ce09c") (contract-version "parser-owned-resolution-v2") (admitted (standard-library 1)))
  (declarations
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
  )
  (references
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Component"))) (target (node (document "memory://snapshot/parts.sysml") (qualified-name "Parts::Part"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Component"))) (cyclic true)
      (supertype (node (document "memory://snapshot/parts.sysml") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/parts.sysml") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
