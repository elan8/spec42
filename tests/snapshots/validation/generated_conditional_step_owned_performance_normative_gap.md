# META
~~~ini
description=The malformed pinned Step owned-performance body remains a normative-specification gap without a derived predicate contract
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.6.3:checkStepOwnedPerformanceSpecialization
blocked_by=normative-specification-gap-step-owned-performance-specialization-body
type=file
libraries=standard
~~~
# SOURCE
~~~kerml
package StepOwnedPerformanceNormativeGap {
    struct Holder {
        composite step work;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind specialization) (source "StepOwnedPerformanceNormativeGap::Holder::work") (target "Objects::Object::ownedPerformance") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_conditional_step_owned_performance_normative_gap.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:5e37dca175049335fd77517abca647d6c44cb7ed0dc544ddfbfd6299cd2dc200") (contract-version "parser-owned-resolution-v1") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_conditional_step_owned_performance_normative_gap.md") (qualified-name "StepOwnedPerformanceNormativeGap"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_step_owned_performance_normative_gap.md") (qualified-name "StepOwnedPerformanceNormativeGap::Holder"))) (kind kerml-structure) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_step_owned_performance_normative_gap.md") (qualified-name "StepOwnedPerformanceNormativeGap::Holder::work"))) (kind kerml-step) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
  )
  (references
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_step_owned_performance_normative_gap.md") (qualified-name "StepOwnedPerformanceNormativeGap::Holder"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_step_owned_performance_normative_gap.md") (qualified-name "StepOwnedPerformanceNormativeGap::Holder::work"))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_step_owned_performance_normative_gap.md") (qualified-name "StepOwnedPerformanceNormativeGap::Holder::work"))) (target (node (document "memory://snapshot/generated_conditional_step_owned_performance_normative_gap.md") (qualified-name "StepOwnedPerformanceNormativeGap::Holder"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/generated_conditional_step_owned_performance_normative_gap.md") (qualified-name "StepOwnedPerformanceNormativeGap::Holder")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_step_owned_performance_normative_gap.md") (qualified-name "StepOwnedPerformanceNormativeGap::Holder::work")))
      (featured-by (node (document "memory://snapshot/generated_conditional_step_owned_performance_normative_gap.md") (qualified-name "StepOwnedPerformanceNormativeGap::Holder")))
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
