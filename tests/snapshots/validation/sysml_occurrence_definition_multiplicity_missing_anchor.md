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
      (diagnostic
        (severity information)
        (code "missing_library_anchor")
        (source "semantic")
        (range (start 0 39) (end 0 40))
      )
      (diagnostic
        (severity information)
        (code "missing_library_anchor")
        (source "semantic")
        (range (start 0 39) (end 0 40))
      )
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
      (diagnostic
        (severity information)
        (code "missing_library_anchor")
        (source "semantic")
        (range (start 0 39) (end 0 40))
      )
      (diagnostic
        (severity information)
        (code "missing_library_anchor")
        (source "semantic")
        (range (start 0 39) (end 0 40))
      )
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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:827e8973a7f365858c47042319c2241122a9202e14da4ddf40b9bf45a219a6ba") (admitted (standard-library 1)))
  (declarations
    (declaration (id (node (document "memory://snapshot/fake-base.kerml") (qualified-name "Base"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/fake-base.kerml") (qualified-name "Base::zeroOrOne"))) (kind kerml-multiplicity) (membership (kind owning) (visibility default)) (facts (multiplicity (lower 0) (upper 1))))
    (declaration (id (node (document "memory://snapshot/fake-base.kerml") (path (named (kind package) (name "Base")) (named (kind kerml-multiplicity) (name "zeroOrOne")) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (kind kerml-multiplicity-range) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/fake-base.kerml") (path (named (kind package) (name "Base")) (named (kind kerml-multiplicity) (name "zeroOrOne")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/fake-base.kerml") (path (named (kind package) (name "Base")) (named (kind kerml-multiplicity) (name "zeroOrOne")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/fake-base.kerml") (path (named (kind package) (name "Base")) (named (kind kerml-multiplicity) (name "zeroOrOne")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/fake-base.kerml") (path (named (kind package) (name "Base")) (named (kind kerml-multiplicity) (name "zeroOrOne")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/fake-base.kerml") (path (named (kind package) (name "Base")) (named (kind kerml-multiplicity) (name "zeroOrOne")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/fake-base.kerml") (path (named (kind package) (name "Base")) (named (kind kerml-multiplicity) (name "zeroOrOne")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Individual"))) (kind occurrence-def) (membership (kind owning) (visibility default)) (facts (modifiers individual) (individual-multiplicity (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind occurrence-def) (name "Individual")) (anonymous (kind kerml-multiplicity) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind occurrence-def) (name "Individual")) (anonymous (kind kerml-multiplicity) (ordinal 0))))) (kind kerml-multiplicity) (membership (kind owning) (visibility default)) (facts (origin individual-multiplicity)))
  )
  (references
  )
  (relationships
    (relationship (kind subsetting) (source (node (document "memory://snapshot/fake-base.kerml") (qualified-name "Base::zeroOrOne"))) (target (node (document "memory://snapshot/occurrences.kerml") (qualified-name "Base::naturals"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/fake-base.kerml") (path (named (kind package) (name "Base")) (named (kind kerml-multiplicity) (name "zeroOrOne")) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (target (node (document "memory://snapshot/occurrences.kerml") (qualified-name "Base::naturals"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/fake-base.kerml") (path (named (kind package) (name "Base")) (named (kind kerml-multiplicity) (name "zeroOrOne")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/fake-base.kerml") (path (named (kind package) (name "Base")) (named (kind kerml-multiplicity) (name "zeroOrOne")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/fake-base.kerml") (path (named (kind package) (name "Base")) (named (kind kerml-multiplicity) (name "zeroOrOne")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/fake-base.kerml") (path (named (kind package) (name "Base")) (named (kind kerml-multiplicity) (name "zeroOrOne")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Individual"))) (target (node (document "memory://snapshot/occurrences.kerml") (qualified-name "Occurrences::Life"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Individual"))) (target (node (document "memory://snapshot/occurrences.kerml") (qualified-name "Occurrences::Occurrence"))) (provenance implied))
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
    (declaration (id (node (document "memory://snapshot/fake-base.kerml") (path (named (kind package) (name "Base")) (named (kind kerml-multiplicity) (name "zeroOrOne")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/occurrences.kerml") (qualified-name "Base::naturals")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/fake-base.kerml") (path (named (kind package) (name "Base")) (named (kind kerml-multiplicity) (name "zeroOrOne")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/fake-base.kerml") (path (named (kind package) (name "Base")) (named (kind kerml-multiplicity) (name "zeroOrOne")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/fake-base.kerml") (path (named (kind package) (name "Base")) (named (kind kerml-multiplicity) (name "zeroOrOne")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/fake-base.kerml") (path (named (kind package) (name "Base")) (named (kind kerml-multiplicity) (name "zeroOrOne")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)))))
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
