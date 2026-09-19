# META
~~~ini
description=Individual multiplicity does not select among competing standard library zeroOrOne anchors
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.9.3:checkOccurrenceDefinitionMultiplicitySpecialization
coverage_role=secondary
type=file
libraries=none
standard_library_document=base-a.kerml
standard_library_document=base-b.kerml
standard_library_document=occurrences.kerml
~~~
# SOURCE
## base-a.kerml
~~~kerml
standard library package Base { multiplicity zeroOrOne [0..1]; }
~~~
## base-b.kerml
~~~kerml
standard library package Base { multiplicity zeroOrOne [0..1]; }
~~~
## occurrences.kerml
~~~kerml
standard library package Occurrences { class Occurrence; class Life; }
standard library package Base { feature naturals; }
~~~
## model.sysml
~~~sysml
package Model {
    individual occurrence def Individual;
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
        (range (start 1 4) (end 1 41))
        (related-information
          (related
            (uri "memory://snapshot/base-a.kerml")
            (range (start 0 32) (end 0 62))
          )
          (related
            (uri "memory://snapshot/base-b.kerml")
            (range (start 0 32) (end 0 62))
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
  (specialization-check (rule_id "sysml-2.0:8.3.9.3:checkOccurrenceDefinitionMultiplicitySpecialization") (outcome unresolved)))
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
        (range (start 1 4) (end 1 41))
        (related-information
          (related
            (uri "memory://snapshot/base-a.kerml")
            (range (start 0 32) (end 0 62))
          )
          (related
            (uri "memory://snapshot/base-b.kerml")
            (range (start 0 32) (end 0 62))
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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:affe676b43f70d34c7f24ff4c48c8fdd36e20d4d9cdfd3baa8e5b0be68d5a1e2") (admitted (standard-library 3)))
  (declarations
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Individual"))) (kind occurrence-def) (membership (kind owning) (visibility default)) (facts (modifiers individual) (individual-multiplicity (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind occurrence-def) (name "Individual")) (anonymous (kind kerml-multiplicity) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind occurrence-def) (name "Individual")) (anonymous (kind kerml-multiplicity) (ordinal 0))))) (kind kerml-multiplicity) (membership (kind owning) (visibility default)) (facts (origin individual-multiplicity)))
  )
  (references
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Individual"))) (target (node (document "memory://snapshot/occurrences.kerml") (qualified-name "Occurrences::Life"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind occurrence-def) (name "Individual")) (anonymous (kind kerml-multiplicity) (ordinal 0))))) (target (node (document "memory://snapshot/occurrences.kerml") (qualified-name "Base::naturals"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Individual")))
      (supertype (node (document "memory://snapshot/occurrences.kerml") (qualified-name "Occurrences::Life")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/occurrences.kerml") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind occurrence-def) (name "Individual")) (anonymous (kind kerml-multiplicity) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/occurrences.kerml") (qualified-name "Base::naturals")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
