# META
~~~ini
description=Generated Step subperformance specialization preserves the pinned self.isComposite owner predicate
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.6.3:checkStepSubperformanceSpecialization
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
  (relationship (kind subsetting) (source "StepSubperformanceSpecialization::Parent::child") (target "Performances::Performance::enclosedPerformances") (provenance implied) (outcome resolved))
  (relationship (kind subsetting) (source "StepSubperformanceSpecialization::Parent::child") (target "Performances::Performance::subperformances") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_conditional_step_subperformance_specialization.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:98d475e59d73e7802b723f24289aaf703b3e165ac8e38ea40b4dbb567ea09151") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_conditional_step_subperformance_specialization.md") (qualified-name "StepSubperformanceSpecialization"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_step_subperformance_specialization.md") (qualified-name "StepSubperformanceSpecialization::Parent"))) (kind kerml-behavior) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_step_subperformance_specialization.md") (qualified-name "StepSubperformanceSpecialization::Parent::child"))) (kind kerml-step) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
  )
  (references
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_step_subperformance_specialization.md") (qualified-name "StepSubperformanceSpecialization::Parent"))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_step_subperformance_specialization.md") (qualified-name "StepSubperformanceSpecialization::Parent::child"))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_step_subperformance_specialization.md") (qualified-name "StepSubperformanceSpecialization::Parent::child"))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_step_subperformance_specialization.md") (qualified-name "StepSubperformanceSpecialization::Parent::child"))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))) (provenance implied))
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
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
