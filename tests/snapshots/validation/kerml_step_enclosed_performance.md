# META
~~~ini
description=KerML 8.3.4.6.3 checkStepEnclosedPerformanceSpecialization subsets a step owned by a behavior to Performances::Performance::enclosedPerformances
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.6.3 checkStepEnclosedPerformanceSpecialization
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.6.3:checkStepEnclosedPerformanceSpecialization
type=file
libraries=standard
~~~
# SOURCE
~~~kerml
package PerformancesProbe {
    behavior Performer {
        step enclosed;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind subsetting) (source "PerformancesProbe::Performer::enclosed") (target "Performances::Performance::enclosedPerformances") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_step_enclosed_performance.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:7789b0cb500296650b57375ec8148e631a4faf585e8be5e556ba78c8d9ff6483") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_step_enclosed_performance.md") (qualified-name "PerformancesProbe"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_step_enclosed_performance.md") (qualified-name "PerformancesProbe::Performer"))) (kind kerml-behavior) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_step_enclosed_performance.md") (qualified-name "PerformancesProbe::Performer::enclosed"))) (kind kerml-step) (membership (kind feature) (visibility default)))
  )
  (references
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_step_enclosed_performance.md") (qualified-name "PerformancesProbe::Performer"))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_step_enclosed_performance.md") (qualified-name "PerformancesProbe::Performer::enclosed"))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_step_enclosed_performance.md") (qualified-name "PerformancesProbe::Performer::enclosed"))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_step_enclosed_performance.md") (qualified-name "PerformancesProbe::Performer::enclosed"))) (target (node (document "memory://snapshot/kerml_step_enclosed_performance.md") (qualified-name "PerformancesProbe::Performer"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_step_enclosed_performance.md") (qualified-name "PerformancesProbe::Performer")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_step_enclosed_performance.md") (qualified-name "PerformancesProbe::Performer::enclosed")))
      (featured-by (node (document "memory://snapshot/kerml_step_enclosed_performance.md") (qualified-name "PerformancesProbe::Performer")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
