# META
~~~ini
description=SysML checkPartDefinitionSpecialization reports competing canonical Parts::Part anchors without choosing one
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.11.2:checkPartDefinitionSpecialization
coverage_role=secondary
type=file
libraries=none
standard_library_document=parts-a.sysml
standard_library_document=parts-b.sysml
~~~
# SOURCE
## parts-a.sysml
~~~sysml
standard library package Parts {
    part def Part;
}
~~~
## parts-b.sysml
~~~sysml
standard library package Parts {
    part def Part;
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
        (severity warning)
        (code "ambiguous_library_anchor")
        (source "semantic")
        (range (start 1 4) (end 1 23))
        (related-information
          (related
            (uri "memory://snapshot/parts-a.sysml")
            (range (start 1 4) (end 1 18))
          )
          (related
            (uri "memory://snapshot/parts-b.sysml")
            (range (start 1 4) (end 1 18))
          )
        )
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
        (severity warning)
        (code "ambiguous_library_anchor")
        (source "semantic")
        (range (start 1 4) (end 1 23))
        (related-information
          (related
            (uri "memory://snapshot/parts-a.sysml")
            (range (start 1 4) (end 1 18))
          )
          (related
            (uri "memory://snapshot/parts-b.sysml")
            (range (start 1 4) (end 1 18))
          )
        )
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:babc4ae45bfbb93a68d6ddff8df84d9659ca9b2e8f46f20002e2493a9df0a281") (contract-version "constructor-expression-specialization-v9") (admitted (standard-library 2)))
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
