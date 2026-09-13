# META
~~~ini
description=Individual multiplicity remains unresolved when its standard-library anchor is missing; a workspace namesake cannot substitute
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
type=file
libraries=none
standard_library_document=occurrences.kerml
rule_family=check
expectation=semantics
coverage_role=secondary
rule_id=sysml-2.0:8.3.9.3:checkOccurrenceDefinitionMultiplicitySpecialization
~~~
# SOURCE
## occurrences.kerml
~~~kerml
standard library package Occurrences { class Occurrence; class Life; }
standard library package Base { feature naturals; }
~~~
## fake-base.kerml
~~~kerml
package Base { multiplicity zeroOrOne [0..1]; }
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
  (document "memory://snapshot/fake-base.kerml"
    (diagnostics
    )
  )
  (document "memory://snapshot/model.sysml"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_anchor")
        (source "semantic")
        (range (start 1 4) (end 1 41))
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
  (document "memory://snapshot/fake-base.kerml"
    (diagnostics
    )
  )
  (document "memory://snapshot/model.sysml"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_anchor")
        (source "semantic")
        (range (start 1 4) (end 1 41))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:6254fe42fa2ac140aab0cf04db82df22099a3457692277b56f8050535bb75b5b") (admitted (standard-library 1)))
  (declarations
    (declaration (id (node (document "memory://snapshot/fake-base.kerml") (qualified-name "Base"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/fake-base.kerml") (qualified-name "Base::zeroOrOne"))) (kind kerml-multiplicity) (membership (kind owning) (visibility default)) (facts (multiplicity (lower 0) (upper 1))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Individual"))) (kind occurrence-def) (membership (kind owning) (visibility default)) (facts (modifiers individual) (individual-multiplicity (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind occurrence-def) (name "Individual")) (anonymous (kind kerml-multiplicity) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind occurrence-def) (name "Individual")) (anonymous (kind kerml-multiplicity) (ordinal 0))))) (kind kerml-multiplicity) (membership (kind owning) (visibility default)) (facts (origin individual-multiplicity)))
  )
  (references
  )
  (relationships
    (relationship (kind subsetting) (source (node (document "memory://snapshot/fake-base.kerml") (qualified-name "Base::zeroOrOne"))) (target (node (document "memory://snapshot/occurrences.kerml") (qualified-name "Base::naturals"))) (provenance implied))
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
    (declaration (id (node (document "memory://snapshot/fake-base.kerml") (qualified-name "Base::zeroOrOne")))
      (supertype (node (document "memory://snapshot/occurrences.kerml") (qualified-name "Base::naturals")) (scopes any feature))
    )
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
