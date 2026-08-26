# META
~~~ini
description=SysML checkPartDefinitionSpecialization reports a missing canonical Parts::Part anchor instead of guessing a workspace substitute
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.11.2:checkPartDefinitionSpecialization
coverage_role=secondary
type=file
libraries=none
standard_library_document=library.sysml
~~~
# SOURCE
## library.sysml
~~~sysml
standard library package Other {
    part def Something;
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
        (severity information)
        (code "missing_library_anchor")
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
    (provenance implied)
    (outcome absent)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/model.sysml"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_anchor")
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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:75a115f514a1bd49f8ac6576656e919c443ec1f28197ddc0fef146c61594279d") (contract-version "constructor-expression-specialization-v9") (admitted (standard-library 1)))
  (declarations
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
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
