# META
~~~ini
description=Generated Step subperformance specialization preserves the pinned self.isComposite owner predicate
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.6.3:checkStepSubperformanceSpecialization
blocked_by=library-gap-step-subperformance-specialization-anchor
type=file
libraries=standard
~~~
# SOURCE
~~~kerml
package StepSubperformanceSpecialization {
    behavior Parent {
        composite step child;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind specialization) (source "StepSubperformanceSpecialization::Parent::child") (provenance implied) (outcome absent)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_conditional_step_subperformance_specialization.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_anchor")
        (source "semantic")
        (range (start 2 8) (end 2 29))
      )
      (diagnostic
        (severity information)
        (code "missing_library_anchor")
        (source "semantic")
        (range (start 2 8) (end 2 29))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:98d475e59d73e7802b723f24289aaf703b3e165ac8e38ea40b4dbb567ea09151") (contract-version "parser-owned-resolution-v2") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_conditional_step_subperformance_specialization.md") (qualified-name "StepSubperformanceSpecialization"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_step_subperformance_specialization.md") (qualified-name "StepSubperformanceSpecialization::Parent"))) (kind kerml-behavior) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_step_subperformance_specialization.md") (qualified-name "StepSubperformanceSpecialization::Parent::child"))) (kind kerml-step) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
  )
  (references
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_step_subperformance_specialization.md") (qualified-name "StepSubperformanceSpecialization::Parent"))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_step_subperformance_specialization.md") (qualified-name "StepSubperformanceSpecialization::Parent::child"))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_step_subperformance_specialization.md") (qualified-name "StepSubperformanceSpecialization::Parent::child"))) (target (node (document "memory://snapshot/generated_conditional_step_subperformance_specialization.md") (qualified-name "StepSubperformanceSpecialization::Parent"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/generated_conditional_step_subperformance_specialization.md") (qualified-name "StepSubperformanceSpecialization::Parent")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_step_subperformance_specialization.md") (qualified-name "StepSubperformanceSpecialization::Parent::child")))
      (featured-by (node (document "memory://snapshot/generated_conditional_step_subperformance_specialization.md") (qualified-name "StepSubperformanceSpecialization::Parent")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any subclassification))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
