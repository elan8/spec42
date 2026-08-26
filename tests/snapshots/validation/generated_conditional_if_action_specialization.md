# META
~~~ini
description=Generated IfActionUsage specialization selects the exact no-else library anchor from SysML 8.3.17.10
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.17.10:checkIfActionUsageSpecialization
blocked_by=semantic-query-gap-anonymous-library-specialization-forms
type=file
libraries=standard
~~~
# SOURCE
~~~sysml
package IfActionSpecialization {
    action def Decision {
        action condition;
        if condition {
            action thenAction;
        }
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind specialization) (source "IfActionSpecialization::Decision::<anonymous>") (target "Actions::ifThenActions") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_conditional_if_action_specialization.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:ea4a56f6de9ff08ab86d94f59ff1d93741589278fcc0af1bab7d0cc2fd482d6a") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_conditional_if_action_specialization.md") (qualified-name "IfActionSpecialization"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_if_action_specialization.md") (qualified-name "IfActionSpecialization::Decision"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_if_action_specialization.md") (path (named (kind package) (name "IfActionSpecialization")) (named (kind action-def) (name "Decision")) (anonymous (kind if) (ordinal 0))))) (kind if) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "condition")))))
    (declaration (id (node (document "memory://snapshot/generated_conditional_if_action_specialization.md") (path (named (kind package) (name "IfActionSpecialization")) (named (kind action-def) (name "Decision")) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "thenAction"))))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_if_action_specialization.md") (qualified-name "IfActionSpecialization::Decision::condition"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/generated_conditional_if_action_specialization.md") (path (named (kind package) (name "IfActionSpecialization")) (named (kind action-def) (name "Decision")) (anonymous (kind if) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "condition")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_if_action_specialization.md") (qualified-name "IfActionSpecialization::Decision::condition")))))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/generated_conditional_if_action_specialization.md") (path (named (kind package) (name "IfActionSpecialization")) (named (kind action-def) (name "Decision")) (anonymous (kind if) (ordinal 0))))) (target (node (document "memory://snapshot/generated_conditional_if_action_specialization.md") (qualified-name "IfActionSpecialization::Decision::condition"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_if_action_specialization.md") (path (named (kind package) (name "IfActionSpecialization")) (named (kind action-def) (name "Decision")) (anonymous (kind if) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_if_action_specialization.md") (qualified-name "IfActionSpecialization::Decision"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_if_action_specialization.md") (path (named (kind package) (name "IfActionSpecialization")) (named (kind action-def) (name "Decision")) (anonymous (kind if) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::ifThenActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_if_action_specialization.md") (path (named (kind package) (name "IfActionSpecialization")) (named (kind action-def) (name "Decision")) (anonymous (kind if) (ordinal 0))))) (target (node (document "memory://snapshot/generated_conditional_if_action_specialization.md") (qualified-name "IfActionSpecialization::Decision"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_if_action_specialization.md") (path (named (kind package) (name "IfActionSpecialization")) (named (kind action-def) (name "Decision")) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "thenAction"))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_if_action_specialization.md") (path (named (kind package) (name "IfActionSpecialization")) (named (kind action-def) (name "Decision")) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "thenAction"))))) (target (node (document "memory://snapshot/generated_conditional_if_action_specialization.md") (path (named (kind package) (name "IfActionSpecialization")) (named (kind action-def) (name "Decision")) (anonymous (kind if) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_if_action_specialization.md") (qualified-name "IfActionSpecialization::Decision::condition"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_if_action_specialization.md") (qualified-name "IfActionSpecialization::Decision::condition"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_if_action_specialization.md") (qualified-name "IfActionSpecialization::Decision::condition"))) (target (node (document "memory://snapshot/generated_conditional_if_action_specialization.md") (qualified-name "IfActionSpecialization::Decision"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/generated_conditional_if_action_specialization.md") (path (named (kind package) (name "IfActionSpecialization")) (named (kind action-def) (name "Decision")) (anonymous (kind if) (ordinal 0))))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/generated_conditional_if_action_specialization.md") (qualified-name "IfActionSpecialization::Decision")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_if_action_specialization.md") (path (named (kind package) (name "IfActionSpecialization")) (named (kind action-def) (name "Decision")) (anonymous (kind if) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/generated_conditional_if_action_specialization.md") (qualified-name "IfActionSpecialization::Decision")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::IfThenAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::ifThenActions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::IfThenAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::ifThenActions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::IfPerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::IfThenPerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_if_action_specialization.md") (path (named (kind package) (name "IfActionSpecialization")) (named (kind action-def) (name "Decision")) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "thenAction")))))
      (featured-by (node (document "memory://snapshot/generated_conditional_if_action_specialization.md") (path (named (kind package) (name "IfActionSpecialization")) (named (kind action-def) (name "Decision")) (anonymous (kind if) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_if_action_specialization.md") (qualified-name "IfActionSpecialization::Decision::condition")))
      (featured-by (node (document "memory://snapshot/generated_conditional_if_action_specialization.md") (qualified-name "IfActionSpecialization::Decision")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
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
  (query (document "memory://snapshot/generated_conditional_if_action_specialization.md") (range (start 3 11) (end 3 20)) (probe (position 3 11))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_if_action_specialization.md") (path (named (kind package) (name "IfActionSpecialization")) (named (kind action-def) (name "Decision")) (anonymous (kind if) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "condition")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_if_action_specialization.md") (qualified-name "IfActionSpecialization::Decision::condition")))))
    )
  )
)
~~~
